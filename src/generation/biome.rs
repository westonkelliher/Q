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

/// Base discriminator for biome Perlin noise generators.
/// Uses a prime number to ensure unique seed offsets for each biome.
const BIOME_PERLIN_DISCRIMINATOR_BASE: u64 = 10007; // Prime number

/// Discriminator multipliers for each biome's Perlin noise.
/// Each biome gets a unique discriminator to ensure independent noise patterns.
const LAKE_DISCRIMINATOR: u64 = 1 * BIOME_PERLIN_DISCRIMINATOR_BASE;
const MEADOW_DISCRIMINATOR: u64 = 2 * BIOME_PERLIN_DISCRIMINATOR_BASE;
const PLAINS_DISCRIMINATOR: u64 = 3 * BIOME_PERLIN_DISCRIMINATOR_BASE;
const FOREST_DISCRIMINATOR: u64 = 4 * BIOME_PERLIN_DISCRIMINATOR_BASE;
const MOUNTAIN_DISCRIMINATOR: u64 = 5 * BIOME_PERLIN_DISCRIMINATOR_BASE;

/// Discriminator for height Perlin noise generator.
const HEIGHT_DISCRIMINATOR: u64 = 6 * BIOME_PERLIN_DISCRIMINATOR_BASE;

/// Base bias values for each biome.
/// These represent the inherent likelihood of each biome appearing.
/// Mountain and Lake biases are reduced since they also get height-based adjustments.
const LAKE_BIAS: f64 = 0.05;      // Reduced from base since low height boosts lakes
const MEADOW_BIAS: f64 = 0.0;     // Neutral bias
const PLAINS_BIAS: f64 = 0.0;     // Neutral bias
const FOREST_BIAS: f64 = 0.1;     // Slight preference for forests
const MOUNTAIN_BIAS: f64 = 0.05;  // Reduced from base since high height boosts mountains

/// Strength of height influence on biome selection.
/// Higher values mean height has more impact on biome determination.
const HEIGHT_INFLUENCE: f64 = 0.3;

/// Determines which biome exists at a given biome-coordinate.
///
/// Uses separate Perlin noise functions for each biome type, plus a height
/// Perlin function. Samples all biome Perlin functions and height at the
/// location, then combines them with biome-specific biases and height adjustments.
/// Higher heights boost mountain likelihood, lower heights boost lake likelihood.
/// Returns the biome with the highest final value.
///
/// Note: These are biome coordinates, not land coordinates.
/// Use `calculate_land_biomes` to get the 9 biomes for a land.
pub fn determine_biome(x: i32, y: i32, seed: u64) -> Biome {
    // Create Perlin instances for each biome with unique seed offsets
    let lake_perlin = Perlin::new((seed.wrapping_add(LAKE_DISCRIMINATOR)) as u32);
    let meadow_perlin = Perlin::new((seed.wrapping_add(MEADOW_DISCRIMINATOR)) as u32);
    let plains_perlin = Perlin::new((seed.wrapping_add(PLAINS_DISCRIMINATOR)) as u32);
    let forest_perlin = Perlin::new((seed.wrapping_add(FOREST_DISCRIMINATOR)) as u32);
    let mountain_perlin = Perlin::new((seed.wrapping_add(MOUNTAIN_DISCRIMINATOR)) as u32);
    let height_perlin = Perlin::new((seed.wrapping_add(HEIGHT_DISCRIMINATOR)) as u32);
    
    // Sample all biome Perlin functions and height at this location
    let offset = seed_offset(seed, 0);
    let lake_value = sample_noise(&lake_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    let meadow_value = sample_noise(&meadow_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    let plains_value = sample_noise(&plains_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    let forest_value = sample_noise(&forest_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    let mountain_value = sample_noise(&mountain_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    let height = sample_noise(&height_perlin, x as f64, y as f64, BIOME_SCALE, offset);
    
    // Calculate final values: base noise + bias + height adjustment
    // Higher heights boost mountains, lower heights boost lakes
    let lake_final = lake_value + LAKE_BIAS + (-height * HEIGHT_INFLUENCE);
    let meadow_final = meadow_value + MEADOW_BIAS;
    let plains_final = plains_value + PLAINS_BIAS;
    let forest_final = forest_value + FOREST_BIAS;
    let mountain_final = mountain_value + MOUNTAIN_BIAS + (height * HEIGHT_INFLUENCE);
    
    // Find the biome with the highest final value
    // In case of ties, prefer biomes in enum order (Lake < Meadow < Plains < Forest < Mountain)
    let mut max_value = lake_final;
    let mut selected_biome = Biome::Lake;
    
    if meadow_final > max_value {
        max_value = meadow_final;
        selected_biome = Biome::Meadow;
    }
    if plains_final > max_value {
        max_value = plains_final;
        selected_biome = Biome::Plains;
    }
    if forest_final > max_value {
        max_value = forest_final;
        selected_biome = Biome::Forest;
    }
    if mountain_final > max_value {
        selected_biome = Biome::Mountain;
    }
    
    selected_biome
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
pub fn calculate_land_biomes(land_x: i32, land_y: i32, seed: u64) -> LandBiomes {
    let x_left   = 2 * land_x - 1;
    let x_center = 2 * land_x;
    let x_right  = 2 * land_x + 1;
    
    let y_top    = 2 * land_y - 1;
    let y_center = 2 * land_y;
    let y_bottom = 2 * land_y + 1;
    
    LandBiomes {
        top_left:     determine_biome(x_left,   y_top,    seed),
        top:          determine_biome(x_center, y_top,    seed),
        top_right:    determine_biome(x_right,  y_top,    seed),
        left:         determine_biome(x_left,   y_center, seed),
        center:       determine_biome(x_center, y_center, seed),
        right:        determine_biome(x_right,  y_center, seed),
        bottom_left:  determine_biome(x_left,   y_bottom, seed),
        bottom:       determine_biome(x_center, y_bottom, seed),
        bottom_right: determine_biome(x_right,  y_bottom, seed),
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
