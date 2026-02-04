use super::types::{Biome, Land, Substrate, Tile, World, Enemy};
use crate::game::crafting::{CraftingRegistry, ItemInstanceId, ItemId, RecipeId, Provenance};
use std::collections::HashMap;

/// Creates a hardcoded 5x5 world for the MVP
/// Lands are at coordinates (0,0) through (4,4)
/// Start position is (0,0) - top-left
/// Boss position is (4,4) - bottom-right
/// 
/// Biome layout (5x5 grid):
/// - Row 0: Plains, Meadow, Mountain, Mountain, Forest
/// - Row 1: Meadow, Forest, Lake, Lake, Meadow
/// - Row 2: Plains, Forest, Lake, Plains, Forest
/// - Row 3: Meadow, Plains, Plains, Forest, Meadow
/// - Row 4: Forest, Meadow, Plains, Meadow, Mountain (boss)
/// 
/// Requires a registry to create item instances for world objects
pub fn create_hardcoded_world(crafting_registry: &mut CraftingRegistry) -> World {
    let mut terrain = HashMap::new();

    // Hand-crafted 5x5 biome layout for variety
    let biome_grid: [[Biome; 5]; 5] = [
        [Biome::Plains,  Biome::Meadow,  Biome::Mountain, Biome::Mountain, Biome::Forest],
        [Biome::Meadow,  Biome::Forest,   Biome::Lake,    Biome::Lake,     Biome::Meadow],
        [Biome::Plains,  Biome::Forest,   Biome::Lake,    Biome::Plains,   Biome::Forest],
        [Biome::Meadow,  Biome::Plains,   Biome::Plains,  Biome::Forest,   Biome::Meadow],
        [Biome::Forest,  Biome::Meadow,   Biome::Plains,  Biome::Meadow,   Biome::Mountain], // Boss at (4,4)
    ];

    // Create a 5x5 grid of lands
    for y in 0..5 {
        for x in 0..5 {
            let center_biome = biome_grid[y][x].clone();
            
            // Determine edge biomes based on neighbors (simplified - use adjacent biomes)
            let top_biome = if y > 0 { biome_grid[y - 1][x].clone() } else { center_biome.clone() };
            let bottom_biome = if y < 4 { biome_grid[y + 1][x].clone() } else { center_biome.clone() };
            let left_biome = if x > 0 { biome_grid[y][x - 1].clone() } else { center_biome.clone() };
            let right_biome = if x < 4 { biome_grid[y][x + 1].clone() } else { center_biome.clone() };
            
            // Corner biomes (diagonal neighbors)
            let top_left_biome = if y > 0 && x > 0 { biome_grid[y - 1][x - 1].clone() } else { center_biome.clone() };
            let top_right_biome = if y > 0 && x < 4 { biome_grid[y - 1][x + 1].clone() } else { center_biome.clone() };
            let bottom_left_biome = if y < 4 && x > 0 { biome_grid[y + 1][x - 1].clone() } else { center_biome.clone() };
            let bottom_right_biome = if y < 4 && x < 4 { biome_grid[y + 1][x + 1].clone() } else { center_biome.clone() };

            // Generate tiles based on biome
            let tiles = generate_tiles_for_biome(&center_biome, x as i32, y as i32, crafting_registry);

            // Determine if this land has an enemy
            // Start position (0,0) has no enemy
            // Boss position (4,4) has a strong enemy
            // Some intermediate lands have enemies
            let enemy = if (x, y) == (0, 0) {
                // Start position - no enemy
                None
            } else if (x, y) == (4, 4) {
                // Boss - strong enemy
                Some(Enemy::new(20, 8)) // High health, high attack
            } else if (x, y) == (1, 0) || (x, y) == (0, 1) {
                // Early enemies - weak
                Some(Enemy::new(8, 3)) // Low health, low attack
            } else if (x, y) == (2, 1) || (x, y) == (1, 2) || (x, y) == (3, 2) {
                // Mid-game enemies - medium
                Some(Enemy::new(12, 5)) // Medium health, medium attack
            } else if (x, y) == (3, 3) || (x, y) == (4, 3) {
                // Late-game enemies - strong
                Some(Enemy::new(15, 6)) // High health, medium-high attack
            } else {
                // Most lands have no enemy (for now)
                None
            };

            // Create land with proper biome borders
            let land = Land {
                tiles,
                center: center_biome.clone(),
                top: top_biome,
                bottom: bottom_biome,
                left: left_biome,
                right: right_biome,
                top_left: top_left_biome,
                top_right: top_right_biome,
                bottom_left: bottom_left_biome,
                bottom_right: bottom_right_biome,
                enemy,
            };

            terrain.insert((x as i32, y as i32), land);
        }
    }

    World {
        name: "MVP World".to_string(),
        terrain,
        seed: 0,
    }
}

/// Create a simple item instance (for world objects like rock, tree, stick)
fn create_simple_item_instance(crafting_registry: &mut CraftingRegistry, item_id: &str) -> ItemInstanceId {
    let instance_id = crafting_registry.next_instance_id();
    let item_instance = crate::game::crafting::ItemInstance::Simple(
        crate::game::crafting::SimpleInstance {
            id: instance_id,
            definition: ItemId(item_id.to_string()),
            provenance: Provenance {
                recipe_id: RecipeId("world_generated".to_string()),
                consumed_inputs: vec![],
                tool_used: None,
                world_object_used: None,
                crafted_at: 0,
            },
        }
    );
    crafting_registry.register_instance(item_instance);
    instance_id
}

/// Generate tiles for a land based on its center biome
fn generate_tiles_for_biome(biome: &Biome, land_x: i32, land_y: i32, crafting_registry: &mut CraftingRegistry) -> [[Tile; 8]; 8] {
    let mut tiles = std::array::from_fn(|_| {
        std::array::from_fn(|_| Tile {
            substrate: Substrate::Grass,
            objects: vec![],
        })
    });

    match biome {
        Biome::Forest => {
            // Forests: mostly grass with some brush, lots of trees
            for y in 0..8 {
                for x in 0..8 {
                    let tile = &mut tiles[y][x];
                    // Mix of grass and brush substrates
                    if (x + y) % 3 == 0 {
                        tile.substrate = Substrate::Brush;
                    }
                    // Add trees throughout (more dense)
                    if ((x + y) as i32 + land_x + land_y) % 3 == 0 {
                        let tree_instance = create_simple_item_instance(crafting_registry, "tree");
                        tile.objects.push(tree_instance);
                    }
                    // Some sticks on the ground
                    if (x + y) % 5 == 0 && tile.objects.is_empty() {
                        let stick_instance = create_simple_item_instance(crafting_registry, "stick");
                        tile.objects.push(stick_instance);
                    }
                }
            }
        }
        Biome::Meadow => {
            // Meadows: all grass, some trees and sticks
            for y in 0..8 {
                for x in 0..8 {
                    let tile = &mut tiles[y][x];
                    // Occasional trees
                    if ((x + y) as i32 + land_x) % 4 == 0 {
                        let tree_instance = create_simple_item_instance(crafting_registry, "tree");
                        tile.objects.push(tree_instance);
                    }
                    // Some sticks
                    if (x + y) % 6 == 0 && tile.objects.is_empty() {
                        let stick_instance = create_simple_item_instance(crafting_registry, "stick");
                        tile.objects.push(stick_instance);
                    }
                }
            }
        }
        Biome::Lake => {
            // Lakes: mostly water substrate, some mud near edges, rocks
            for y in 0..8 {
                for x in 0..8 {
                    let tile = &mut tiles[y][x];
                    // Center area is water
                    let dist_from_center = ((x as f32 - 3.5).abs() + (y as f32 - 3.5).abs()) / 2.0;
                    if dist_from_center < 2.5 {
                        tile.substrate = Substrate::Water;
                    } else if dist_from_center < 3.5 {
                        tile.substrate = Substrate::Mud;
                    }
                    // Rocks near water edges
                    if dist_from_center > 2.0 && dist_from_center < 3.0 && (x + y) % 3 == 0 {
                        let rock_instance = create_simple_item_instance(crafting_registry, "rock");
                        tile.objects.push(rock_instance);
                    }
                }
            }
        }
        Biome::Mountain => {
            // Mountains: mostly stone substrate, some dirt, lots of rocks
            for y in 0..8 {
                for x in 0..8 {
                    let tile = &mut tiles[y][x];
                    // Mix of stone and dirt
                    if (x + y) % 2 == 0 {
                        tile.substrate = Substrate::Stone;
                    } else {
                        tile.substrate = Substrate::Dirt;
                    }
                    // Many rocks
                    if ((x + y) as i32 + land_x + land_y) % 2 == 0 {
                        let rock_instance = create_simple_item_instance(crafting_registry, "rock");
                        tile.objects.push(rock_instance);
                    }
                }
            }
        }
        Biome::Plains => {
            // Plains: mix of grass and dirt, some trees, rocks, and sticks
            for y in 0..8 {
                for x in 0..8 {
                    let tile = &mut tiles[y][x];
                    // Some dirt patches
                    if (x + y) % 4 == 0 {
                        tile.substrate = Substrate::Dirt;
                    }
                    // Occasional trees
                    if ((x + y) as i32 + land_x) % 5 == 0 {
                        let tree_instance = create_simple_item_instance(crafting_registry, "tree");
                        tile.objects.push(tree_instance);
                    }
                    // Some rocks
                    if (x + y) % 6 == 0 && tile.objects.is_empty() {
                        let rock_instance = create_simple_item_instance(crafting_registry, "rock");
                        tile.objects.push(rock_instance);
                    }
                    // Some sticks
                    if (x + y) % 7 == 0 && tile.objects.is_empty() {
                        let stick_instance = create_simple_item_instance(crafting_registry, "stick");
                        tile.objects.push(stick_instance);
                    }
                }
            }
        }
    }

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_world() -> (World, CraftingRegistry) {
        let mut crafting_registry = CraftingRegistry::new();
        crate::game::crafting::content::register_sample_content(&mut crafting_registry);
        let world = create_hardcoded_world(&mut crafting_registry);
        (world, crafting_registry)
    }

    #[test]
    fn test_create_hardcoded_world() {
        let (world, _crafting_registry) = create_test_world();
        
        assert_eq!(world.name, "MVP World");
        assert_eq!(world.seed, 0);
        assert_eq!(world.terrain.len(), 25); // 5x5 = 25 lands
    }

    #[test]
    fn test_world_has_all_coordinates() {
        let (world, _crafting_registry) = create_test_world();
        
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
        let (world, _crafting_registry) = create_test_world();
        
        // Start position should be (0, 0) - top-left
        let start_land = world.terrain.get(&(0, 0));
        assert!(start_land.is_some());
        
        if let Some(land) = start_land {
            // Start should be Plains biome
            assert_eq!(land.center, Biome::Plains);
        }
    }

    #[test]
    fn test_world_boss_position_biome() {
        let (world, _crafting_registry) = create_test_world();
        
        // Boss position should be (4, 4) - bottom-right
        let boss_land = world.terrain.get(&(4, 4));
        assert!(boss_land.is_some());
        
        if let Some(land) = boss_land {
            // Boss should be Mountain biome (challenging)
            assert_eq!(land.center, Biome::Mountain);
        }
    }

    #[test]
    fn test_land_structure() {
        let (world, _crafting_registry) = create_test_world();
        
        // Check a few lands have correct structure
        for y in 0..5 {
            for x in 0..5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    // Check that all biome fields are set (they may differ from center now)
                    // Just verify they're valid biomes
                    let _ = &land.center;
                    let _ = &land.top;
                    let _ = &land.bottom;
                    let _ = &land.left;
                    let _ = &land.right;
                    let _ = &land.top_left;
                    let _ = &land.top_right;
                    let _ = &land.bottom_left;
                    let _ = &land.bottom_right;
                    
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
        let (world, _crafting_registry) = create_test_world();
        
        // Check that all tiles have substrates (varies by biome now)
        if let Some(land) = world.terrain.get(&(2, 2)) {
            // This is a Lake biome, should have Water substrate in center
            let mut found_water = false;
            for row in &land.tiles {
                for tile in row {
                    // Verify substrate is valid
                    match tile.substrate {
                        Substrate::Grass | Substrate::Dirt | Substrate::Stone | 
                        Substrate::Mud | Substrate::Water | Substrate::Brush => {},
                    }
                    if tile.substrate == Substrate::Water {
                        found_water = true;
                    }
                }
            }
            // Lake biome should have some water tiles
            assert!(found_water, "Lake biome should have water substrate");
        }
    }

    #[test]
    fn test_land_has_some_objects() {
        let (world, _crafting_registry) = create_test_world();
        
        // Check that some lands have objects (trees, rocks, or sticks)
        let mut found_objects = false;
        let mut object_counts = std::collections::HashMap::new();
        for y in 0..5 {
            for x in 0..5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    for row in &land.tiles {
                        for tile in row {
                            if !tile.objects.is_empty() {
                                found_objects = true;
                                for obj in &tile.objects {
                                    *object_counts.entry(format!("{:?}", obj)).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        // At least some lands should have objects based on the generation logic
        assert!(found_objects, "Expected at least some lands to have objects");
        // Should have variety of objects
        assert!(object_counts.len() > 0, "Should have at least one type of object");
    }

    #[test]
    fn test_biome_variety() {
        let (world, _crafting_registry) = create_test_world();
        
        // Check that we have all 5 biomes represented
        let mut biomes_found = std::collections::HashSet::new();
        for y in 0..5 {
            for x in 0..5 {
                if let Some(land) = world.terrain.get(&(x, y)) {
                    biomes_found.insert(format!("{:?}", land.center));
                }
            }
        }
        // Should have at least 4 different biomes (all 5 ideally)
        assert!(biomes_found.len() >= 4, "Expected at least 4 different biomes, found: {:?}", biomes_found);
    }

    #[test]
    fn test_world_no_extra_coordinates() {
        let (world, _crafting_registry) = create_test_world();
        
        // Check that there are no coordinates outside 0-4 range
        for (coords, _) in &world.terrain {
            assert!(coords.0 >= 0 && coords.0 < 5, 
                "X coordinate {} out of range", coords.0);
            assert!(coords.1 >= 0 && coords.1 < 5, 
                "Y coordinate {} out of range", coords.1);
        }
    }
}
