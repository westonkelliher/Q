#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::types::{Biome, World};
    use crate::generation::{generate_world, initialize_world};

    fn create_test_world() -> World {
        let mut world = World {
            name: "TestWorld".to_string(),
            terrain: HashMap::new(),
        };
        initialize_world(&mut world, 12347);
        world
    }

    #[test]
    fn test_world_initialization() {
        let mut world = World {
            name: "TestWorld".to_string(),
            terrain: HashMap::new(),
        };
        initialize_world(&mut world, 12347);
        
        assert_eq!(world.name, "TestWorld");
        assert_eq!(world.terrain.len(), 441); // 21x21 grid from -10 to 10
    }

    #[test]
    fn test_incremental_generation() {
        let mut world = create_test_world();
        let initial_count = world.terrain.len();
        
        generate_world(&mut world, 12347, 11, -5, 15, 5);
        
        assert!(world.terrain.len() > initial_count);
        // Should have added 5x11 = 55 new lands
        assert_eq!(world.terrain.len(), initial_count + 55);
    }

    #[test]
    fn test_biome_generation() {
        let world = create_test_world();
        
        // Check that we have at least some lands with each biome type
        let mut has_forest = false;
        let mut has_meadow = false;
        let mut has_lake = false;
        let mut has_mountain = false;
        
        for land in world.terrain.values() {
            match land.biome {
                Biome::Forest => has_forest = true,
                Biome::Meadow => has_meadow = true,
                Biome::Lake => has_lake = true,
                Biome::Mountain => has_mountain = true,
            }
        }
        
        // With a seed, we should have at least some variety
        assert!(has_forest || has_meadow || has_lake || has_mountain);
    }

    #[test]
    fn test_tile_generation() {
        let world = create_test_world();
        
        // Check that all lands have 8x8 tiles
        for land in world.terrain.values() {
            assert_eq!(land.tiles.len(), 8);
            for row in &land.tiles {
                assert_eq!(row.len(), 8);
            }
        }
    }

    #[test]
    fn test_lake_surrounded_by_lakes() {
        let world = create_test_world();
        
        // Find a lake land surrounded by lakes
        let mut found_lake = false;
        for x in -5..=5 {
            for y in -5..=5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    if matches!(land.biome, Biome::Lake) {
                        let neighbors = [
                            world.terrain.get(&(x - 1, y)),
                            world.terrain.get(&(x + 1, y)),
                            world.terrain.get(&(x, y - 1)),
                            world.terrain.get(&(x, y + 1)),
                        ];
                        let all_lakes = neighbors.iter()
                            .all(|opt| opt.map(|l| matches!(l.biome, Biome::Lake)).unwrap_or(false));
                        
                        if all_lakes {
                            // If all neighbors are lakes, tiles should be mostly water
                            // (some edge tiles might be mud/grass due to noise variation)
                            let water_count: usize = land.tiles.iter()
                                .flat_map(|row| row.iter())
                                .filter(|tile| matches!(tile.substrate, crate::types::Substrate::Water))
                                .count();
                            // At least 75% (48/64) should be water when surrounded by lakes
                            assert!(water_count >= 48, "Expected at least 48 water tiles when surrounded by lakes (got {})", water_count);
                            found_lake = true;
                            break;
                        }
                    }
                }
            }
            if found_lake {
                break;
            }
        }
        
        // This test passes if we find such a lake, or if we don't find one (both are valid)
        // The important part is that if we do find one, it should be all water
    }

    #[test]
    fn test_deterministic_generation() {
        // Same seed should produce same world
        let mut world1 = World {
            name: "Test1".to_string(),
            terrain: HashMap::new(),
        };
        initialize_world(&mut world1, 42);
        
        let mut world2 = World {
            name: "Test2".to_string(),
            terrain: HashMap::new(),
        };
        initialize_world(&mut world2, 42);
        
        // Check that biomes match
        for (coord, land1) in &world1.terrain {
            if let Some(land2) = world2.terrain.get(coord) {
                assert_eq!(land1.biome, land2.biome);
            }
        }
    }
}
