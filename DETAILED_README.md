# Detailed Technical Documentation for Q

This document provides comprehensive technical context for LLMs working on this codebase. It covers architecture, design decisions, algorithms, and implementation details.

> **Last Updated**: 2026-01-31  
> **Commit**: `d81d211`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Module Structure](#module-structure)
4. [Core Algorithms](#core-algorithms)
5. [Data Structures](#data-structures)
6. [API Reference](#api-reference)
7. [Testing Strategy](#testing-strategy)
8. [Design Decisions](#design-decisions)
9. [Extension Points](#extension-points)
10. [Common Patterns](#common-patterns)

---

## Project Overview

**Q** is a procedural world generation system that creates infinite, deterministic 2D worlds using Perlin noise. The world is organized hierarchically:

- **World**: Top-level container with a name and terrain map
- **Land**: 8x8 grid of tiles at coordinates (x, y), with 9 biomes arranged in a 3x3 pattern
- **Tile**: Individual cell with a substrate type and optional objects

### Key Characteristics

- **Deterministic**: Same seed produces identical worlds
- **Incremental**: Generate terrain on-demand for specific coordinate ranges
- **9-Biome System**: Each land contains 9 biomes (center, 4 edges, 4 corners) using biome sub-coordinates
- **Edge Sharing**: Adjacent lands automatically share edge biomes through deterministic biome sub-coordinate system
- **Biome-based**: Tile generation uses zone-based mapping (center/edge/corner) to determine biome per tile

---

## Architecture

### Module Organization

The codebase follows a modular structure:

```
src/
├── main.rs          # Binary entry point with CLI (accepts seed and --graphics flag)
├── lib.rs           # Library root (re-exports public API)
├── types.rs         # Domain types and enums
├── generation.rs    # World generation algorithms
├── io.rs            # File I/O and serialization
├── display.rs       # Text-based rendering
├── terrain_view.rs  # Terrain view system (biome overview)
├── land_view.rs     # Land view system (detailed 8x8 tile grid)
├── graphics_loop.rs # Main graphics loop (coordinates views)
├── render/          # Renderer abstraction layer
│   ├── mod.rs       # Renderer trait and types
│   └── macroquad.rs # Macroquad renderer implementation
└── tests.rs         # Unit tests (compiled only in test mode)
tests/
└── integration_tests.rs  # Integration tests
```

### Module Dependencies

```
main.rs → types, generation, io, display, render, terrain_view, land_view, graphics_loop
lib.rs → types, generation, io, display, terrain_view, land_view, render, tests
types.rs → (no dependencies on other modules)
generation.rs → types
io.rs → types
display.rs → types
terrain_view.rs → render, types
land_view.rs → render, types
graphics_loop.rs → terrain_view, land_view, render::macroquad, types
render/mod.rs → types
render/macroquad.rs → render, types, macroquad
tests.rs → types, generation
```

---

## Module Structure

### `types.rs` - Domain Types

**Purpose**: Defines all core data structures and enums.

**Key Types**:

- `Substrate`: Ground material (Grass, Dirt, Stone, Mud, Water, Brush)
- `Object`: Placed items (Rock, Tree, Stick)
- `Tile`: Combines substrate + objects
- `Biome`: Land classification (Forest, Meadow, Lake, Mountain)
- `Land`: 8x8 tile grid + 9 biomes (center, top, bottom, left, right, top_left, top_right, bottom_left, bottom_right)
- `World`: Container with name + terrain HashMap

**Important Details**:

- All types derive `Serialize`, `Deserialize` for JSON persistence
- `World.terrain` uses custom serialization (see `io.rs`) because HashMap keys are tuples
- `Biome`, `Substrate`, `Object` have `to_char()` methods that return emoji representations for display
  - `Biome::to_char() -> &str`: Returns multi-character emoji strings ("🟩", "🟨", "🟦", "⬜")
  - `Substrate::to_char() -> char`: Returns single emoji characters (circles: '🟢', '🟤', etc.)
  - `Object::to_char() -> char`: Returns single emoji characters ('⚫', '🟩', '🟤')

**Serialization Note**: `World.terrain` uses `(i32, i32)` as keys, which JSON doesn't support directly. Custom serializers convert to/from `"x,y"` string keys.

### `generation.rs` - World Generation

**Purpose**: Contains all procedural generation logic.

**Key Functions**:

1. **`determine_biome(x, y, perlin, seed)`**
   - Uses Perlin noise to assign biomes
   - Applies seed-based offset to ensure (0, 0) varies with different seeds
   - Noise sampled at `[(x * 0.1) + offset_x, (y * 0.1) + offset_y]` where offsets are derived from seed
   - Thresholds: `<-0.3` Lake, `-0.3..0` Meadow, `0..0.4` Forest, `>=0.4` Mountain

2. **`calculate_land_biomes(land_x, land_y, perlin, seed)`**
   - Calculates 9 biomes for a land using biome sub-coordinate system
   - **Formula**: For land (lx, ly), biome coords are:
     - X: `(2*lx - 1)`, `(2*lx)`, `(2*lx + 1)` → left, center, right
     - Y: `(2*ly - 1)`, `(2*ly)`, `(2*ly + 1)` → top, center, bottom
   - Returns `LandBiomes` struct with all 9 biomes
   - **Edge Sharing**: Adjacent lands share edge biomes automatically (e.g., land (1,0) left edge matches land (0,0) right edge)

3. **`get_tile_biome(biomes, tile_x, tile_y)`**
   - Maps 8x8 tile coordinates to one of 9 biomes
   - **Simple zone-based mapping**:
     - Corners (1 tile each): (0,0), (7,0), (0,7), (7,7)
     - Edges (6 tiles each): row 0 cols 1-6, row 7 cols 1-6, col 0 rows 1-6, col 7 rows 1-6
     - Center (36 tiles): rows 1-6, cols 1-6

4. **`generate_land_terrain(land_x, land_y, biomes, seed)`**
   - Generates 8x8 tile grid for a land using 9-biome system
   - Uses `get_tile_biome()` to determine biome for each tile
   - Creates per-land Perlin generator with seed derived from coordinates
   - Samples noise at fine-grained level for tile variation
   - **Simplified**: No longer checks neighbors (edge sharing handled by biome sub-coordinates)

5. **`generate_world(world, seed, x1, y1, x2, y2)`**
   - Generates terrain for coordinate range [x1..=x2, y1..=y2]
   - Creates master Perlin generator from seed
   - Calls `calculate_land_biomes` then `generate_land_terrain` for each coordinate
   - Assigns all 9 biome fields to `Land` struct

4. **`initialize_world(world, seed)`**
   - Convenience function: generates [-10, -10] to [10, 10] (441 lands)

**Algorithm Details**:

- **Seed Derivation**: Each land gets unique Perlin generator via:
  ```rust
  seed + (land_x * 73856093) + (land_y * 19349663)
  ```
  Uses prime multipliers to avoid collisions.

- **Uniformity Factor**: When all 4 neighbors match the biome:
  - Lakes → all water tiles
  - Meadows → all grass tiles
  - Forests → mostly grass/brush
  - Mountains → all stone tiles

- **Noise Sampling**: Tile-level noise uses:
  ```rust
  noise_x = land_x + tile_x * 0.125
  noise_y = land_y + tile_y * 0.125
  perlin.get([noise_x * 0.5, noise_y * 0.5])
  ```

### `io.rs` - File I/O

**Purpose**: Handles saving/loading worlds to/from JSON.

**Key Functions**:

- `save_world(world)` → Saves to `worlds/{world.name}.json` (creates `worlds/` directory if needed)
- `load_world(path)` → Loads world from JSON file
  - Supports absolute paths or simple names (e.g., "MyWorld")
  - Simple names automatically resolved to `worlds/{name}.json`
  - Automatically handles `.json` extension (strips and re-adds as needed)

**Custom Serialization**:

- `serialize_terrain`: Converts `HashMap<(i32, i32), Land>` to JSON map with string keys `"x,y"`
- `deserialize_terrain`: Converts back using `TerrainVisitor` that parses `"x,y"` keys

**Note**: Uses `serde_json::to_string_pretty` for human-readable output.

### `display.rs` - Text Rendering

**Purpose**: Provides text-based visualization of worlds and lands.

**Functions**:

- `print_world(world, x1, y1, x2, y2)`: Prints biome overview grid
  - Uses `Biome::to_char()` for emoji representation (1 emoji per biome)
  - Shows ⬛ for ungenerated terrain

- `print_land(land)`: Prints detailed 8x8 tile grid
  - Displays biome type above the grid
  - Shows substrate emoji when tile has no objects
  - Shows object emoji when present (substrate hidden - mutually exclusive)
  - Shows 🔴 (red circle) for tiles with multiple objects
  - Includes coordinate headers (0-7 for both axes)

### `render/mod.rs` - Renderer Abstraction

**Purpose**: Defines the `Renderer` trait that abstracts graphics operations, allowing different rendering backends (macroquad, Bevy, etc.).

**Key Types**:

- `Renderer`: Trait defining graphics operations
  - `init()`: Initialize the renderer
  - `clear(color)`: Clear screen to color
  - `draw_tile(...)`: Render a single tile with substrate/objects
  - `draw_biome_overview(...)`: Render biome-level overview
  - `draw_selection_indicator(...)`: Draw selection highlight
  - `draw_grid(...)`: Draw grid overlay
  - `present()`: Present the rendered frame
  - `should_close()`: Check if window should close
  - `get_mouse_pos()`: Get mouse coordinates
  - `get_keys_pressed()`: Get currently pressed keys
  - `window_size()`: Get viewport dimensions
- `Color`: RGBA color representation (f32 values 0.0-1.0)
- `Key`: Input key enumeration (Arrow keys, WASD, Z/X for view switching, Escape, etc.)
- `RenderError`: Error type with variants (InitializationFailed, RenderingFailed, Other)

**Design**: The abstraction is designed to be simple enough for immediate-mode APIs (like macroquad) while being complete enough for ECS-based engines (like Bevy). This allows easy migration between backends.

### `render/macroquad.rs` - Macroquad Renderer

**Purpose**: Implements the `Renderer` trait using macroquad as the graphics backend.

**Features**:

- Color mapping for substrates, biomes, and objects
- Selection indicator rendering (bright yellow-orange border with corner markers)
- Grid overlay rendering for Land view
- Input handling (keyboard and mouse)
- Multi-object rendering: Shows red indicator (40% size) when tile has multiple objects, vs 60% sized single object
- Window size querying
- **Shadow color function**: Natural shadow effect using cascading color shifts (darkens and blue-shifts)
- **Biome border rendering**: Colored borders showing biome transitions using edge/corner biomes

**Color Schemes**:
- **Substrates**: Grass (green), Dirt (brown), Stone (gray), Mud (dark brown), Water (blue), Brush (yellow-green)
- **Biomes**: Forest (dark green), Meadow (light green/yellow), Lake (blue), Mountain (gray/white)
- **Objects**: Rock (dark gray), Tree (green), Stick (brown)

**Shadow Color Algorithm**:
- `shadow_color(color)`: Creates natural shadow effect using cascading color shifts
  - Red: `color.r * 0.6`
  - Green: `color.g * 0.6 + r * 0.05` (shifted by red)
  - Blue: `color.b * 0.65 + g * 0.05` (shifted by green, higher multiplier preserves blue)
  - Creates darker, blue-shifted shadows that preserve original color character

**Renderer Methods**:
- `draw_biome_overview()`: Draws single biome square
- `draw_biome_overview_with_borders()`: Draws biome square with colored borders
  - Center area uses center biome color
  - Borders (2px) use edge biomes with shadow effect
  - Corners use corner biomes with shadow effect
  - Shows biome transitions between adjacent lands

### `terrain_view.rs` - Terrain View System

**Purpose**: Self-contained terrain view system for biome overview rendering.

**Key Types**:

- `TerrainCamera`: Camera/viewport state for terrain view with land-level selection

**TerrainCamera**:

- **Position**: `x: f32, y: f32` - current camera position (smooth following)
- **Target**: `target_x: f32, target_y: f32` - target position (land center)
- **Selection**: `selected_land_x: i32, selected_land_y: i32` - currently selected land
- **Tile Size**: Fixed at 48.0 pixels for terrain view

**Key Methods**:

- `new()`: Initialize camera at origin
- `world_to_screen()`: Convert world coordinates to screen coordinates
- `update()`: Smoothly interpolate camera toward target (follow speed: 8.0)
- `move_selection()`: Move selection in discrete steps (one land per keypress)
- `update_target()`: Update target position based on current selection
- `set_selected_land()`: Set selected land (used when switching from land view)
- `sync_position_from()`: Sync camera position from another camera (for view switching)

**Key Functions**:

- `render()`: Renders biome overview with colored borders showing biome transitions
  - Each land displays as single square with center biome color
  - Borders (2px) colored by edge biomes with shadow effect
  - Corners colored by corner biomes with shadow effect
  - Visualizes the 9-biome system and edge sharing between lands
- `handle_input()`: Processes movement input, returns true if view should switch to land view

**Coordinate System**:

- Uses world coordinates directly (land positions)
- Camera tracks land centers at `(land_x, land_y)`

### `land_view.rs` - Land View System

**Purpose**: Self-contained land view system for detailed 8x8 tile grid rendering.

**Key Types**:

- `LandCamera`: Camera/viewport state for land view with tile-level selection

**LandCamera**:

- **Position**: `x: f32, y: f32` - current camera position (smooth following)
- **Target**: `target_x: f32, target_y: f32` - target position (selected tile position)
- **Land Selection**: `selected_land_x: i32, selected_land_y: i32` - currently viewed land
- **Tile Selection**: `selected_tile_x: usize, selected_tile_y: usize` - selected tile within land (0-7)
- **Tile Size**: Fixed at 64.0 pixels for land view

**Key Methods**:

- `new()`: Initialize camera at origin with tile selection at center (4, 4)
- `world_to_screen()`: Convert world coordinates to screen coordinates (for land center)
- `update()`: Smoothly interpolate camera toward target (follow speed: 8.0)
- `move_selection()`: Move tile selection within land (clamped to 0-7 range)
- `update_target()`: Update target position based on selected tile world position
- `set_land()`: Set which land is being viewed (used when switching from terrain view)
- `sync_position_from()`: Sync camera position from another camera (prevents snapping when switching views)
- `get_selected_tile_world_pos()`: Get world position of selected tile

**Key Functions**:

- `render()`: Renders detailed 8x8 grid with tiles, grid overlay, and selection indicator
- `handle_input()`: Processes movement input, returns true if view should switch to terrain view

**Coordinate System**:

- Uses screen-space positioning for tiles (centered grid, direct pixel positioning)
- Land center is at `(land_x + 0.5, land_y + 0.5)` in world coordinates
- Tiles are positioned directly in screen space relative to land center

### `graphics_loop.rs` - Graphics Loop

**Purpose**: Main graphics loop that coordinates between terrain and land view systems.

**Key Types**:

- `ViewMode`: Enum for tracking active view (`Terrain`, `Land`)

**Architecture**:

- **Separate Cameras**: Maintains independent `TerrainCamera` and `LandCamera` instances
- **View Coordination**: Routes input to appropriate view's `handle_input()` function
- **View Switching**: When switching views, syncs camera positions to prevent snapping:
  - Terrain → Land: Syncs land camera to land center `(land_x + 0.5, land_y + 0.5)`
  - Land → Terrain: Syncs terrain camera to land center `(land_x, land_y)`

**Features**:

- Discrete movement handling (key_pressed, not key_down)
- Camera smooth following (both cameras update each frame)
- View mode switching (Z for Land view, X for Terrain view)
- UI text display

**Controls**:
- WASD/Arrow keys: Move selection (discrete steps, view-dependent)
- Z: Switch to Land View
- X: Switch to Terrain View
- ESC: Exit

### `lib.rs` - Library Root

**Purpose**: Exposes public API and re-exports commonly used types.

**Re-exports**: All public types and functions from modules for convenience.

**Note**: Binary (`main.rs`) and library (`lib.rs`) share the same modules but are separate compilation units.

---

## Core Algorithms

### Biome Determination

```rust
offset_x = hash_function(seed) * 1000.0
offset_y = hash_function(seed) * 1000.0
noise_value = perlin.get([(x * 0.1) + offset_x, (y * 0.1) + offset_y])
if noise_value < -0.3 => Lake
else if noise_value < 0.0 => Meadow
else if noise_value < 0.4 => Forest
else => Mountain
```

**Note**: The seed-based offset ensures that coordinate (0, 0) produces different biomes for different seeds, rather than always being the same biome.

### Tile Substrate Generation

For each biome, thresholds are adjusted by `uniformity_factor`:

- **Lake**: `threshold = -0.2 + uniformity_factor`
  - Below threshold → Water
  - Above threshold but < `0.0 + uniformity_factor` → Mud
  - Otherwise → Grass

- **Meadow**: `threshold = -0.3 - uniformity_factor`
  - Below threshold → Dirt
  - Otherwise → Grass

- **Forest**: Uses two thresholds
  - `dirt_threshold = -0.4 - uniformity_factor`
  - `brush_threshold = 0.2 + uniformity_factor`
  - Below dirt_threshold → Dirt
  - Between → Grass
  - Above brush_threshold → Brush

- **Mountain**: `threshold = -0.2 - uniformity_factor`
  - Below threshold → Stone
  - Between threshold and 0.2 → Dirt
  - Above 0.2 → Grass

### Object Generation

Objects are added based on noise thresholds (lower threshold = more objects):

- **Lake**: Rocks only, threshold `0.5 + uniformity_factor`
- **Meadow**: Rocks (`0.3 - uniformity_factor`) and Sticks (`0.6 - uniformity_factor`)
- **Forest**: Trees (`-0.2 - uniformity_factor`), Rocks (`0.4 - uniformity_factor`), Sticks (`0.7 - uniformity_factor`)
- **Mountain**: Multiple Rocks (`0.0 - uniformity_factor`, `0.5 - uniformity_factor`), Trees (`0.8 - uniformity_factor`)

---

## Data Structures

### World Hierarchy

```
World
├── name: String
└── terrain: HashMap<(i32, i32), Land>
    └── Land
        ├── biome: Biome
        └── tiles: [[Tile; 8]; 8]
            └── Tile
                ├── substrate: Substrate
                └── objects: Vec<Object>
```

### Coordinate System

- **Land coordinates**: `(i32, i32)` - can be negative, zero-centered
- **Tile coordinates**: `(0..8, 0..8)` within each land
- **World coordinates**: Not explicitly stored; derived from land + tile positions

### Memory Considerations

- Each `Land` contains 64 `Tile` structs
- Each `Tile` has a `Vec<Object>` (typically 0-3 items)
- Worlds can grow large: 441 lands = ~28K tiles
- Consider memory usage when generating very large worlds

---

## API Reference

### Public API (from `lib.rs`)

**Types**:
- `Biome`, `Land`, `Object`, `Substrate`, `Tile`, `World`

**Functions**:
- `determine_biome(x: i32, y: i32, perlin: &Perlin, seed: u64) -> Biome`
- `generate_land_terrain(land_x: i32, land_y: i32, biome: &Biome, world: &World, seed: u64) -> [[Tile; 8]; 8]`
- `generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32)`
- `initialize_world(world: &mut World, seed: u64)`
- `load_world(path: &str) -> Result<World, Box<dyn std::error::Error>>`
- `save_world(world: &World) -> Result<(), Box<dyn std::error::Error>>`
- `print_land(land: &Land)`
- `print_world(world: &World, x1: i32, y1: i32, x2: i32, y2: i32)`
- `render_terrain_view(renderer, world, camera)` - from `terrain_view::render`
- `render_land_view(renderer, world, camera)` - from `land_view::render`
- `handle_terrain_input(renderer, camera) -> bool` - from `terrain_view::handle_input`
- `handle_land_input(renderer, camera) -> bool` - from `land_view::handle_input`
- `TerrainCamera`, `LandCamera` types

### Usage Pattern

```rust
use Q::{World, initialize_world, generate_world, save_world, print_world};
use std::collections::HashMap;

let mut world = World {
    name: "MyWorld".to_string(),
    terrain: HashMap::new(),
};
initialize_world(&mut world, 12345);
generate_world(&mut world, 12345, 11, -5, 15, 5);
print_world(&world, -5, -5, 5, 5);
save_world(&world)?;
```

---

## Testing Strategy

### Unit Tests (`src/tests.rs`)

Located in `#[cfg(test)]` module, compiled only during testing.

**Test Coverage**:
1. `test_world_initialization`: Verifies world creation and initial generation
2. `test_incremental_generation`: Tests adding new regions
3. `test_biome_generation`: Ensures biome variety
4. `test_tile_generation`: Validates 8x8 grid structure
5. `test_lake_surrounded_by_lakes`: Checks neighbor-aware generation (at least 75% water when surrounded by lakes)
6. `test_deterministic_generation`: Verifies same seed = same world

**Helper Functions**:
- `create_test_world()`: Creates standard test world with seed 12347

### Integration Tests (`tests/integration_tests.rs`)

Tests the library as an external user would use it.

**Test Coverage**:
1. `test_save_and_load_world`: Round-trip serialization
2. `test_world_display_functions`: Display functions don't panic
3. `test_large_world_generation`: Performance and correctness at scale

**Running Tests**:
- `cargo test` - All tests
- `cargo test --lib` - Unit tests only
- `cargo test --test integration_tests` - Integration tests only

---

## Design Decisions

### Why Perlin Noise?

- Smooth, natural-looking variation
- Deterministic with seed
- Efficient to sample
- Good for terrain generation

### Why 8x8 Tiles per Land?

- Balance between detail and memory
- Powers of 2 are efficient
- Small enough for detailed inspection, large enough for meaningful areas

### Why Neighbor-Aware Generation?

- Creates more natural biome transitions
- Reduces jarring boundaries
- Makes uniform regions (all lakes, all mountains) more consistent

### Why Custom Serialization?

- JSON doesn't support tuple keys
- String keys `"x,y"` are human-readable
- Maintains compatibility with standard JSON tools

### Why Separate Binary and Library?

- Allows library to be used by other projects
- Binary provides simple demo/CLI
- Tests can import library easily

### Why Separate Terrain and Land View Systems?

- **Independence**: Each view system is self-contained with its own camera and state management
- **Clarity**: Separates concerns - terrain view handles land-level selection, land view handles tile-level selection
- **Maintainability**: Easier to modify one view without affecting the other
- **Coordinate Systems**: Each view uses its own coordinate system optimized for its rendering needs
- **Smooth Transitions**: View switching syncs camera positions to prevent visual snapping

---

## Extension Points

### Adding New Biomes

1. Add variant to `Biome` enum in `types.rs`
2. Add `to_char()` representation
3. Add generation logic in `generate_land_terrain()`:
   - Substrate thresholds
   - Object generation rules
4. Update `determine_biome()` thresholds if needed

### Adding New Substrates

1. Add variant to `Substrate` enum
2. Add `to_char()` representation
3. Use in `generate_land_terrain()` for appropriate biomes

### Adding New Objects

1. Add variant to `Object` enum
2. Add `to_char()` representation
3. Add generation logic in `generate_land_terrain()`

### Changing Tile Grid Size

1. Update `Land.tiles` type from `[[Tile; 8]; 8]` to desired size
2. Update `generate_land_terrain()` to generate correct size
3. Update `print_land()` coordinate headers
4. Update tests that check grid size

### Adding New Generation Features

- **Rivers**: Add to `generate_land_terrain()` using noise patterns
- **Structures**: Add to `Object` enum and generation logic
- **Height/Elevation**: Add `height: f64` to `Tile` struct
- **Resources**: Add `resources: HashMap<Resource, u32>` to `Tile`

---

## Common Patterns

### Creating a World

```rust
let mut world = World {
    name: "MyWorld".to_string(),
    terrain: HashMap::new(),
};
initialize_world(&mut world, seed);
```

### Generating Specific Region

```rust
generate_world(&mut world, seed, x1, y1, x2, y2);
```

### Accessing Terrain

```rust
if let Some(land) = world.terrain.get(&(x, y)) {
    // Use land
}
```

### Iterating Over Terrain

```rust
for ((x, y), land) in &world.terrain {
    // Process each land
}
```

### Checking Neighbors

```rust
let neighbors = [
    world.terrain.get(&(x - 1, y)), // West
    world.terrain.get(&(x + 1, y)), // East
    world.terrain.get(&(x, y - 1)), // North
    world.terrain.get(&(x, y + 1)), // South
];
```

### Error Handling

Most functions return `Result` types. Use `?` operator or `match`:

```rust
match save_world(&world) {
    Ok(()) => println!("Saved!"),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Dependencies

- **noise**: Perlin noise generation (`noise::Perlin`)
- **serde**: Serialization framework (`Serialize`, `Deserialize` traits)
- **serde_json**: JSON serialization
- **rand**: Not directly used, but noise crate may use it
- **macroquad**: Graphics rendering backend (immediate-mode, lightweight)

---

## Performance Considerations

- **Generation**: O(n) where n = number of lands generated
- **Memory**: ~64 tiles per land, each tile has Vec<Object>
- **Serialization**: Entire world loaded/saved at once (consider streaming for very large worlds)
- **Lookup**: HashMap access is O(1) average case

---

## Future Improvements

Potential enhancements (not yet implemented):

1. **Caching**: Cache generated lands to avoid regeneration
2. **Streaming**: Stream large worlds instead of loading all at once
3. **Compression**: Compress saved worlds
4. **Multi-threading**: Parallel generation for large regions
5. **Biome Transitions**: Smooth transitions at land boundaries
6. **Climate Zones**: Temperature/precipitation affecting biomes
7. **Structures**: Villages, dungeons, etc.
8. **Resources**: Mineable resources, harvestable plants

---

## Notes for LLMs

When modifying this codebase:

1. **Maintain Determinism**: Same seed must produce same world
2. **Preserve Neighbor Logic**: Changes to generation should consider neighbor awareness
3. **Update Tests**: Add tests for new features
4. **Follow Patterns**: Use existing patterns for consistency
5. **Document Changes**: Update this file if architecture changes significantly
6. **Check Serialization**: Ensure new types are serializable if they're part of World
7. **Consider Performance**: Large worlds can be memory-intensive
8. **Update Commit Hash Before Committing**: When the Director (user) asks for a commit, update the commit hash in this file (at the top) to reference the previous commit hash BEFORE making the commit. This ensures the documentation references the commit that existed before the changes being committed.

---

## Quick Reference

**File Locations**:
- Types: `src/types.rs`
- Generation: `src/generation.rs`
- I/O: `src/io.rs`
- Display: `src/display.rs`
- Terrain View: `src/terrain_view.rs`
- Land View: `src/land_view.rs`
- Graphics Loop: `src/graphics_loop.rs`
- Renderer: `src/render/mod.rs`, `src/render/macroquad.rs`
- Tests: `src/tests.rs`, `tests/integration_tests.rs`

**Key Constants**:
- Tile grid size: 8x8
- Initial generation: -10 to 10 (441 lands)
- Noise scale: 0.1 for biomes, 0.5 for tiles
- Uniformity factor: `matching_neighbors * 0.2`
- Terrain view tile size: 48px
- Land view tile size: 64px
- Camera follow speed: 8.0

**Common Seeds**:
- Test seed: `12347`
- Demo seed: `12345`
