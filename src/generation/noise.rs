//! Noise utilities for terrain generation.
//!
//! Provides helpers for seeded Perlin noise with deterministic offsets.

use ::noise::{NoiseFn, Perlin};

/// Scale factor for biome determination noise.
/// Lower values = larger biome regions.
pub const BIOME_SCALE: f64 = 0.05;

/// Scale factor for height noise.
/// Kept separate from biome scale to allow independent control.
pub const HEIGHT_SCALE: f64 = 0.1;

/// Scale factor for substrate variation within tiles.
/// Higher values = more variation per land (8x8 tiles).
/// With 0.4, a single land spans ~3.2 noise units.
pub const SUBSTRATE_SCALE: f64 = 0.4;

/// Computes a deterministic 2D offset from a seed value.
/// Uses LCG-style hashing to derive x/y offsets.
///
/// The `discriminator` allows deriving different offsets from the same seed
/// (e.g., for different biomes or layers).
pub fn seed_offset(seed: u64, discriminator: u64) -> (f64, f64) {
    let combined = seed.wrapping_add(discriminator);
    let offset_x = ((combined.wrapping_mul(1103515245).wrapping_add(12345)) % 1000000) as f64 / 1000.0;
    let offset_y = ((combined.wrapping_mul(2147483647).wrapping_add(54321)) % 1000000) as f64 / 1000.0;
    (offset_x, offset_y)
}

/// Samples Perlin noise at scaled coordinates with a seed-based offset.
pub fn sample_noise(perlin: &Perlin, x: f64, y: f64, scale: f64, offset: (f64, f64)) -> f64 {
    perlin.get([x * scale + offset.0, y * scale + offset.1])
}

/// Creates a land-specific seed for features that don't need cross-boundary continuity.
/// Uses large primes to distribute seeds across the coordinate space.
pub fn land_local_seed(base_seed: u64, land_x: i32, land_y: i32) -> u64 {
    base_seed
        .wrapping_add((land_x as u64).wrapping_mul(73856093))
        .wrapping_add((land_y as u64).wrapping_mul(19349663))
}
