use crate::game::world::types::{Land, World};

pub fn print_world(world: &World, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Iterate through rows (y coordinates)
    for y in y1..=y2 {
        // Iterate through columns (x coordinates) for this row
        for x in x1..=x2 {
            if let Some(land) = world.terrain.get(&(x, y)) {
                print!("{}", land.center.to_char());
            } else {
                // If land doesn't exist, print ungenerated marker
                print!("⬛");
            }
        }
        // Newline after each row
        println!();
    }
}

pub fn print_land(land: &Land) {
    println!("Center Biome: {:?}", land.center);
    println!("Tiles (substrate or object):");
    println!("  0 1 2 3 4 5 6 7");
    for (y, row) in land.tiles.iter().enumerate() {
        print!("{} ", y);
        for tile in row.iter() {
            // If there are items, show indicator (can't display item without registry)
            if tile.items.is_empty() {
                print!("{}", tile.substrate.to_char());
            } else {
                // Show item indicator
                if tile.items.len() == 1 {
                    print!("📦"); // Box emoji for single item
                } else {
                    print!("🔴"); // Red circle for multiple items
                }
            }
        }
        println!();
    }
}
