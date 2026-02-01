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
    // Create a seeded Perlin noise generator
    let perlin = Perlin::new(seed as u32);
    
    // Default tile: all dirt substrate, no objects
    let default_tile = Tile {
        substrate: Substrate::Dirt,
        objects: Vec::new(),
    };
    
    // Generate terrain for the specified range
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biome = determine_biome(x, y, &perlin);
            
            let land = Land {
                tiles: std::array::from_fn(|_| std::array::from_fn(|_| default_tile.clone())),
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
    
    println!("\nPrinting large area (showing -12 to 18, -8 to 8):");
    println!("(Areas with ## are ungenerated, other areas show biomes)");
    print_world(&world, -12, -8, 18, 8);
    
    println!("\nSaving world...");
    save_world(&world)?;
    println!("World saved to {}.json", world.name);
    
    Ok(())
}
