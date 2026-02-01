# Q

A procedural world generation system built in Rust using Perlin noise.

## Usage

```rust
let mut world = World {
    name: "MyWorld".to_string(),
    terrain: HashMap::new(),
};
initialize_world(&mut world, 12345);
generate_world(&mut world, 12345, 11, -5, 15, 5);
print_world(&world, -5, -5, 5, 5);
print_land(&world.terrain.get(&(0, 0)).unwrap());
save_world(&world)?;
```

## Features

- **Biome-level generation**: Each land has a biome (Forest, Meadow, Lake, Mountain)
- **Tile-level generation**: Each land contains an 8x8 grid of tiles with substrates and objects
- **Neighbor-aware**: Terrain generation considers adjacent land biomes for more natural transitions
- **Incremental generation**: Generate terrain on-demand for specific coordinate ranges

## Biome Characters

- ` Y` - Forest
- ` .` - Meadow
- `~~` - Lake
- `/\` - Mountain
- `##` - Ungenerated terrain

## Tile Substrates

- `G` - Grass
- `D` - Dirt
- `S` - Stone
- `M` - Mud
- `W` - Water
- `B` - Brush

## Tile Objects

- `T` - Tree
- `r` - Rock
- `s` - Stick
