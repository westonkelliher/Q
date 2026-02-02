/// 8x8 subsystem CLI entry point

use eight_by_eight::{Color, Grid8x8};

fn main() {
    println!("8x8 subsystem");
    
    // Create a new grid with a default color
    let default_color = Color::rgb(0.5, 0.5, 0.5);
    let mut grid = Grid8x8::new(default_color);
    
    // Example: Set some tiles with different colors
    grid.set_color(0, 0, Color::rgb(1.0, 0.0, 0.0)); // Red
    grid.set_color(7, 7, Color::rgb(0.0, 1.0, 0.0)); // Green
    grid.set_color(3, 4, Color::rgb(0.0, 0.0, 1.0)); // Blue
    
    // Example: Add some strings to tiles
    grid.add_string(0, 0, "top-left".to_string());
    grid.add_string(7, 7, "bottom-right".to_string());
    grid.add_string(3, 4, "center".to_string());
    grid.add_string(3, 4, "blue tile".to_string());
    
    // Display some tile information
    println!("\nGrid size: {}x{}", grid.width(), grid.height());
    
    if let Some(tile) = grid.get(0, 0) {
        println!("\nTile at (0, 0):");
        println!("  Color: R={:.2}, G={:.2}, B={:.2}", tile.color.r, tile.color.g, tile.color.b);
        println!("  Strings: {:?}", tile.strings);
    }
    
    if let Some(tile) = grid.get(3, 4) {
        println!("\nTile at (3, 4):");
        println!("  Color: R={:.2}, G={:.2}, B={:.2}", tile.color.r, tile.color.g, tile.color.b);
        println!("  Strings: {:?}", tile.strings);
    }
}
