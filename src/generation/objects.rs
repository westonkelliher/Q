//! Object generation based on biome type and substrate.
//!
//! Objects (trees, rocks, sticks) are spawned pseudo-randomly with biome-specific
//! placement rates. Trees cannot grow on stone substrate.

use crate::types::{Biome, Object, Substrate};

/// Generates objects for a tile based on its biome, substrate, and a pseudo-random value.
///
/// This function dispatches to biome-specific generation functions that implement
/// substrate-aware placement rules.
pub fn objects_for_biome(
    biome: &Biome,
    substrate: &Substrate,
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    match biome {
        Biome::Lake => generate_lake_objects(seed, land_x, land_y, tile_x, tile_y),
        Biome::Meadow => generate_meadow_objects(substrate, seed, land_x, land_y, tile_x, tile_y),
        Biome::Forest => generate_forest_objects(substrate, seed, land_x, land_y, tile_x, tile_y),
        Biome::Mountain => generate_mountain_objects(substrate, seed, land_x, land_y, tile_x, tile_y),
    }
}

/// Generates objects for Lake biome.
///
/// Lake always places Rock when an object is placed (~7.5% of tiles).
fn generate_lake_objects(
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    const PLACEMENT_THRESHOLD: f64 = 0.075; // 7.5% of tiles
    
    let random_value = tile_random_value(seed, land_x, land_y, tile_x, tile_y);
    
    if random_value < PLACEMENT_THRESHOLD {
        vec![Object::Rock]
    } else {
        Vec::new()
    }
}

/// Generates objects for Meadow biome.
///
/// - Trees: 3-5% of grass/dirt tiles (trees cannot grow on stone)
/// - Rocks/Sticks: ~5-8% of all tiles (Rock 80%, Stick 20%)
fn generate_meadow_objects(
    substrate: &Substrate,
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    const TREE_PLACEMENT_THRESHOLD: f64 = 0.04; // 4% average (middle of 3-5% range)
    const OTHER_PLACEMENT_THRESHOLD: f64 = 0.065; // 6.5% average (middle of 5-8% range)
    
    let random_value = tile_random_value(seed, land_x, land_y, tile_x, tile_y);
    let object_type_value = tile_random_value(seed.wrapping_add(1), land_x, land_y, tile_x, tile_y);
    
    // Check if tile is eligible for trees (grass or dirt only)
    let can_have_tree = matches!(substrate, Substrate::Grass | Substrate::Dirt);
    
    // Try to place a tree first if substrate is eligible
    if can_have_tree && random_value < TREE_PLACEMENT_THRESHOLD {
        return vec![Object::Tree];
    }
    
    // Otherwise, try to place rock or stick
    if random_value < OTHER_PLACEMENT_THRESHOLD {
        if object_type_value < 0.8 {
            vec![Object::Rock]
        } else {
            vec![Object::Stick]
        }
    } else {
        Vec::new()
    }
}

/// Generates objects for Forest biome.
///
/// - Trees: 40% of grass/brush/dirt tiles (trees cannot grow on stone)
/// - Rocks/Sticks: ~8-12% of all tiles (Rock 75%, Stick 25%)
fn generate_forest_objects(
    substrate: &Substrate,
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    const TREE_PLACEMENT_THRESHOLD: f64 = 0.40; // 40% of eligible tiles
    const OTHER_PLACEMENT_THRESHOLD: f64 = 0.10; // 10% average (middle of 8-12% range)
    
    let random_value = tile_random_value(seed, land_x, land_y, tile_x, tile_y);
    let object_type_value = tile_random_value(seed.wrapping_add(1), land_x, land_y, tile_x, tile_y);
    
    // Check if tile is eligible for trees (grass, brush, or dirt only)
    let can_have_tree = matches!(substrate, Substrate::Grass | Substrate::Brush | Substrate::Dirt);
    
    // Use separate random values for trees vs rocks/sticks to allow independent placement
    let tree_value = random_value;
    let rock_stick_value = tile_random_value(seed.wrapping_add(2), land_x, land_y, tile_x, tile_y);
    
    // Try to place a tree if substrate is eligible
    if can_have_tree && tree_value < TREE_PLACEMENT_THRESHOLD {
        return vec![Object::Tree];
    }
    
    // Try to place rock or stick (independent of tree placement)
    if rock_stick_value < OTHER_PLACEMENT_THRESHOLD {
        if object_type_value < 0.75 {
            vec![Object::Rock]
        } else {
            vec![Object::Stick]
        }
    } else {
        Vec::new()
    }
}

/// Generates objects for Mountain biome.
///
/// - Rocks: 15-20% of all tiles (can spawn on stone or dirt)
/// - Trees: 30-40% of dirt patches only (trees cannot grow on stone)
fn generate_mountain_objects(
    substrate: &Substrate,
    seed: u64,
    land_x: i32,
    land_y: i32,
    tile_x: usize,
    tile_y: usize,
) -> Vec<Object> {
    const ROCK_PLACEMENT_THRESHOLD: f64 = 0.175; // 17.5% average (middle of 15-20% range)
    const TREE_PLACEMENT_THRESHOLD: f64 = 0.35; // 35% average (middle of 30-40% range)
    
    let random_value = tile_random_value(seed, land_x, land_y, tile_x, tile_y);
    
    // Check if tile is eligible for trees (dirt only in mountains)
    let can_have_tree = matches!(substrate, Substrate::Dirt);
    
    // Try to place a tree first if substrate is dirt
    if can_have_tree && random_value < TREE_PLACEMENT_THRESHOLD {
        return vec![Object::Tree];
    }
    
    // Otherwise, try to place rock (rocks can spawn on stone or dirt)
    if random_value < ROCK_PLACEMENT_THRESHOLD {
        vec![Object::Rock]
    } else {
        Vec::new()
    }
}

/// Adds sticks deterministically near trees in a land.
///
/// For each tile containing a tree, checks nearby tiles (within 1 tile radius)
/// and deterministically places sticks on some of them based on a deterministic
/// random value. Sticks are only added to tiles that don't already have objects.
/// Reduced likelihood: 5% chance per nearby empty tile (down from 15%).
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
                            
                            // 5% chance to place a stick near a tree (reduced from 15%)
                            if stick_value < 0.05 {
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
