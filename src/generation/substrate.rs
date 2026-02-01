//! Substrate generation based on biome type.
//!
//! Each biome has its own substrate rules that determine what ground
//! type appears at each tile based on noise values.

use ::noise::Perlin;
use crate::types::{Biome, Substrate};
use super::noise::{seed_offset, sample_noise, SUBSTRATE_SCALE};

/// Seed offset for the substrate Perlin noise generator.
/// Uses a prime to ensure substrate patterns differ from biome patterns.
const SUBSTRATE_SEED_OFFSET: u64 = 999983;

/// Discriminator values for per-biome noise offsets.
/// Each biome gets a unique offset so their substrate patterns differ.
const BIOME_DISCRIMINATOR_BASE: u64 = 7919; // Prime number

fn biome_discriminator(biome: &Biome) -> u64 {
    let biome_id: u64 = match biome {
        Biome::Lake => 0,
        Biome::Meadow => 1,
        Biome::Forest => 2,
        Biome::Mountain => 3,
    };
    biome_id.wrapping_mul(BIOME_DISCRIMINATOR_BASE)
}

/// Gets substrate noise for a tile, using biome-specific offsets.
///
/// The noise is globally continuous (uses global tile coordinates),
/// ensuring adjacent lands blend seamlessly.
pub fn get_substrate_noise(
    biome: &Biome,
    global_tile_x: i32,
    global_tile_y: i32,
    perlin: &Perlin,
    seed: u64,
) -> f64 {
    let offset = seed_offset(seed, biome_discriminator(biome));
    sample_noise(perlin, global_tile_x as f64, global_tile_y as f64, SUBSTRATE_SCALE, offset)
}

/// Determines the substrate for a tile based on its biome and noise value.
///
/// # Substrate Rules by Biome
///
/// | Biome    | Substrates              | Noise Thresholds           |
/// |----------|-------------------------|----------------------------|
/// | Lake     | Water only              | (always)                   |
/// | Meadow   | Dirt, Grass             | < -0.8 → Dirt (rare), else Grass  |
/// | Forest   | Dirt, Grass, Brush      | < -0.4 → Dirt, < 0.2 → Grass, else Brush |
/// | Mountain | Stone, Dirt             | < 0.6 → Stone, else Dirt   |
pub fn substrate_for_biome(biome: &Biome, noise: f64) -> Substrate {
    match biome {
        // Generates substrate for Lake biome.
        Biome::Lake => Substrate::Water,
        
        // Generates substrate for Meadow biome.
        Biome::Meadow => {
            if noise < -0.8 { Substrate::Dirt }  // Very rare dirt patches
            else { Substrate::Grass }
        }
        
        // Generates substrate for Forest biome.
        Biome::Forest => {
            if noise < -0.4 { Substrate::Dirt }
            else if noise < 0.2 { Substrate::Grass }
            else { Substrate::Brush }
        }
        
        // Generates substrate for Mountain biome.
        Biome::Mountain => {
            if noise < 0.6 { Substrate::Stone }
            else { Substrate::Dirt }
        }
    }
}

/// Generates substrate for Lake biome at global coordinates.
pub fn generate_lake_substrate(global_x: i32, global_y: i32, seed: u64) -> Substrate {
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let biome = Biome::Lake;
    let noise = get_substrate_noise(&biome, global_x, global_y, &substrate_perlin, seed);
    substrate_for_biome(&biome, noise)
}

/// Generates substrate for Meadow biome at global coordinates.
pub fn generate_meadow_substrate(global_x: i32, global_y: i32, seed: u64) -> Substrate {
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let biome = Biome::Meadow;
    let noise = get_substrate_noise(&biome, global_x, global_y, &substrate_perlin, seed);
    substrate_for_biome(&biome, noise)
}

/// Generates substrate for Forest biome at global coordinates.
pub fn generate_forest_substrate(global_x: i32, global_y: i32, seed: u64) -> Substrate {
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let biome = Biome::Forest;
    let noise = get_substrate_noise(&biome, global_x, global_y, &substrate_perlin, seed);
    substrate_for_biome(&biome, noise)
}

/// Generates substrate for Mountain biome at global coordinates.
pub fn generate_mountain_substrate(global_x: i32, global_y: i32, seed: u64) -> Substrate {
    let substrate_perlin = Perlin::new(seed.wrapping_add(SUBSTRATE_SEED_OFFSET) as u32);
    let biome = Biome::Mountain;
    let noise = get_substrate_noise(&biome, global_x, global_y, &substrate_perlin, seed);
    substrate_for_biome(&biome, noise)
}
