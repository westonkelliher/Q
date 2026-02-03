use mvp::{create_hardcoded_world, GameState, print_world, print_land};

fn main() {
    println!("Creating hardcoded MVP world...");
    let world = create_hardcoded_world();
    
    println!("\nWorld overview (5x5 grid):");
    print_world(&world, 0, 0, 4, 4);
    
    println!("\nInitializing game state...");
    let mut game_state = GameState::new(world);
    
    println!("\nStarting position: {:?}", game_state.current_land());
    println!("View mode: {:?}", game_state.view_mode);
    
    // Test movement between lands
    println!("\n=== Testing terrain movement ===");
    println!("Current land: {:?}", game_state.current_land());
    
    game_state.move_terrain(1, 0); // Move right
    println!("After moving right: {:?}", game_state.current_land());
    
    game_state.move_terrain(0, 1); // Move down
    println!("After moving down: {:?}", game_state.current_land());
    
    game_state.move_terrain(-1, 0); // Move left
    println!("After moving left: {:?}", game_state.current_land());
    
    // Test entering land view
    println!("\n=== Testing enter land view ===");
    game_state.enter_land();
    println!("View mode: {:?}", game_state.view_mode);
    println!("Current tile: {:?}", game_state.current_tile());
    
    // Test movement within land
    println!("\n=== Testing land movement ===");
    game_state.move_land(1, 0); // Move right
    println!("After moving right: {:?}", game_state.current_tile());
    
    game_state.move_land(0, 1); // Move down
    println!("After moving down: {:?}", game_state.current_tile());
    
    game_state.move_land(-1, 0); // Move left
    println!("After moving left: {:?}", game_state.current_tile());
    
    // Test exiting land view
    println!("\n=== Testing exit land view ===");
    game_state.exit_land();
    println!("View mode: {:?}", game_state.view_mode);
    println!("Current land: {:?}", game_state.current_land());
    
    // Test coordinate clamping
    println!("\n=== Testing coordinate clamping ===");
    game_state.move_terrain(10, 10); // Try to move beyond bounds
    println!("After trying to move (10, 10): {:?}", game_state.current_land());
    
    game_state.enter_land();
    game_state.move_land(10, 10); // Try to move beyond bounds
    println!("After trying to move tile (10, 10): {:?}", game_state.current_tile());
    
    // Show a sample land
    println!("\n=== Sample land details ===");
    let (land_x, land_y) = game_state.current_land();
    if let Some(land) = game_state.world.terrain.get(&(land_x, land_y)) {
        println!("Land at ({}, {}):", land_x, land_y);
        print_land(land);
    }
    
    println!("\nAll tests completed!");
}
