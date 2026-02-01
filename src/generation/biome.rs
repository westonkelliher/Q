//! Biome determination and tile-to-biome mapping.
//!
//! Each Land has 9 biomes in a 3x3 pattern. This module handles:
//! - Determining which biome exists at a given coordinate
//! - Mapping individual tiles within a land to their biome

use ::noise::Perlin;
use crate::types::Biome;
use super::noise::{seed_offset, sample_noise, BIOME_SCALE};

/// Holds the 9 biomes for a land, arranged in a 3x3 pattern.
///
/// ```text
///   top_left    |    top     |   top_right
///   ------------|------------|-------------
///   left        |   center   |   right
///   ------------|------------|-------------
///   bottom_left |   bottom   |   bottom_right
/// ```
///
/// Biome sub-coordinate formula for land (lx, ly):
///   X coords: (2*lx - 1), (2*lx), (2*lx + 1)  => left, center, right
///   Y coords: (2*ly - 1), (2*ly), (2*ly + 1)  => top, center, bottom
pub struct LandBiomes {
    pub center: Biome,
    pub top: Biome,
    pub bottom: Biome,
    pub left: Biome,
    pub right: Biome,
    pub top_left: Biome,
    pub top_right: Biome,
    pub bottom_left: Biome,
    pub bottom_right: Biome,
}

/// Noise thresholds for biome determination.
/// Perlin noise returns values roughly in [-1.0, 1.0].
const LAKE_THRESHOLD: f64 = -0.3;
const MEADOW_THRESHOLD: f64 = 0.0;
const FOREST_THRESHOLD: f64 = 0.4;
// Above FOREST_THRESHOLD = Mountain

/// Determines which biome exists at a given biome-coordinate.
///
/// Note: These are biome coordinates, not land coordinates.
/// Use `calculate_land_biomes` to get the 9 biomes for a land.
pub fn determine_biome(x: i32, y: i32, perlin: &Perlin, seed: u64) -> Biome {
    let offset = seed_offset(seed, 0);
    let noise_value = sample_noise(perlin, x as f64, y as f64, BIOME_SCALE, offset);
    
    if noise_value < LAKE_THRESHOLD {
        Biome::Lake
    } else if noise_value < MEADOW_THRESHOLD {
        Biome::Meadow
    } else if noise_value < FOREST_THRESHOLD {
        Biome::Forest
    } else {
        Biome::Mountain
    }
}

/// Calculates all 9 biomes for a land using biome sub-coordinates.
///
/// # Formula
/// For land at (land_x, land_y):
/// - biome X coords: (2*land_x - 1), (2*land_x), (2*land_x + 1)
/// - biome Y coords: (2*land_y - 1), (2*land_y), (2*land_y + 1)
///
/// # Example: Land (0, 0)
/// ```text
/// X coords: -1, 0, 1
/// Y coords: -1, 0, 1
/// top_left=(-1,-1), top=(0,-1), top_right=(1,-1)
/// left=(-1,0), center=(0,0), right=(1,0)
/// bottom_left=(-1,1), bottom=(0,1), bottom_right=(1,1)
/// ```
///
/// # Example: Land (-4, -5)
/// ```text
/// X coords: -9, -8, -7
/// Y coords: -11, -10, -9
/// center biome coord = (-8, -10)
/// ```
pub fn calculate_land_biomes(land_x: i32, land_y: i32, perlin: &Perlin, seed: u64) -> LandBiomes {
    let x_left   = 2 * land_x - 1;
    let x_center = 2 * land_x;
    let x_right  = 2 * land_x + 1;
    
    let y_top    = 2 * land_y - 1;
    let y_center = 2 * land_y;
    let y_bottom = 2 * land_y + 1;
    
    LandBiomes {
        top_left:     determine_biome(x_left,   y_top,    perlin, seed),
        top:          determine_biome(x_center, y_top,    perlin, seed),
        top_right:    determine_biome(x_right,  y_top,    perlin, seed),
        left:         determine_biome(x_left,   y_center, perlin, seed),
        center:       determine_biome(x_center, y_center, perlin, seed),
        right:        determine_biome(x_right,  y_center, perlin, seed),
        bottom_left:  determine_biome(x_left,   y_bottom, perlin, seed),
        bottom:       determine_biome(x_center, y_bottom, perlin, seed),
        bottom_right: determine_biome(x_right,  y_bottom, perlin, seed),
    }
}

/// Gets the biome for a specific tile within a land.
///
/// # Tile Mapping (8x8 grid)
/// ```text
/// - 4 corners (1 tile each): (0,0), (7,0), (0,7), (7,7)
/// - 4 edges (6 tiles each):
///     top:    row 0, cols 1-6
///     bottom: row 7, cols 1-6
///     left:   col 0, rows 1-6
///     right:  col 7, rows 1-6
/// - center (36 tiles): rows 1-6, cols 1-6
/// ```
pub fn get_tile_biome(biomes: &LandBiomes, tile_x: usize, tile_y: usize) -> &Biome {
    let is_top    = tile_y == 0;
    let is_bottom = tile_y == 7;
    let is_left   = tile_x == 0;
    let is_right  = tile_x == 7;
    
    match (is_left, is_right, is_top, is_bottom) {
        // Corners
        (true,  false, true,  false) => &biomes.top_left,
        (false, true,  true,  false) => &biomes.top_right,
        (true,  false, false, true)  => &biomes.bottom_left,
        (false, true,  false, true)  => &biomes.bottom_right,
        // Edges
        (_,     _,     true,  false) => &biomes.top,
        (_,     _,     false, true)  => &biomes.bottom,
        (true,  false, _,     _)     => &biomes.left,
        (false, true,  _,     _)     => &biomes.right,
        // Center
        _ => &biomes.center,
    }
}
