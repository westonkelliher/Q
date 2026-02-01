use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::de::Visitor;
use std::fmt;
use noise::{NoiseFn, Perlin};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Substrate {
    Grass,
    Dirt,
    Stone,
    Mud,
    Water,
    Brush,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Object {
    Rock,
    Tree,
    Stick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Tile {
    substrate: Substrate,
    objects: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Biome {
    Forest,
    Meadow,
    Lake,
    Mountain,
}

impl Biome {
    fn to_chars(&self) -> &str {
        match self {
            Biome::Forest => " Y",
            Biome::Meadow => " .",
            Biome::Lake => "~~",
            Biome::Mountain => "/\\",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Land {
    tiles: [[Tile; 8]; 8],
    biome: Biome,
}

fn serialize_terrain<S>(terrain: &HashMap<(i32, i32), Land>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(terrain.len()))?;
    for (k, v) in terrain {
        let key = format!("{},{}", k.0, k.1);
        map.serialize_entry(&key, v)?;
    }
    map.end()
}

fn deserialize_terrain<'de, D>(deserializer: D) -> Result<HashMap<(i32, i32), Land>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TerrainVisitor;

    impl<'de> Visitor<'de> for TerrainVisitor {
        type Value = HashMap<(i32, i32), Land>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with string keys")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((key, value)) = access.next_entry::<String, Land>()? {
                let parts: Vec<&str> = key.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        map.insert((x, y), value);
                    }
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(TerrainVisitor)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct World {
    name: String,
    #[serde(serialize_with = "serialize_terrain", deserialize_with = "deserialize_terrain")]
    terrain: HashMap<(i32, i32), Land>,
}

fn determine_biome(x: i32, y: i32, perlin: &Perlin) -> Biome {
    // Sample Perlin noise at the coordinate
    // Scale coordinates for better noise distribution
    let noise_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
    
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

fn generate_land_terrain(land_x: i32, land_y: i32, biome: &Biome, world: &World, seed: u64) -> [[Tile; 8]; 8] {
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

fn save_world(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.json", world.name);
    let json = serde_json::to_string_pretty(world)?;
    fs::write(filename, json)?;
    Ok(())
}

fn load_world(path: &str) -> Result<World, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let world: World = serde_json::from_str(&contents)?;
    Ok(world)
}

fn generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Create a seeded Perlin noise generator for biome determination
    let perlin = Perlin::new(seed as u32);
    
    // Generate terrain for the specified range
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biome = determine_biome(x, y, &perlin);
            
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

fn initialize_world(world: &mut World, seed: u64) {
    generate_world(world, seed, -10, -10, 10, 10);
}

fn print_world(world: &World, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Iterate through rows (y coordinates)
    for y in y1..=y2 {
        // Iterate through columns (x coordinates) for this row
        for x in x1..=x2 {
            if let Some(land) = world.terrain.get(&(x, y)) {
                print!("{}", land.biome.to_chars());
            } else {
                // If land doesn't exist, print spaces
                print!("##");
            }
        }
        // Newline after each row
        println!();
    }
}

impl Substrate {
    fn to_char(&self) -> char {
        match self {
            Substrate::Grass => 'g',
            Substrate::Dirt => 'd',
            Substrate::Stone => 's',
            Substrate::Mud => 'm',
            Substrate::Water => 'w',
            Substrate::Brush => 'b',
        }
    }
}

impl Object {
    fn to_char(&self) -> char {
        match self {
            Object::Rock => 'R',
            Object::Tree => 'T',
            Object::Stick => 'S',
        }
    }
}

fn print_land(land: &Land) {
    println!("Biome: {:?}", land.biome);
    println!("Tiles (substrate + objects):");
    println!("  0 1 2 3 4 5 6 7");
    for (y, row) in land.tiles.iter().enumerate() {
        print!("{} ", y);
        for tile in row.iter() {
            // Print substrate character
            print!("{}", tile.substrate.to_char());
            // Print objects if any
            if tile.objects.is_empty() {
                print!(" ");
            } else {
                // Show first object, or multiple if there are many
                if tile.objects.len() == 1 {
                    print!("{}", tile.objects[0].to_char());
                } else {
                    print!("*"); // Multiple objects
                }
            }
        }
        println!();
    }
}

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
    
    println!("\n=== Testing terrain generation ===");
    
    // Find a lake land and check its neighbors
    println!("\nFinding a lake land surrounded by lakes...");
    let mut found_lake = false;
    for x in -5..=5 {
        for y in -5..=5 {
            if let Some(land) = world.terrain.get(&(x, y)) {
                if matches!(land.biome, Biome::Lake) {
                    // Check if all neighbors are also lakes
                    let neighbors = [
                        world.terrain.get(&(x - 1, y)),
                        world.terrain.get(&(x + 1, y)),
                        world.terrain.get(&(x, y - 1)),
                        world.terrain.get(&(x, y + 1)),
                    ];
                    let all_lakes = neighbors.iter()
                        .all(|opt| opt.map(|l| matches!(l.biome, Biome::Lake)).unwrap_or(false));
                    
                    if all_lakes {
                        println!("\nLand at ({}, {}) - Lake surrounded by 4 lake neighbors:", x, y);
                        print_land(land);
                        found_lake = true;
                        break;
                    }
                }
            }
        }
        if found_lake {
            break;
        }
    }
    
    if !found_lake {
        println!("No lake land with all 4 lake neighbors found. Showing a regular lake:");
        if let Some(land) = world.terrain.get(&(-3, 3)) {
            print_land(land);
        }
    }
    
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
