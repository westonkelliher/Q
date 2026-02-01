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
/// | Meadow   | Rock (80%) or Stick (20%) |
/// | Forest   | Tree (50%), Rock (40%), or Stick (10%) |
/// | Mountain | Rock (70%) or Tree (30%)  |
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
            if object_type_value < 0.8 {
                Object::Rock
            } else {
                Object::Stick
            }
        }
        
        Biome::Forest => {
            if object_type_value < 0.5 {
                Object::Tree
            } else if object_type_value < 0.9 {
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

/// Adds sticks deterministically near trees in a land.
///
/// For each tile containing a tree, checks nearby tiles (within 1 tile radius)
/// and deterministically places sticks on some of them based on a deterministic
/// random value. Sticks are only added to tiles that don't already have objects.
pub fn add_sticks_near_trees(
    tiles: &mut [[crate::types::Tile; 8]; 8],
    seed: u64,
    land_x: i32,
    land_y: i32,
) {
    // Seed offset for stick placement near trees (different from object placement)
    const STICK_NEAR_TREE_SEED_OFFSET: u64 = 2000003;
    
    // Check each tile for trees
    for tile_y in 0..8 {
        for tile_x in 0..8 {
            // Check if this tile has a tree
            let has_tree = tiles[tile_y][tile_x].objects.iter().any(|obj| matches!(obj, crate::types::Object::Tree));
            
            if has_tree {
                // Check nearby tiles (within 1 tile radius, including diagonals)
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        // Skip the tree tile itself
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        
                        let nearby_x = tile_x as i32 + dx;
                        let nearby_y = tile_y as i32 + dy;
                        
                        // Check bounds (0-7)
                        if nearby_x < 0 || nearby_x >= 8 || nearby_y < 0 || nearby_y >= 8 {
                            continue;
                        }
                        
                        let nearby_tile = &mut tiles[nearby_y as usize][nearby_x as usize];
                        
                        // Only add stick if tile doesn't already have objects
                        if nearby_tile.objects.is_empty() {
                            // Deterministically decide if this nearby tile should have a stick
                            // Include BOTH the tree's position AND the nearby tile's position in the seed
                            // This ensures each tree-tile pair has independent randomness
                            let tree_offset = (tile_x as u64)
                                .wrapping_mul(0x517CC1B727220A95)
                                .wrapping_add((tile_y as u64).wrapping_mul(0x5D6DCB8D5C20A2AB));
                            let stick_value = tile_random_value(
                                seed.wrapping_add(STICK_NEAR_TREE_SEED_OFFSET).wrapping_add(tree_offset),
                                land_x,
                                land_y,
                                nearby_x as usize,
                                nearby_y as usize,
                            );
                            
                            // 15% chance to place a stick near a tree
                            if stick_value < 0.15 {
                                nearby_tile.objects.push(crate::types::Object::Stick);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generates a deterministic pseudo-random value between 0.0 and 1.0 for a specific tile.
///
/// Uses a high-quality hash function (SplitMix64-inspired) with proper bit mixing to ensure
/// deterministic placement with good statistical properties (no correlation patterns).
fn tile_random_value(seed: u64, land_x: i32, land_y: i32, tile_x: usize, tile_y: usize) -> f64 {
    // Combine seed with coordinates using XOR and golden ratio-derived constants
    // XOR provides better mixing than addition (avoids correlation when coords change together)
    let mut hash = seed;
    hash ^= (land_x as u64).wrapping_mul(0x9E3779B97F4A7C15); // golden ratio constant
    hash ^= (land_y as u64).wrapping_mul(0xBF58476D1CE4E5B9);
    hash ^= (tile_x as u64).wrapping_mul(0x94D049BB133111EB);
    hash ^= (tile_y as u64).wrapping_mul(0xC6A4A7935BD1E995);
    
    // SplitMix64-style mixing for excellent bit distribution
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xBF58476D1CE4E5B9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94D049BB133111EB);
    hash ^= hash >> 31;
    
    // Convert to f64 in range [0.0, 1.0)
    (hash as f64) / (u64::MAX as f64)
}
