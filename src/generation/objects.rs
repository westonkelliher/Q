//! Object generation based on biome type.
//!
//! Objects (trees, rocks, sticks) are spawned pseudo-randomly with sparse placement.
//! 5-10% of tiles will have an object, determined deterministically by tile coordinates and seed.

use crate::types::{Biome, Object};

/// Probability threshold for object placement (7.5% - middle of 5-10% range).
const OBJECT_PLACEMENT_THRESHOLD: f64 = 0.075;

/// Generates objects for a tile based on its biome and a pseudo-random value.
///
/// Objects are placed sparsely (5-10% of tiles) using completely pseudo-random placement.
/// When an object is placed, the biome determines which type of object appears.
///
/// # Object Rules by Biome
///
/// | Biome    | Object Type                |
/// |----------|----------------------------|
/// | Lake     | Rock                       |
/// | Meadow   | Rock or Stick (random)     |
/// | Forest   | Tree, Rock, or Stick (random) |
/// | Mountain | Rock or Tree (random)      |
pub fn objects_for_biome(
    biome: &Biome,
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    // Generate deterministic pseudo-random value for this tile
    let random_value = tile_random_value(seed, land_x, land_y, tile_x, tile_y);
    
    // Check if this tile should have an object (5-10% chance)
    if random_value >= OBJECT_PLACEMENT_THRESHOLD {
        return Vec::new();
    }
    
    // Generate a second random value to determine object type
    let object_type_value = tile_random_value(seed.wrapping_add(1), land_x, land_y, tile_x, tile_y);
    
    // Place one object based on biome
    let object = match biome {
        Biome::Lake => Object::Rock,
        
        Biome::Meadow => {
            if object_type_value < 0.5 {
                Object::Rock
            } else {
                Object::Stick
            }
        }
        
        Biome::Forest => {
            if object_type_value < 0.5 {
                Object::Tree
            } else if object_type_value < 0.75 {
                Object::Rock
            } else {
                Object::Stick
            }
        }
        
        Biome::Mountain => {
            if object_type_value < 0.7 {
                Object::Rock
            } else {
                Object::Tree
            }
        }
    };
    
    vec![object]
}

/// Generates a deterministic pseudo-random value between 0.0 and 1.0 for a specific tile.
///
/// Uses a simple hash function based on the seed and tile coordinates to ensure
/// deterministic placement while being completely pseudo-random (no noise patterns).
fn tile_random_value(seed: u64, land_x: i32, land_y: i32, tile_x: usize, tile_y: usize) -> f64 {
    // Combine seed with tile coordinates using prime multipliers for good distribution
    let hash = seed
        .wrapping_add((land_x as u64).wrapping_mul(73856093))
        .wrapping_add((land_y as u64).wrapping_mul(19349663))
        .wrapping_add((tile_x as u64).wrapping_mul(15485863))
        .wrapping_add((tile_y as u64).wrapping_mul(32452843));
    
    // Use LCG-style hashing to generate a value
    let value = hash
        .wrapping_mul(1103515245)
        .wrapping_add(12345);
    
    // Convert to f64 in range [0.0, 1.0)
    // Use modulo with a large prime to get good distribution
    (value % 1000000007) as f64 / 1000000007.0
}
