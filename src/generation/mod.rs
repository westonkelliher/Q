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
const BIOME_DISCRIMINATOR_BASE: u64 = 7919; // Prime number

/// Scale factor for brush placement in forests.
/// Lower values = larger brush patches.
/// Set lower than SUBSTRATE_SCALE to create larger, more cohesive brush areas.
const BRUSH_SCALE: f64 = 0.15; // Larger features than substrate scale (0.4)

/// Discriminator for brush-specific noise (separate from substrate noise).
const BRUSH_DISCRIMINATOR: u64 = 7 * BIOME_DISCRIMINATOR_BASE;

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
/// Uses a two-stage approach: first determines dirt vs grass/brush using normal scale,
/// then uses a larger-scale noise to determine brush patches within grass/brush areas.
fn generate_forest_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    // Stage 1: Determine dirt vs grass/brush using normal substrate scale
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let substrate_offset = seed_offset(seed, 2 * BIOME_DISCRIMINATOR_BASE); // Forest discriminator = 2 * BASE
    let substrate_noise = sample_noise(&substrate_perlin, global_x as f64, global_y as f64, SUBSTRATE_SCALE, substrate_offset);
    
    // First determine if it's dirt or grass/brush area
    let substrate = if substrate_noise < -0.4 {
        Substrate::Dirt
    } else {
        // Stage 2: For grass/brush areas, use larger-scale noise to determine brush patches
        // This creates larger, more cohesive brush areas without affecting dirt/grass distribution
        let brush_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
        let brush_offset = seed_offset(seed, BRUSH_DISCRIMINATOR);
        let brush_noise = sample_noise(&brush_perlin, global_x as f64, global_y as f64, BRUSH_SCALE, brush_offset);
        
        // Use the brush noise to determine if this grass/brush area becomes brush
        // Threshold of 0.2 maintains similar brush frequency but with larger patches
        if brush_noise > 0.2 {
            Substrate::Brush
        } else {
            Substrate::Grass
        }
    };
    
    let objects = objects::generate_forest_objects(&substrate, seed, global_x, global_y);
    Tile { substrate, objects }
}

/// Generates a tile for Plains biome.
/// Plains are mostly dirt with some grass patches.
fn generate_plains_tile(global_x: i32, global_y: i32, seed: u64) -> Tile {
    let perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let offset = seed_offset(seed, 4 * BIOME_DISCRIMINATOR_BASE); // Plains discriminator = 4 * BASE
    let noise = sample_noise(&perlin, global_x as f64, global_y as f64, SUBSTRATE_SCALE, offset);
    
    // Mostly dirt (below 0.45), with some grass patches (above 0.45)
    let substrate = if noise < 0.45 {
        Substrate::Dirt
    } else {
        Substrate::Grass
    };
    
    let objects = objects::generate_plains_objects(&substrate, seed, global_x, global_y);
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
                Biome::Plains => generate_plains_tile(global_x, global_y, seed),
                Biome::Forest => generate_forest_tile(global_x, global_y, seed),
                Biome::Mountain => generate_mountain_tile(global_x, global_y, seed),
            }
        })
    });
    
    // Second pass: Add sticks deterministically near trees
    objects::add_sticks_near_trees(&mut tiles, seed, land_x, land_y);
    
    tiles
}

/// Generates a single land at the specified coordinates.
/// Skips generation if the land already exists to preserve any dynamic changes.
pub fn generate_land(world: &mut World, seed: u64, land_x: i32, land_y: i32) {
    // Skip if land already exists to preserve dynamic changes
    if world.terrain.contains_key(&(land_x, land_y)) {
        return;
    }
    
    let biomes = calculate_land_biomes(land_x, land_y, seed);
    let tiles = generate_land_terrain(land_x, land_y, &biomes, seed);
    
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
    
    world.terrain.insert((land_x, land_y), land);
}

/// Generates world terrain for a rectangular region of lands.
///
/// Coordinates are inclusive: generates from (x1, y1) to (x2, y2).
/// Skips lands that already exist to preserve any dynamic changes.
pub fn generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            generate_land(world, seed, x, y);
        }
    }
}

/// Initializes a world with the default generation area (-10 to 10).
pub fn initialize_world(world: &mut World, seed: u64) {
    generate_world(world, seed, -10, -10, 10, 10);
}

/// Ensures terrain is generated within the specified radius of the center position.
/// Only generates lands that don't already exist, preserving any dynamic changes.
/// Uses circular radius (Euclidean distance) instead of square radius.
pub fn ensure_terrain_generated(world: &mut World, center_x: i32, center_y: i32, radius: f32) {
    let radius_squared = radius * radius;
    let radius_ceil = radius.ceil() as i32;
    
    // Generate each ungenerated land within circular radius
    for dx in -radius_ceil..=radius_ceil {
        for dy in -radius_ceil..=radius_ceil {
            // Check if this coordinate is within the circular radius
            let distance_squared = (dx * dx + dy * dy) as f32;
            if distance_squared <= radius_squared {
                let x = center_x + dx;
                let y = center_y + dy;
                generate_land(world, world.seed, x, y);
            }
        }
    }
}
