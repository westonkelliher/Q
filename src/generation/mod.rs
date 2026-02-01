//! Terrain generation for the world.
//!
//! This module handles procedural generation of lands, including:
//! - Biome determination (Lake, Meadow, Forest, Mountain)
//! - Substrate generation (Water, Grass, Dirt, Stone, Brush)
//! - Object placement (Trees, Rocks, Sticks)
//!
//! # Architecture
//!
//! ```text
//! mod.rs       - Public API: generate_world, tile generation per biome
//! ├── noise.rs - Noise utilities, seed offsets, constants
//! ├── biome.rs - Biome determination and tile-to-biome mapping
//! └── objects.rs - Object spawning rules per biome
//! ```
//!
//! # Coordinate Systems
//!
//! - **Land coordinates**: (land_x, land_y) - identifies a land in the world
//! - **Tile coordinates**: (tile_x, tile_y) - 0-7 within a single land
//! - **Global tile coordinates**: land * 8 + tile - used for cross-boundary continuity
//! - **Biome coordinates**: 2 * land ± 1 - used for the 9-biome system

mod noise;
mod biome;
mod objects;

use ::noise::Perlin;
use crate::types::{Biome, Land, Substrate, Tile, World};
use noise::{seed_offset, sample_noise, SUBSTRATE_SCALE};

// Re-export public items
pub use biome::{LandBiomes, calculate_land_biomes, determine_biome, get_tile_biome};

/// Seed offset for the substrate Perlin noise generator.
/// Uses a prime to ensure substrate patterns differ from biome patterns.
const SUBSTRATE_SEED_OFFSET: u64 = 999983;

/// Discriminator values for per-biome noise offsets.
/// Each biome gets a unique offset so their substrate patterns differ.
const 
BIOME_DISCRIMINATOR_BASE: u64 = 7919; // Prime number

/// Generates a tile for Lake biome.
fn generate_lake_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    let substrate = Substrate::Water;
    let objects = objects::generate_lake_objects(seed, global_x, global_y);
    Tile { substrate, objects }
}

/// Generates a tile for Meadow biome.
fn generate_meadow_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    let perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let offset = seed_offset(seed, BIOME_DISCRIMINATOR_BASE); // Meadow discriminator = 1 * BASE
    let noise = sample_noise(&perlin, global_x as f64, global_y as f64, SUBSTRATE_SCALE, offset);
    
    let substrate = if noise < -0.8 { 
        Substrate::Dirt 
    } else { 
        Substrate::Grass 
    };
    
    let objects = objects::generate_meadow_objects(&substrate, seed, global_x, global_y);
    Tile { substrate, objects }
}

/// Generates a tile for Forest biome.
fn generate_forest_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    let perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let offset = seed_offset(seed, 2 * BIOME_DISCRIMINATOR_BASE); // Forest discriminator = 2 * BASE
    let noise = sample_noise(&perlin, global_x as f64, global_y as f64, SUBSTRATE_SCALE, offset);
    
    let substrate = if noise < -0.4 {
        Substrate::Dirt
    } else if noise < 0.2 {
        Substrate::Grass
    } else {
        Substrate::Brush
    };
    
    let objects = objects::generate_forest_objects(&substrate, seed, global_x, global_y);
    Tile { substrate, objects }
}

/// Generates a tile for Mountain biome.
fn generate_mountain_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    let perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let offset = seed_offset(seed, 3 * BIOME_DISCRIMINATOR_BASE); // Mountain discriminator = 3 * BASE
    let noise = sample_noise(&perlin, global_x as f64, global_y as f64, SUBSTRATE_SCALE, offset);
    
    let substrate = if noise < 0.6 {
        Substrate::Stone
    } else {
        Substrate::Dirt
    };
    
    let objects = objects::generate_mountain_objects(&substrate, seed, global_x, global_y);
    Tile { substrate, objects }
}

/// Generates terrain tiles for a land based on its 9 biomes.
///
/// Uses the biome at each tile position to determine substrate and objects.
/// Substrate noise is globally continuous; object noise is land-local.
/// After initial object placement, adds sticks deterministically near trees.
pub fn generate_land_terrain(
    land_x: i32,
    land_y: i32,
    biomes: &LandBiomes,
    seed: u64,
) -> [[Tile; 8]; 8] {
    // First pass: Generate all tiles with substrate and initial objects
    let mut tiles = std::array::from_fn(|tile_y| {
        std::array::from_fn(|tile_x| {
            let biome = get_tile_biome(biomes, tile_x, tile_y);
            let global_x = land_x * 8 + tile_x as i32;
            let global_y = land_y * 8 + tile_y as i32;
            
            match biome {
                Biome::Lake => generate_lake_tile(global_x, global_y, seed),
                Biome::Meadow => generate_meadow_tile(global_x, global_y, seed),
                Biome::Forest => generate_forest_tile(global_x, global_y, seed),
                Biome::Mountain => generate_mountain_tile(global_x, global_y, seed),
            }
        })
    });
    
    // Second pass: Add sticks deterministically near trees
    objects::add_sticks_near_trees(&mut tiles, seed, land_x, land_y);
    
    tiles
}

/// Generates world terrain for a rectangular region of lands.
///
/// Coordinates are inclusive: generates from (x1, y1) to (x2, y2).
pub fn generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32) {
    let biome_perlin = Perlin::new(seed as u32);
    
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biomes = calculate_land_biomes(x, y, &biome_perlin, seed);
            let tiles = generate_land_terrain(x, y, &biomes, seed);
            
            let land = Land {
                tiles,
                center: biomes.center,
                top: biomes.top,
                bottom: biomes.bottom,
                left: biomes.left,
                right: biomes.right,
                top_left: biomes.top_left,
                top_right: biomes.top_right,
                bottom_left: biomes.bottom_left,
                bottom_right: biomes.bottom_right,
            };
            
            world.terrain.insert((x, y), land);
        }
    }
}

/// Initializes a world with the default generation area (-10 to 10).
pub fn initialize_world(world: &mut World, seed: u64) {
    generate_world(world, seed, -10, -10, 10, 10);
}
