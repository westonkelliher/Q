use crate::types::{Biome, Land, Substrate, Tile, World};
use std::collections::HashMap;

/// Creates a hardcoded 5x5 world for the MVP
/// Lands are at coordinates (0,0) through (4,4)
/// Start position is (0,0) - top-left
/// Boss position is (4,4) - bottom-right
pub fn create_hardcoded_world() -> World {
    let mut terrain = HashMap::new();

    // Create a 5x5 grid of lands
    for y in 0..5 {
        for x in 0..5 {
            // Determine biome based on position (mostly Plains, some Forests for variety)
            let biome = if (x + y) % 3 == 0 && (x != 0 || y != 0) {
                Biome::Forest
            } else {
                Biome::Plains
            };

            // Create tiles - all grass substrate, empty objects by default
            let tiles = std::array::from_fn(|_| {
                std::array::from_fn(|_| Tile {
                    substrate: Substrate::Grass,
                    objects: vec![],
                })
            });

            // Add a few trees or rocks for variety
            let mut tiles = tiles;
            // Add a tree at (2, 2) in some lands
            if (x + y) % 2 == 0 {
                tiles[2][2].objects.push(crate::types::Object::Tree);
            }
            // Add a rock at (5, 5) in some other lands
            if (x + y) % 3 == 1 {
                tiles[5][5].objects.push(crate::types::Object::Rock);
            }

            // Create land with all 9 biome fields set to the same biome
            let land = Land {
                tiles,
                center: biome.clone(),
                top: biome.clone(),
                bottom: biome.clone(),
                left: biome.clone(),
                right: biome.clone(),
                top_left: biome.clone(),
                top_right: biome.clone(),
                bottom_left: biome.clone(),
                bottom_right: biome.clone(),
            };

            terrain.insert((x, y), land);
        }
    }

    World {
        name: "MVP World".to_string(),
        terrain,
        seed: 0,
    }
}
