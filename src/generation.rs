use noise::{NoiseFn, Perlin};
use crate::types::{Biome, Land, Object, Substrate, Tile, World};

pub fn determine_biome(x: i32, y: i32, perlin: &Perlin, seed: u64) -> Biome {
    // Create a seed-based offset to ensure (0, 0) isn't always the same biome
    // Use a simple hash-like function to derive offset from seed
    let offset_x = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) % 1000000) as f64 / 1000000.0 * 1000.0;
    let offset_y = ((seed.wrapping_mul(2147483647).wrapping_add(54321)) % 1000000) as f64 / 1000000.0 * 1000.0;
    
    // Sample Perlin noise at the coordinate with seed-based offset
    // Scale coordinates for better noise distribution
    let noise_value = perlin.get([(x as f64 * 0.1) + offset_x, (y as f64 * 0.1) + offset_y]);
    
    // Perlin noise returns values roughly in range [-1.0, 1.0]
    // Map to biome based on noise value ranges
    if noise_value < -0.3 {
        Biome::Lake
    } else if noise_value < 0.0 {
        Biome::Meadow
    } else if noise_value < 0.4 {
        Biome::Forest
    } else {
        Biome::Mountain
    }
}

pub fn generate_land_terrain(land_x: i32, land_y: i32, biome: &Biome, world: &World, seed: u64) -> [[Tile; 8]; 8] {
    // Check the four adjacent lands
    let neighbors = [
        world.terrain.get(&(land_x - 1, land_y)), // West
        world.terrain.get(&(land_x + 1, land_y)), // East
        world.terrain.get(&(land_x, land_y - 1)), // North
        world.terrain.get(&(land_x, land_y + 1)), // South
    ];
    
    // Count how many neighbors match this biome
    let matching_neighbors = neighbors.iter()
        .filter(|opt| opt.map(|land| &land.biome) == Some(biome))
        .count();
    
    // Create a Perlin noise generator for this specific land
    // Use land coordinates to create a unique seed offset
    let land_seed = seed.wrapping_add((land_x as u64).wrapping_mul(73856093).wrapping_add((land_y as u64).wrapping_mul(19349663)));
    let perlin = Perlin::new(land_seed as u32);
    
    // Generate tiles for the 8x8 grid
    std::array::from_fn(|tile_y| {
        std::array::from_fn(|tile_x| {
            // Use fine-grained noise for tile-level variation
            // Offset by land coordinates and tile position
            let noise_x = (land_x as f64) + (tile_x as f64) * 0.125;
            let noise_y = (land_y as f64) + (tile_y as f64) * 0.125;
            let noise_value = perlin.get([noise_x * 0.5, noise_y * 0.5]);
            
            // Adjust thresholds based on matching neighbors
            // More matching neighbors = more uniform/pure biome
            let uniformity_factor = matching_neighbors as f64 * 0.2; // 0.0 to 0.8
            
            // Determine substrate based on biome, noise, and neighbor uniformity
            let substrate = match biome {
                Biome::Lake => {
                    // If all 4 neighbors are lakes, make it all water
                    if matching_neighbors == 4 {
                        Substrate::Water
                    } else {
                        // Lakes are mostly water with some mud/grass edges
                        // More matching neighbors = more water
                        let threshold = -0.2 + uniformity_factor;
                        if noise_value < threshold {
                            Substrate::Water
                        } else if noise_value < 0.0 + uniformity_factor {
                            Substrate::Mud
                        } else {
                            Substrate::Grass
                        }
                    }
                }
                Biome::Meadow => {
                    // If all 4 neighbors are meadows, make it all grass
                    if matching_neighbors == 4 {
                        Substrate::Grass
                    } else {
                        // More uniform meadows = more grass
                        let threshold = -0.3 - uniformity_factor;
                        if noise_value < threshold {
                            Substrate::Dirt
                        } else {
                            Substrate::Grass
                        }
                    }
                }
                Biome::Forest => {
                    // If all 4 neighbors are forests, make it mostly grass/brush
                    if matching_neighbors == 4 {
                        // Pure forest: mostly grass with some brush
                        if noise_value < 0.3 {
                            Substrate::Grass
                        } else {
                            Substrate::Brush
                        }
                    } else {
                        // More uniform forests = more grass/brush, less dirt
                        let dirt_threshold = -0.4 - uniformity_factor;
                        let brush_threshold = 0.2 + uniformity_factor;
                        if noise_value < dirt_threshold {
                            Substrate::Dirt
                        } else if noise_value < brush_threshold {
                            Substrate::Grass
                        } else {
                            Substrate::Brush
                        }
                    }
                }
                Biome::Mountain => {
                    // If all 4 neighbors are mountains, make it all stone
                    if matching_neighbors == 4 {
                        Substrate::Stone
                    } else {
                        // More uniform mountains = more stone
                        let threshold = -0.2 - uniformity_factor;
                        if noise_value < threshold {
                            Substrate::Stone
                        } else if noise_value < 0.2 {
                            Substrate::Dirt
                        } else {
                            Substrate::Grass
                        }
                    }
                }
            };
            
            // Generate objects based on biome, noise, and neighbor uniformity
            let mut objects = Vec::new();
            
            match biome {
                Biome::Lake => {
                    // Lakes rarely have objects, maybe some rocks
                    // Even less objects if surrounded by lakes
                    let threshold = 0.5 + uniformity_factor;
                    if noise_value > threshold {
                        objects.push(Object::Rock);
                    }
                }
                Biome::Meadow => {
                    // Meadows have occasional rocks and sticks
                    let rock_threshold = 0.3 - uniformity_factor;
                    let stick_threshold = 0.6 - uniformity_factor;
                    if noise_value > rock_threshold {
                        objects.push(Object::Rock);
                    }
                    if noise_value > stick_threshold {
                        objects.push(Object::Stick);
                    }
                }
                Biome::Forest => {
                    // Forests have trees, rocks, and sticks
                    // More trees if surrounded by forests
                    let tree_threshold = -0.2 - uniformity_factor;
                    let rock_threshold = 0.4 - uniformity_factor;
                    let stick_threshold = 0.7 - uniformity_factor;
                    if noise_value > tree_threshold {
                        objects.push(Object::Tree);
                    }
                    if noise_value > rock_threshold {
                        objects.push(Object::Rock);
                    }
                    if noise_value > stick_threshold {
                        objects.push(Object::Stick);
                    }
                }
                Biome::Mountain => {
                    // Mountains have rocks and occasional trees
                    // More rocks if surrounded by mountains
                    let rock_threshold1 = 0.0 - uniformity_factor;
                    let rock_threshold2 = 0.5 - uniformity_factor;
                    let tree_threshold = 0.8 - uniformity_factor;
                    if noise_value > rock_threshold1 {
                        objects.push(Object::Rock);
                    }
                    if noise_value > rock_threshold2 {
                        objects.push(Object::Rock);
                    }
                    if noise_value > tree_threshold {
                        objects.push(Object::Tree);
                    }
                }
            }
            
            Tile {
                substrate,
                objects,
            }
        })
    })
}

pub fn generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Create a seeded Perlin noise generator for biome determination
    let perlin = Perlin::new(seed as u32);
    
    // Generate terrain for the specified range
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biome = determine_biome(x, y, &perlin, seed);
            
            // Generate terrain within the land (check neighbors)
            let tiles = generate_land_terrain(x, y, &biome, world, seed);
            
            let land = Land {
                tiles,
                biome,
            };
            
            world.terrain.insert((x, y), land);
        }
    }
}

pub fn initialize_world(world: &mut World, seed: u64) {
    generate_world(world, seed, -10, -10, 10, 10);
}
