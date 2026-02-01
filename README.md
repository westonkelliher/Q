# Q

A procedural world generation system built in Rust using Perlin noise. Generate infinite, deterministic 2D worlds with biomes, terrain, and objects.

## Quick Start

```bash
cargo run
```

This will generate a sample world, display it, and save it to `TestWorld.json`.

## Features

- **🌍 Infinite Worlds**: Generate terrain on-demand for any coordinate range
- **🎲 Deterministic**: Same seed always produces the same world
- **🌲 Biome System**: Four biomes (Forest, Meadow, Lake, Mountain) with unique characteristics
- **🗺️ Detailed Terrain**: Each land contains an 8x8 grid of tiles with substrates and objects
- **🔗 Neighbor-Aware**: Terrain generation considers adjacent biomes for natural transitions
- **💾 Persistent**: Save and load worlds as JSON files

## Usage

```rust
use Q::{World, initialize_world, generate_world, print_world, save_world};
use std::collections::HashMap;

// Create a new world
let mut world = World {
    name: "MyWorld".to_string(),
    terrain: HashMap::new(),
};

// Initialize with a seed (generates -10 to 10)
initialize_world(&mut world, 12345);

// Generate additional regions
generate_world(&mut world, 12345, 11, -5, 15, 5);

// Display the world
print_world(&world, -5, -5, 5, 5);

// Save to file
save_world(&world)?;
```

## World Structure

The world is organized hierarchically:

- **World**: Top-level container with a name and terrain map
- **Land**: An 8×8 grid of tiles at coordinates (x, y), each with a biome
- **Tile**: Individual cell with a substrate type and optional objects

## Biomes

Each land has one of four biomes:

| Biome | Symbol | Description |
|-------|--------|-------------|
| Forest | ` Y` | Dense trees, rocks, and sticks |
| Meadow | ` .` | Open grasslands with occasional rocks |
| Lake | `~~` | Water with mud edges, rare rocks |
| Mountain | `/\` | Stone and dirt, many rocks, occasional trees |

Ungenerated terrain shows as `##`.

## Tile Details

Each tile has:

**Substrates** (ground material):
- `g` - Grass
- `d` - Dirt
- `s` - Stone
- `m` - Mud
- `w` - Water
- `b` - Brush

**Objects** (placed items):
- `T` - Tree
- `R` - Rock
- `S` - Stick
- `*` - Multiple objects

## Examples

### Generate and Display

```rust
let mut world = World {
    name: "Example".to_string(),
    terrain: HashMap::new(),
};
initialize_world(&mut world, 42);
print_world(&world, -3, -3, 3, 3);
```

### Inspect a Specific Land

```rust
if let Some(land) = world.terrain.get(&(0, 0)) {
    print_land(land);
}
```

### Save and Load

```rust
// Save
save_world(&world)?;

// Load
let loaded = load_world("Example.json")?;
```

## Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests
```

## Project Structure

```
src/
├── main.rs          # Demo application
├── lib.rs           # Library API
├── types.rs         # Domain types
├── generation.rs    # World generation logic
├── io.rs            # File I/O
└── display.rs       # Text rendering
tests/
└── integration_tests.rs
```

## Dependencies

- `noise` - Perlin noise generation
- `serde` / `serde_json` - Serialization

## Documentation

For detailed technical documentation including architecture, algorithms, API reference, and extension points, see **[DETAILED_README.md](DETAILED_README.md)**. That document is designed for developers and LLMs working on the codebase.

## License

[Add your license here]
