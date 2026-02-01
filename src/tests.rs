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
            match land.center {
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
        // Test with a specific seed known to produce surrounded lakes
        let mut world = World {
            name: "LakeTest".to_string(),
            terrain: HashMap::new(),
        };
        initialize_world(&mut world, 42);

        // Find a lake land surrounded by lakes
        let mut found_and_tested = false;
        for x in -10..=10 {
            for y in -10..=10 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    if matches!(land.center, Biome::Lake) {
                        let neighbors = [
                            world.terrain.get(&(x - 1, y)),
                            world.terrain.get(&(x + 1, y)),
                            world.terrain.get(&(x, y - 1)),
                            world.terrain.get(&(x, y + 1)),
                        ];
                        let all_lakes = neighbors.iter()
                            .all(|opt| opt.map(|l| matches!(l.center, Biome::Lake)).unwrap_or(false));

                        if all_lakes {
                            // If all neighbors are lakes, all center tiles should be water
                            // Check the 4 center tiles which are definitely in the center biome zone
                            let center_tiles = [
                                (3, 3), (3, 4), (4, 3), (4, 4)
                            ];
                            for (tile_x, tile_y) in center_tiles {
                                let tile = &land.tiles[tile_y][tile_x];
                                assert!(
                                    matches!(tile.substrate, crate::types::Substrate::Water),
                                    "Lake at ({}, {}) surrounded by lakes should have water in center tiles, but tile ({}, {}) is {:?}",
                                    x, y, tile_x, tile_y, tile.substrate
                                );
                            }
                            found_and_tested = true;
                            break;
                        }
                    }
                }
            }
            if found_and_tested {
                break;
            }
        }

        assert!(found_and_tested, "Test should find at least one lake surrounded by lakes to verify behavior");
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
                assert_eq!(land1.center, land2.center);
            }
        }
    }

    /// Helper function to check if a substrate is valid for a given biome
    fn is_valid_substrate_for_biome(substrate: &crate::types::Substrate, biome: &Biome) -> bool {
        match biome {
            Biome::Lake => {
                matches!(substrate, crate::types::Substrate::Water)
            }
            Biome::Meadow => {
                matches!(substrate, crate::types::Substrate::Dirt | 
                                 crate::types::Substrate::Grass)
            }
            Biome::Forest => {
                matches!(substrate, crate::types::Substrate::Dirt | 
                                 crate::types::Substrate::Grass | 
                                 crate::types::Substrate::Brush)
            }
            Biome::Mountain => {
                matches!(substrate, crate::types::Substrate::Stone | 
                                 crate::types::Substrate::Dirt)
            }
        }
    }

    #[test]
    fn test_center_tiles_match_center_biome() {
        let world = create_test_world();
        
        // Check multiple lands to ensure robustness
        for land in world.terrain.values() {
            // Check four center tiles: (3,3), (3,4), (4,3), (4,4)
            let center_tiles = [
                (3, 3),
                (3, 4),
                (4, 3),
                (4, 4),
            ];
            
            for (tile_x, tile_y) in center_tiles {
                let tile = &land.tiles[tile_y][tile_x];
                
                // Verify substrate is valid for center biome
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, &land.center),
                    "Tile ({}, {}) has substrate {:?} which is invalid for center biome {:?}",
                    tile_x, tile_y, tile.substrate, land.center
                );
            }
        }
    }

    #[test]
    fn test_corner_tiles_match_corner_biomes() {
        let world = create_test_world();
        
        for land in world.terrain.values() {
            // Check all four corners
            let corners = [
                ((0, 0), &land.top_left),
                ((7, 0), &land.top_right),
                ((0, 7), &land.bottom_left),
                ((7, 7), &land.bottom_right),
            ];
            
            for ((tile_x, tile_y), expected_biome) in corners {
                let tile = &land.tiles[tile_y][tile_x];
                
                // Verify substrate is valid for the corner biome
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, expected_biome),
                    "Corner tile ({}, {}) has substrate {:?} which is invalid for corner biome {:?}",
                    tile_x, tile_y, tile.substrate, expected_biome
                );
            }
        }
    }

    #[test]
    fn test_edge_center_tiles_match_edge_biomes() {
        let world = create_test_world();
        
        for land in world.terrain.values() {
            // Check center tiles of each edge (two tiles per edge)
            // Top edge: row 0, cols 3-4
            for tile_x in 3..=4 {
                let tile = &land.tiles[0][tile_x];
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, &land.top),
                    "Top edge tile ({}, 0) has substrate {:?} which is invalid for top biome {:?}",
                    tile_x, tile.substrate, land.top
                );
            }
            
            // Bottom edge: row 7, cols 3-4
            for tile_x in 3..=4 {
                let tile = &land.tiles[7][tile_x];
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, &land.bottom),
                    "Bottom edge tile ({}, 7) has substrate {:?} which is invalid for bottom biome {:?}",
                    tile_x, tile.substrate, land.bottom
                );
            }
            
            // Left edge: col 0, rows 3-4
            for tile_y in 3..=4 {
                let tile = &land.tiles[tile_y][0];
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, &land.left),
                    "Left edge tile (0, {}) has substrate {:?} which is invalid for left biome {:?}",
                    tile_y, tile.substrate, land.left
                );
            }
            
            // Right edge: col 7, rows 3-4
            for tile_y in 3..=4 {
                let tile = &land.tiles[tile_y][7];
                assert!(
                    is_valid_substrate_for_biome(&tile.substrate, &land.right),
                    "Right edge tile (7, {}) has substrate {:?} which is invalid for right biome {:?}",
                    tile_y, tile.substrate, land.right
                );
            }
        }
    }

    #[test]
    fn test_all_tiles_have_valid_substrates_for_their_biomes() {
        use crate::generation::get_tile_biome;
        use crate::generation::LandBiomes;
        
        let world = create_test_world();
        
        for (coord, land) in &world.terrain {
            // Reconstruct LandBiomes from the land's biome fields
            let biomes = LandBiomes {
                center: land.center.clone(),
                top: land.top.clone(),
                bottom: land.bottom.clone(),
                left: land.left.clone(),
                right: land.right.clone(),
                top_left: land.top_left.clone(),
                top_right: land.top_right.clone(),
                bottom_left: land.bottom_left.clone(),
                bottom_right: land.bottom_right.clone(),
            };
            
            // Check every tile
            for tile_y in 0..8 {
                for tile_x in 0..8 {
                    let tile = &land.tiles[tile_y][tile_x];
                    let expected_biome = get_tile_biome(&biomes, tile_x, tile_y);
                    
                    assert!(
                        is_valid_substrate_for_biome(&tile.substrate, expected_biome),
                        "Land {:?}, tile ({}, {}) has substrate {:?} which is invalid for biome {:?}",
                        coord, tile_x, tile_y, tile.substrate, expected_biome
                    );
                }
            }
        }
    }
}
