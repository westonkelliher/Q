# MVP - Minimum Viable Product

> **Last Updated**: 2026-02-02  
> **Previous Commit**: `2f0398c`  
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

**Step 1 Complete**: Movement/Terrain Foundation (No UI)

The project currently has:
- ✅ Hardcoded 5x5 world with lands at coordinates (0,0) through (4,4)
- ✅ Core data structures (World, Land, Tile, Biome, Substrate, Object)
- ✅ Camera systems (TerrainCamera, LandCamera) for managing views
- ✅ Game state management with movement between lands and within lands
- ✅ Coordinate clamping (0-4 for lands, 0-7 for tiles)
- ✅ View switching between terrain view and land view
- ✅ Text-based display utilities for testing

**Not Yet Implemented**:
- ❌ REPL/CLI interface for user interaction
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
└── src/
    ├── main.rs          # Test/demo program
    ├── lib.rs           # Module exports
    ├── types.rs         # Core data types
    ├── camera.rs        # CameraCore for view management
    ├── terrain_view.rs  # TerrainCamera (land-level view)
    ├── land_view.rs     # LandCamera (tile-level view)
    ├── world.rs         # Hardcoded world generation
    ├── game_state.rs    # GameState and movement logic
    └── display.rs       # Text-based display utilities
```

## Usage

### Building and Running

```bash
cd subsystems/mvp
cargo build
cargo run
```

The current `main.rs` includes test code that demonstrates:
- World creation
- Movement between lands (terrain view)
- Movement within lands (land view)
- View switching
- Coordinate clamping

### Testing Movement

The test program verifies:
- Terrain movement with coordinate clamping (0-4)
- Land movement with coordinate clamping (0-7)
- Entering/exiting land view
- World structure and land details

## Next Steps

See `MVP_BRAINSTORM.md` for the complete implementation plan. The next steps are:

1. **Step 2**: Add combat obstacle system
2. **Step 3**: Add crafting system (gathering, inventory, crafting, workstations)
3. **Step 4**: Add equipment system
4. **Step 5**: Add content and progression system

## Dependencies

- `serde` with derive feature - Serialization support
- `serde_json` - JSON serialization (for potential future save/load)

No graphics dependencies yet - the UI/REPL will be added in a future step.
