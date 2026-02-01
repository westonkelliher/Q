mod types;
mod generation;
mod io;
mod display;
mod render;
mod camera;
mod terrain_view;
mod land_view;
mod graphics_loop;

use std::collections::HashMap;
use types::World;
use generation::initialize_world;
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
        name: format!("World_{}", seed),
        terrain: HashMap::new(),
        seed,
    };
    initialize_world(&mut world, seed);
    println!("World '{}' initialized with {} lands", world.name, world.terrain.len());

    if use_text_mode {
        // In text mode, show a sample of the world
        let range = 5;
        println!("\nWorld overview (showing {} to {}):", -range, range);
        println!("(Areas with ⬛ are ungenerated, other areas show biomes)");
        print_world(&world, -range, -range, range, range);

        // Show a few sample lands from the center area
        println!("\nSample land details:");
        for (x, y) in [(0, 0), (1, 0), (0, 1)] {
            println!("\nLand at ({}, {}):", x, y);
            if let Some(land) = world.terrain.get(&(x, y)) {
                print_land(land);
            } else {
                println!("  (not generated)");
            }
        }
    } else {
        println!("\nStarting graphics mode...");
        println!("Controls:");
        println!("  WASD/Arrows: Move selection");
        println!("  Z: Toggle between Terrain and Land view");
        println!("  X: Toggle adjacent lands (Land view only)");
        println!("  -/=: Zoom out/in");
        println!("  ESC: Exit");
        graphics_loop::run_graphics_loop(&mut world).await?;
    }
    
    println!("\nSaving world...");
    save_world(&world)?;
    println!("World saved to worlds/{}.json", world.name);
    
    Ok(())
}
