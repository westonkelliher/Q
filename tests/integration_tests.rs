use std::collections::HashMap;
use Q::types::World;
use Q::generation::{generate_world, initialize_world};
use Q::io::{load_world, save_world};
use Q::display::{print_land, print_world};

#[test]
fn test_save_and_load_world() {
    let mut world = World {
        name: "TestSaveLoad".to_string(),
        terrain: HashMap::new(),
    };
    initialize_world(&mut world, 999);
    
    // Save the world
    save_world(&world).expect("Failed to save world");
    
    // Load it back
    let loaded_world = load_world("TestSaveLoad.json").expect("Failed to load world");
    
    assert_eq!(world.name, loaded_world.name);
    assert_eq!(world.terrain.len(), loaded_world.terrain.len());
    
    // Check that center biomes match
    for (coord, original_land) in &world.terrain {
        if let Some(loaded_land) = loaded_world.terrain.get(coord) {
            assert_eq!(original_land.center, loaded_land.center);
            assert_eq!(original_land.top, loaded_land.top);
            assert_eq!(original_land.bottom, loaded_land.bottom);
            assert_eq!(original_land.left, loaded_land.left);
            assert_eq!(original_land.right, loaded_land.right);
            assert_eq!(original_land.top_left, loaded_land.top_left);
            assert_eq!(original_land.top_right, loaded_land.top_right);
            assert_eq!(original_land.bottom_left, loaded_land.bottom_left);
            assert_eq!(original_land.bottom_right, loaded_land.bottom_right);
        }
    }
    
    // Clean up
    std::fs::remove_file("worlds/TestSaveLoad.json").ok();
}

#[test]
fn test_world_display_functions() {
    let mut world = World {
        name: "DisplayTest".to_string(),
        terrain: HashMap::new(),
    };
    initialize_world(&mut world, 123);
    
    // These should not panic
    print_world(&world, -2, -2, 2, 2);
    
    if let Some(land) = world.terrain.get(&(0, 0)) {
        print_land(land);
    }
}

#[test]
fn test_large_world_generation() {
    let mut world = World {
        name: "LargeWorld".to_string(),
        terrain: HashMap::new(),
    };
    
    // Generate a larger area
    generate_world(&mut world, 42, -20, -20, 20, 20);
    
    assert_eq!(world.terrain.len(), 41 * 41); // 41x41 grid
    
    // Verify we have variety of biomes
    let mut has_forest = false;
    let mut has_meadow = false;
    let mut has_lake = false;
    let mut has_mountain = false;
    
    for land in world.terrain.values() {
        match land.center {
            Q::types::Biome::Forest => has_forest = true,
            Q::types::Biome::Meadow => has_meadow = true,
            Q::types::Biome::Lake => has_lake = true,
            Q::types::Biome::Mountain => has_mountain = true,
        }
    }
    
    // Should have multiple biomes
    let unique_biomes = [has_forest, has_meadow, has_lake, has_mountain]
        .iter()
        .filter(|&&b| b)
        .count();
    assert!(unique_biomes > 1, "Expected multiple biomes, found {}", unique_biomes);
}
