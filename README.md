# Q

A procedural world generation system built in Rust using Perlin noise.

## Features

- **Procedural World Generation**: Generate infinite worlds using Perlin noise for natural-looking terrain
- **Biome System**: Four distinct biomes (Forest, Meadow, Lake, Mountain) with visual representation
- **Incremental Generation**: Generate terrain on-demand for specific coordinate ranges
- **Persistence**: Save and load worlds to/from JSON files
- **Visualization**: Print world maps with ASCII characters representing different biomes

## World Structure

- **World**: Contains a name and a terrain map
- **Land**: An 8x8 grid of tiles with an associated biome
- **Tile**: Contains a substrate type (grass, dirt, stone, mud, water, brush) and a list of objects (rock, tree, stick)
- **Biome**: Determines the visual representation and characteristics of a land area

## Usage

```rust
// Create a new world
let mut world = World {
    name: "MyWorld".to_string(),
    terrain: HashMap::new(),
};

// Initialize the world (generates terrain from -10 to 10)
initialize_world(&mut world, 12345);

// Generate additional terrain regions
generate_world(&mut world, 12345, 11, -5, 15, 5);

// Print a section of the world
print_world(&world, -5, -5, 5, 5);

// Save the world
save_world(&world)?;

// Load a world
let loaded_world = load_world("MyWorld.json")?;
```

## Biome Characters

- ` Y` - Forest
- ` .` - Meadow
- `~~` - Lake
- `/\` - Mountain
- `##` - Ungenerated terrain

## Dependencies

- `noise` - Perlin noise generation
- `serde` / `serde_json` - Serialization for save/load functionality

## License

[Add your license here]
