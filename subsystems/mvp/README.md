# MVP - Minimum Viable Product

> **Last Updated**: 2026-02-02  
> **Previous Commit**: `81fd727`  
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

The MVP (Minimum Viable Product) is a simplified version of the Q game that combines movement/terrain, crafting, and combat systems into a playable game. The goal is to create a small 5x5 grid where the player starts at the top-left, defeats enemies to unlock lands, crafts gear to get stronger, and eventually defeats the final boss at the bottom-right.

## Current State

**Step 1 Complete**: Movement/Terrain Foundation with Web Interface

The project currently has:
- ✅ Hardcoded 5x5 world with lands at coordinates (0,0) through (4,4)
- ✅ Core data structures (World, Land, Tile, Biome, Substrate, Object)
- ✅ Camera systems (TerrainCamera, LandCamera) for managing views
- ✅ Game state management with movement between lands and within lands
- ✅ Coordinate clamping (0-4 for lands, 0-7 for tiles)
- ✅ View switching between terrain view and land view
- ✅ Text-based display utilities for testing
- ✅ Web interface with REPL-style command input
- ✅ Visual game display (terrain view and land view)
- ✅ Command history and status display

**Not Yet Implemented**:
- ❌ Combat system integration
- ❌ Crafting system integration
- ❌ Inventory system
- ❌ Equipment system
- ❌ Content/progression system

## Project Structure

```
subsystems/mvp/
├── Cargo.toml
├── README.md
├── MVP_BRAINSTORM.md    # Original brainstorm document
├── static/
│   └── index.html       # Web interface frontend
└── src/
    ├── main.rs          # Web server entry point
    ├── lib.rs           # Module exports
    ├── types.rs         # Core data types
    ├── camera.rs        # CameraCore for view management
    ├── terrain_view.rs  # TerrainCamera (land-level view)
    ├── land_view.rs     # LandCamera (tile-level view)
    ├── world.rs         # Hardcoded world generation
    ├── game_state.rs    # GameState and movement logic
    ├── display.rs       # Text-based display utilities
    └── web.rs           # Web server and API
```

## Usage

### Building and Running

```bash
cd subsystems/mvp
cargo build
cargo run
```

The web server will start on `http://127.0.0.1:3000`. Open this URL in your browser to play the game.

### Commands

- `U`, `D`, `L`, `R` - Move up, down, left, right
- `E` or `ENTER` - Enter land view
- `X` or `EXIT` - Exit land view
- `H` or `HELP` - Show help

The web interface provides:
- Visual display of terrain (5x5 biome grid) and land (8x8 tile grid)
- Command input with keyboard support
- Command history sidebar
- Status bar showing current position and view mode

## Next Steps

See `MVP_BRAINSTORM.md` for the complete implementation plan. The next steps are:

1. **Step 2**: Add combat obstacle system
2. **Step 3**: Add crafting system (gathering, inventory, crafting, workstations)
3. **Step 4**: Add equipment system
4. **Step 5**: Add content and progression system

## Dependencies

- `serde` with derive feature - Serialization support
- `serde_json` - JSON serialization (for API responses)
- `axum` - Web framework for HTTP server
- `tokio` - Async runtime
- `tower-http` - Static file serving and middleware
