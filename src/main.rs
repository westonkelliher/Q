mod types;
mod generation;
mod io;
mod display;

use std::collections::HashMap;
use types::World;
use generation::{generate_world, initialize_world};
use io::save_world;
use display::{print_land, print_world};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing world...");
    let mut world = World {
        name: "TestWorld".to_string(),
        terrain: HashMap::new(),
    };
    initialize_world(&mut world, 12347);
    println!("World '{}' initialized with {} lands", world.name, world.terrain.len());
    
    println!("\nGenerating extra region (11 to 15, -5 to 5)...");
    generate_world(&mut world, 12347, 11, -5, 15, 5);
    println!("World now has {} lands", world.terrain.len());
    
    println!("\nPrinting world overview (showing -5 to 5):");
    println!("(Areas with ## are ungenerated, other areas show biomes)");
    print_world(&world, -5, -5, 5, 5);
    
    println!("\nShowing sample lands:");
    println!("\nLand at (0, 0):");
    if let Some(land) = world.terrain.get(&(0, 0)) {
        print_land(land);
    }
    
    println!("\nLand at (2, -1):");
    if let Some(land) = world.terrain.get(&(2, -1)) {
        print_land(land);
    }
    
    println!("\nSaving world...");
    save_world(&world)?;
    println!("World saved to {}.json", world.name);
    
    Ok(())
}
