# 8x8 Subsystem

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `7e46b70`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

---

## Notes for LLMs

When modifying this subsystem:

1. **Update Tests**: Add tests for new features
2. **Update This README**: Keep documentation current with changes
3. **Update Commit Hash Before Committing**: When asked to commit, update the "Previous Commit" hash at the top of this file to reference the commit that existed BEFORE the changes being committed

### Commit Workflow

When the user says "commit":
1. Check `git log --oneline -1` for current commit hash
2. Update "Previous Commit" in this README to that hash
3. Update "Last Updated" date if significant changes
4. Stage changes and commit with descriptive message

---

## Overview

The 8x8 subsystem serves as the base for various systems that involve the land view. This subsystem provides an 8x8 grid of tiles, where each tile has a color and a vector of strings.

## Features

- **8x8 Grid**: Fixed-size 8x8 grid of tiles
- **Tile Color**: Each tile has an RGBA color (f32 values 0.0-1.0)
- **Tile Strings**: Each tile can store a vector of strings for metadata or labels
- **Safe Access**: Bounds-checked access methods (`get`, `get_mut`, `set`)
- **Index Support**: Direct indexing with `grid[(x, y)]` syntax (panics on out-of-bounds)
- **Convenience Methods**: Helper methods for setting colors and managing strings

## File Structure

```
src/
├── main.rs    # CLI entry point with example usage
└── lib.rs     # Core library functionality (Color, Tile, Grid8x8)
```

## Usage

### Basic Example

```rust
use eight_by_eight::{Color, Grid8x8};

// Create a new grid with a default color
let mut grid = Grid8x8::new(Color::rgb(0.5, 0.5, 0.5));

// Set a tile's color
grid.set_color(0, 0, Color::rgb(1.0, 0.0, 0.0)); // Red

// Add strings to a tile
grid.add_string(0, 0, "top-left".to_string());
grid.add_string(0, 0, "corner".to_string());

// Access a tile
if let Some(tile) = grid.get(0, 0) {
    println!("Color: {:?}", tile.color);
    println!("Strings: {:?}", tile.strings);
}

// Direct indexing (panics on out-of-bounds)
grid[(3, 4)].color = Color::rgb(0.0, 1.0, 0.0); // Green
```

### Running the Example

```bash
cargo run
```

## API

### Core Types

#### `Color`

RGBA color representation with f32 values (0.0-1.0).

```rust
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

**Methods:**
- `Color::new(r, g, b, a)` - Create a color with RGBA values
- `Color::rgb(r, g, b)` - Create a color with RGB values (alpha defaults to 1.0)

#### `Tile`

A single tile in the grid with a color and vector of strings.

```rust
pub struct Tile {
    pub color: Color,
    pub strings: Vec<String>,
}
```

**Methods:**
- `Tile::new(color)` - Create a tile with a color and empty strings vector
- `Tile::with_strings(color, strings)` - Create a tile with a color and initial strings

#### `Grid8x8`

An 8x8 grid of tiles.

```rust
pub struct Grid8x8 {
    // Internal tile storage
}
```

**Methods:**
- `Grid8x8::new(default_color)` - Create a new grid with all tiles set to the default color
- `get(x, y)` - Get a reference to the tile at (x, y), returns `Option<&Tile>`
- `get_mut(x, y)` - Get a mutable reference to the tile at (x, y), returns `Option<&mut Tile>`
- `set(x, y, tile)` - Set the tile at (x, y), returns `bool` indicating success
- `set_color(x, y, color)` - Set the color of the tile at (x, y), returns `bool` indicating success
- `add_string(x, y, string)` - Add a string to the tile at (x, y), returns `bool` indicating success
- `clear_strings(x, y)` - Clear all strings from the tile at (x, y), returns `bool` indicating success
- `width()` - Returns 8 (grid width)
- `height()` - Returns 8 (grid height)

**Indexing:**
- `grid[(x, y)]` - Direct access to tile (panics if out of bounds)
- `grid[(x, y)] = tile` - Direct assignment (panics if out of bounds)

**Note:** Coordinates are zero-indexed, with (0, 0) at the top-left and (7, 7) at the bottom-right.
