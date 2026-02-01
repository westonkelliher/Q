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
//! mod.rs          - Public API: generate_world, initialize_world
//! ├── noise.rs    - Noise utilities, seed offsets, constants
//! ├── biome.rs    - Biome determination and tile-to-biome mapping
//! ├── substrate.rs - Substrate rules per biome
//! └── objects.rs  - Object spawning rules per biome
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
mod substrate;
mod objects;

use ::noise::Perlin;
use crate::types::{Land, Tile, World};

// Re-export public items
pub use biome::{LandBiomes, calculate_land_biomes, determine_biome, get_tile_biome};

/// Seed offset for the substrate Perlin noise generator.
/// Uses a prime to ensure substrate patterns differ from biome patterns.
const SUBSTRATE_SEED_OFFSET: u64 = 999983;

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
    substrate_perlin: &Perlin,
) -> [[Tile; 8]; 8] {
    // First pass: Generate all tiles with substrate and initial objects
    let mut tiles = std::array::from_fn(|tile_y| {
        std::array::from_fn(|tile_x| {
            let biome = get_tile_biome(biomes, tile_x, tile_y);
            
            // Global tile coordinates for substrate (cross-boundary continuity)
            let global_x = land_x * 8 + tile_x as i32;
            let global_y = land_y * 8 + tile_y as i32;
            
            // Generate substrate
            let substrate_noise = substrate::get_substrate_noise(
                biome, global_x, global_y, substrate_perlin, seed
            );
            let substrate = substrate::substrate_for_biome(biome, substrate_noise);
            
            // Generate objects (pseudo-random sparse placement)
            let objects = objects::objects_for_biome(
                biome, seed, land_x, land_y, tile_x, tile_y
            );
            
            Tile { substrate, objects }
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
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biomes = calculate_land_biomes(x, y, &biome_perlin, seed);
            let tiles = generate_land_terrain(x, y, &biomes, seed, &substrate_perlin);
            
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
