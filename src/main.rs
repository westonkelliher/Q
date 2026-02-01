mod types;
mod generation;
mod io;
mod display;
mod render;
mod terrain_view;
mod land_view;
mod graphics_loop;

use std::collections::HashMap;
use types::World;
use generation::{generate_world, initialize_world};
use io::save_world;
use display::{print_land, print_world};

#[macroquad::main("Q - World Generator")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut use_graphics = false;
    let mut seed = 12347u64; // Default seed
    
    // Parse arguments
    for i in 1..args.len() {
        match args[i].as_str() {
            "--graphics" | "-g" => {
                use_graphics = true;
            }
            arg => {
                // Try to parse as seed if it's a number
                if let Ok(parsed_seed) = arg.parse::<u64>() {
                    seed = parsed_seed;
                }
            }
        }
    }
    
    println!("Initializing world with seed {}...", seed);
    let mut world = World {
        name: "TestWorld".to_string(),
        terrain: HashMap::new(),
    };
    initialize_world(&mut world, seed);
    println!("World '{}' initialized with {} lands", world.name, world.terrain.len());
    
    println!("\nGenerating extra region (11 to 15, -5 to 5)...");
    generate_world(&mut world, seed, 11, -5, 15, 5);
    println!("World now has {} lands", world.terrain.len());
    
    if use_graphics {
        println!("\nStarting graphics mode...");
        println!("Controls: WASD/Arrows to move, Z/X to zoom, ESC to exit");
        graphics_loop::run_graphics_loop(&world).await?;
    } else {
        println!("\nPrinting world overview (showing -5 to 5):");
        println!("(Areas with ⬛ are ungenerated, other areas show biomes)");
        println!("(Use --graphics or -g flag to enable graphics mode)");
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
    }
    
    println!("\nSaving world...");
    save_world(&world)?;
    println!("World saved to worlds/{}.json", world.name);
    
    Ok(())
}
