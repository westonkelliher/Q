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
save_world(&world)?;
```

## Biome Characters

- ` Y` - Forest
- ` .` - Meadow
- `~~` - Lake
- `/\` - Mountain
- `##` - Ungenerated terrain
