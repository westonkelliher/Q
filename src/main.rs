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
use macroquad::prelude::*;

/// Window configuration - sets window size 50% larger than default (1200x900)
/// macOS compatible with high DPI support and resizable window
fn window_conf() -> Conf {
    Conf {
        window_title: "Q - World Generator".to_owned(),
        window_width: 1200,  // 50% larger than default 800
        window_height: 900,  // 50% larger than default 600
        high_dpi: true,      // Enable high DPI support for macOS Retina displays
        window_resizable: true, // Allow window resizing (macOS compatible)
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut use_text_mode = false; // Graphics mode is default
    let mut seed = 12347u64; // Default seed
    
    // Parse arguments
    for i in 1..args.len() {
        match args[i].as_str() {
            "--text" | "-t" | "--print" | "-p" => {
                use_text_mode = true;
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
    
    if use_text_mode {
        println!("\nPrinting world overview (showing -5 to 5):");
        println!("(Areas with ⬛ are ungenerated, other areas show biomes)");
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
    } else {
        println!("\nStarting graphics mode...");
        println!("Controls: WASD/Arrows to move, Z to toggle view, X to toggle adjacent lands, ESC to exit");
        graphics_loop::run_graphics_loop(&world).await?;
    }
    
    println!("\nSaving world...");
    save_world(&world)?;
    println!("World saved to worlds/{}.json", world.name);
    
    Ok(())
}
