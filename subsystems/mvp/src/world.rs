use crate::types::{Biome, Land, Substrate, Tile, World};
use std::collections::HashMap;

/// Creates a hardcoded 5x5 world for the MVP
/// Lands are at coordinates (0,0) through (4,4)
/// Start position is (0,0) - top-left
/// Boss position is (4,4) - bottom-right
pub fn create_hardcoded_world() -> World {
    let mut terrain = HashMap::new();

    // Create a 5x5 grid of lands
    for y in 0..5 {
        for x in 0..5 {
            // Determine biome based on position (mostly Plains, some Forests for variety)
            let biome = if (x + y) % 3 == 0 && (x != 0 || y != 0) {
                Biome::Forest
            } else {
                Biome::Plains
            };

            // Create tiles - all grass substrate, empty objects by default
            let tiles = std::array::from_fn(|_| {
                std::array::from_fn(|_| Tile {
                    substrate: Substrate::Grass,
                    objects: vec![],
                })
            });

            // Add a few trees or rocks for variety
            let mut tiles = tiles;
            // Add a tree at (2, 2) in some lands
            if (x + y) % 2 == 0 {
                tiles[2][2].objects.push(crate::types::Object::Tree);
            }
            // Add a rock at (5, 5) in some other lands
            if (x + y) % 3 == 1 {
                tiles[5][5].objects.push(crate::types::Object::Rock);
            }

            // Create land with all 9 biome fields set to the same biome
            let land = Land {
                tiles,
                center: biome.clone(),
                top: biome.clone(),
                bottom: biome.clone(),
                left: biome.clone(),
                right: biome.clone(),
                top_left: biome.clone(),
                top_right: biome.clone(),
                bottom_left: biome.clone(),
                bottom_right: biome.clone(),
            };

            terrain.insert((x, y), land);
        }
    }

    World {
        name: "MVP World".to_string(),
        terrain,
        seed: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hardcoded_world() {
        let world = create_hardcoded_world();
        
        assert_eq!(world.name, "MVP World");
        assert_eq!(world.seed, 0);
        assert_eq!(world.terrain.len(), 25); // 5x5 = 25 lands
    }

    #[test]
    fn test_world_has_all_coordinates() {
        let world = create_hardcoded_world();
        
        // Check that all coordinates from (0,0) to (4,4) exist
        for y in 0..5 {
            for x in 0..5 {
                assert!(world.terrain.contains_key(&(x, y)), 
                    "Missing land at ({}, {})", x, y);
            }
        }
    }

    #[test]
    fn test_world_start_position() {
        let world = create_hardcoded_world();
        
        // Start position should be (0, 0) - top-left
        let start_land = world.terrain.get(&(0, 0));
        assert!(start_land.is_some());
        
        if let Some(land) = start_land {
            // Start should be Plains biome
            assert_eq!(land.center, Biome::Plains);
        }
    }

    #[test]
    fn test_world_boss_position() {
        let world = create_hardcoded_world();
        
        // Boss position should be (4, 4) - bottom-right
        let boss_land = world.terrain.get(&(4, 4));
        assert!(boss_land.is_some());
    }

    #[test]
    fn test_land_structure() {
        let world = create_hardcoded_world();
        
        // Check a few lands have correct structure
        for y in 0..5 {
            for x in 0..5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    // Check that all biome fields are set
                    assert_eq!(land.center, land.top);
                    assert_eq!(land.center, land.bottom);
                    assert_eq!(land.center, land.left);
                    assert_eq!(land.center, land.right);
                    assert_eq!(land.center, land.top_left);
                    assert_eq!(land.center, land.top_right);
                    assert_eq!(land.center, land.bottom_left);
                    assert_eq!(land.center, land.bottom_right);
                    
                    // Check tiles are 8x8
                    assert_eq!(land.tiles.len(), 8);
                    for row in &land.tiles {
                        assert_eq!(row.len(), 8);
                    }
                }
            }
        }
    }

    #[test]
    fn test_land_tiles_have_substrates() {
        let world = create_hardcoded_world();
        
        // Check that all tiles have substrates
        if let Some(land) = world.terrain.get(&(2, 2)) {
            for row in &land.tiles {
                for tile in row {
                    // All tiles should have Grass substrate by default
                    assert_eq!(tile.substrate, Substrate::Grass);
                }
            }
        }
    }

    #[test]
    fn test_land_has_some_objects() {
        let world = create_hardcoded_world();
        
        // Check that some lands have objects (trees or rocks)
        let mut found_objects = false;
        for y in 0..5 {
            for x in 0..5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    for row in &land.tiles {
                        for tile in row {
                            if !tile.objects.is_empty() {
                                found_objects = true;
                            }
                        }
                    }
                }
            }
        }
        // At least some lands should have objects based on the generation logic
        assert!(found_objects, "Expected at least some lands to have objects");
    }

    #[test]
    fn test_world_no_extra_coordinates() {
        let world = create_hardcoded_world();
        
        // Check that there are no coordinates outside 0-4 range
        for (coords, _) in &world.terrain {
            assert!(coords.0 >= 0 && coords.0 < 5, 
                "X coordinate {} out of range", coords.0);
            assert!(coords.1 >= 0 && coords.1 < 5, 
                "Y coordinate {} out of range", coords.1);
        }
    }
}
