# MVP - Minimum Viable Product

> **Last Updated**: 2026-02-04  
> **Previous Commit**: `438f6b1`  
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
**Step 2 Complete**: Combat Obstacle System with GUI

The project currently has:
- ✅ Hardcoded 5x5 world with lands at coordinates (0,0) through (4,4)
- ✅ Core data structures (World, Land, Tile, Biome, Substrate, Object, Enemy)
- ✅ Camera systems (TerrainCamera, LandCamera) for managing views
- ✅ Game state management with movement between lands and within lands
- ✅ Coordinate clamping (0-4 for lands, 0-7 for tiles)
- ✅ View switching between terrain view, combat view, and land view
- ✅ Text-based display utilities for testing
- ✅ Web interface with REPL-style command input
- ✅ Visual game display (terrain view, combat view, and land view)
- ✅ Command history and status display
- ✅ Character system with position tracking and stats (health, attack)
- ✅ RPG-Awesome font icons for all graphics (character sprites, objects, enemy indicators)
- ✅ Character icon rendering in terrain, land, and combat views (ra-player variants with bold black outline)
- ✅ Character appearance cycling with 'C' command (wizard → mage → person → wizard)
- ✅ Object icons: tree (ra-pine-tree, green), rock (ra-gem, gray), stick (ra-bowling-pin, brown)
- ✅ Enemy indicators: crossed-swords for terrain view, monster-skull for combat view
- ✅ Character stats display with health bar in web UI
- ✅ Camera follows character (character is source of truth for position)
- ✅ Combat system with simultaneous attack resolution
- ✅ Combat view mode that triggers when entering lands with enemies
- ✅ Combat GUI with side-by-side player/enemy panels, health bars, and round display
- ✅ Enemy system integrated into lands (enemies block land entry until defeated)
- ✅ Enemies with varying difficulty (weak early, medium mid, strong late, boss at 4,4)
- ✅ Combat commands (attack, flee) - character health persists after battles
- ✅ Death screen shown when player dies in combat (restores to half health on continue)
- ✅ Win screen shown when player wins combat (press Enter to continue)
- ✅ Death/win screens are non-stateful display overlays (don't persist across page refreshes)
- ✅ Simultaneous defeat (Draw) counts as a death for the player
- ✅ Character health persists across battles (not restored when fleeing)
- ✅ Character restores to half health when dying and continuing
- ✅ Enemies always start at full health when combat begins
- ✅ Unified 'E' command for enter/exit/flee (context-dependent)
- ✅ Enter key works anywhere on page to dismiss death/win screens

**Not Yet Implemented**:
- ❌ Crafting system integration
- ❌ Equipment system
- ❌ Content/progression system

**In Progress (Step 3 - Inventory)**:
- ✅ Inventory data structure (simple list, no stacking)
- ✅ Toggle inventory with backtick (`) key
- ✅ Inventory overlay UI display
- ✅ Inventory state is non-persistent (resets on page refresh)
- ⚠️ Item pickup system (not yet implemented)
- ⚠️ Mob drops (not yet implemented)

## Project Structure

```
subsystems/mvp/
├── Cargo.toml
├── README.md
├── MVP_BRAINSTORM.md    # Original brainstorm document
├── static/
│   ├── index.html       # Web interface frontend
│   └── rpg-awesome/     # RPG-Awesome font icon library (CSS and fonts)
└── src/
    ├── main.rs          # Web server entry point
    ├── lib.rs           # Module exports
    └── game/
    │   ├── mod.rs       # Game module exports
    │   ├── character.rs # Character system with stats and position
    │   ├── combat.rs    # Combat system with Combatant and CombatState
    │   ├── game_state.rs # GameState and movement logic
    │   ├── world/
    │   │   ├── mod.rs   # World module exports
    │   │   ├── camera.rs # CameraCore for view management
    │   │   ├── terrain_view.rs # TerrainCamera (land-level view)
    │   │   ├── land_view.rs # LandCamera (tile-level view)
    │   │   ├── types.rs  # Core data types
    │   │   └── world.rs  # Hardcoded world generation
    │   └── ...
    └── web/
        ├── mod.rs       # Web server and API
        └── display.rs   # Text-based display utilities
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

**Movement Commands:**
- `U`, `D`, `L`, `R` - Move up, down, left, right (in terrain or land view)
- `E` or `ENTER` - Context-dependent: Enter land (terrain view), Exit land (land view), or Flee combat (combat view)
- `C` or `CYCLE` - Cycle character appearance (wizard → mage → person → wizard)

**Combat Commands (when in combat):**
- `A` or `ATTACK` - Attack the enemy (executes one combat round)
- `E` or `ENTER` - Flee combat (returns to terrain view, character health persists)

**Inventory Commands:**
- `` ` `` (backtick) or `I` - Toggle inventory overlay (works in any view mode)

**General:**
- `H` or `HELP` - Show help (context-aware based on current view mode)

The web interface provides:
- Visual display of terrain (5x5 biome grid), combat (player vs enemy), and land (8x8 tile grid)
- Command input with keyboard support
- Command history sidebar
- Status bar showing current position and view mode
- Character stats display with health bar
- Combat view with side-by-side player/enemy panels, health bars, and round counter

## Next Steps

See `MVP_BRAINSTORM.md` for the complete implementation plan.

**Currently Working On:**
- **Step 3** (In Progress): Crafting system
  - ✅ Inventory foundation complete (see `INVENTORY_TEST.md` for testing guide)
  - 🔄 Next: Item pickup system (P command)
  - 🔄 Then: Mob drops, crafting GUI, object placement

**Future Steps:**
1. **Step 4**: Add equipment system
2. **Step 5**: Add content and progression system

## Dependencies

- `serde` with derive feature - Serialization support
- `serde_json` - JSON serialization (for API responses)
- `axum` - Web framework for HTTP server
- `tokio` - Async runtime
- `tower-http` - Static file serving and middleware

## Graphics

The game uses [RPG-Awesome](https://nagoshiashumari.github.io/Rpg-Awesome/) font icons for all graphics:
- **Character sprites**: RPG-Awesome player icons (`ra-player`, `ra-player-king`, `ra-player-pyromaniac`) with bold black outline effect
- **Objects**: 
  - Tree: `ra-pine-tree` (green)
  - Rock: `ra-gem` (gray)
  - Stick: `ra-bowling-pin` (brown, bold, scaled)
- **Enemy indicators**: 
  - Terrain view: `ra-crossed-swords` (indicates enemy presence on land)
  - Combat view: `ra-monster-skull` (enemy sprite)
- **UI icons**: Death screen (`ra-skull`), Win screen (`ra-trophy`)

RPG-Awesome files are located in `static/rpg-awesome/` and served via the `/static` route.



# Note
- when user mentions parent project they mean "Q" (../../{src,README.md})