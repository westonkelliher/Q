use crate::types::{Land, World};

pub fn print_world(world: &World, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Iterate through rows (y coordinates)
    for y in y1..=y2 {
        // Iterate through columns (x coordinates) for this row
        for x in x1..=x2 {
            if let Some(land) = world.terrain.get(&(x, y)) {
                print!("{}", land.biome.to_char());
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
    println!("Biome: {:?}", land.biome);
    println!("Tiles (substrate or object):");
    println!("  0 1 2 3 4 5 6 7");
    for (y, row) in land.tiles.iter().enumerate() {
        print!("{} ", y);
        for tile in row.iter() {
            // If there are objects, show only the object (not the substrate)
            if tile.objects.is_empty() {
                print!("{}", tile.substrate.to_char());
            } else {
                // Show first object, or multiple if there are many
                if tile.objects.len() == 1 {
                    print!("{}", tile.objects[0].to_char());
                } else {
                    print!("🔴"); // Multiple objects (red circle - fixed width)
                }
            }
        }
        println!();
    }
}
