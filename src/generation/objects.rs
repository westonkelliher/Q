//! Object generation based on biome type.
//!
//! Objects (trees, rocks, sticks) are spawned based on noise values.
//! Unlike substrates, objects use land-local noise since they don't
//! need to blend across boundaries.

use ::noise::{NoiseFn, Perlin};
use crate::types::{Biome, Object};

/// Generates objects for a tile based on its biome and noise value.
///
/// # Object Rules by Biome
///
/// | Biome    | Objects                    | Noise Thresholds                    |
/// |----------|----------------------------|-------------------------------------|
/// | Lake     | Rock (rare)                | > 0.7 → Rock                        |
/// | Meadow   | Rock, Stick                | > 0.5 → Rock, > 0.8 → Stick         |
/// | Forest   | Tree (common), Rock, Stick | > 0.0 → Tree, > 0.6 → Rock, > 0.8 → Stick |
/// | Mountain | Rock (abundant), Tree      | > 0.2 → Rock, > 0.6 → Rock, > 0.9 → Tree |
pub fn objects_for_biome(biome: &Biome, noise: f64) -> Vec<Object> {
    let mut objects = Vec::new();
    
    match biome {
        Biome::Lake => {
            if noise > 0.7 { objects.push(Object::Rock); }
        }
        
        Biome::Meadow => {
            if noise > 0.5 { objects.push(Object::Rock); }
            if noise > 0.8 { objects.push(Object::Stick); }
        }
        
        Biome::Forest => {
            if noise > 0.0 { objects.push(Object::Tree); }
            if noise > 0.6 { objects.push(Object::Rock); }
            if noise > 0.8 { objects.push(Object::Stick); }
        }
        
        Biome::Mountain => {
            if noise > 0.2 { objects.push(Object::Rock); }
            if noise > 0.6 { objects.push(Object::Rock); } // Extra rocks at high noise
            if noise > 0.9 { objects.push(Object::Tree); }
        }
    }
    
    objects
}

/// Calculates object noise for a tile using land-local coordinates.
///
/// Objects don't need cross-boundary blending, so we use a simpler
/// local coordinate system within each land.
pub fn get_object_noise(
    perlin: &Perlin,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> f64 {
    let noise_x = (land_x as f64) + (tile_x as f64) * 0.125;
    let noise_y = (land_y as f64) + (tile_y as f64) * 0.125;
    perlin.get([noise_x * 0.5, noise_y * 0.5])
}
