# MVP - Minimum Viable Product

> **Last Updated**: 2026-02-04  
> **Previous Commit**: `b95d715`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

---

## Overview

The MVP (Minimum Viable Product) is a simplified version of the Q game that combines movement/terrain, crafting, and combat systems into a playable game. The goal is to create a small 5x5 grid where the player starts at the top-left, defeats enemies to unlock lands, crafts gear to get stronger, and eventually defeats the final boss at the bottom-right.

**Current Status:**
- ✅ Step 1 Complete: Movement/Terrain Foundation with Web Interface
- ✅ Step 2 Complete: Combat Obstacle System with GUI
- ✅ CLI/REPL Interface for E2E Testing
- 🔄 Step 3 In Progress: Crafting System (inventory foundation complete)

---

# Notes For LLMs

**Parent Project**: When the user mentions the "parent project," they mean "Q" located at `../../{src,README.md}` relative to this subsystem.

When modifying this subsystem:

1. **Update Tests**: Add e2e test scripts in `tests/` for new features
2. **Update This README**: Keep documentation current with changes
3. **Update Commit Hash Before Committing**: When asked to commit, update the "Previous Commit" hash at the top of this file to reference the commit that existed BEFORE the changes being committed

### Commit Workflow

When the user says "commit":
1. Check `git log --oneline -1` for current commit hash
2. Update "Previous Commit" in this README to that hash
3. Update "Last Updated" date if significant changes
4. Stage changes and commit with descriptive message

---

# Code Design

## Backend

The backend is structured around a single source of truth: `GameState`. All game logic operates on this mutable state, which contains the world, character, and current view mode.

### Full Game State and Representation

**Core Structure:**
```rust
pub struct GameState {
    pub world: World,                    // Complete 5x5 world with all lands
    pub current_mode: CurrentMode,       // Terrain, Land, or Combat
    pub terrain_camera: TerrainCamera,   // Camera for terrain view
    pub land_camera: LandCamera,         // Camera for land view
    pub character: Character,            // Player character (source of truth for position)
    pub combat_round: u32,              // Combat round counter
}

pub enum CurrentMode {
    Terrain,  // Viewing biome overview (5x5 grid of lands)
    Land,     // Viewing detailed tile grid (8x8 tiles within a land)
    Combat,   // In combat with enemy
}
```

**Data Model:**
- `World` - Contains hashmap of lands indexed by (x, y) coordinates (0-4 range)
- `Land` - Contains 8x8 grid of tiles, biome info for 9 zones, optional enemy
- `Tile` - Contains substrate and list of objects
- `Character` - Tracks health, attack, land position, tile position, inventory
- `Enemy` - Health, attack, max health, defeated status

**Key Design Principles:**
1. **Character is source of truth** for position (cameras follow character)
2. **Coordinate clamping** enforced at boundaries (0-4 lands, 0-7 tiles)
3. **State transitions** handled explicitly (terrain ↔ land ↔ combat)
4. **Health persistence** - character health persists across battles, enemies reset

### Crafting Registry

Not yet implemented in MVP. Planned structure:
- Recipe system with input/output items
- Crafting UI for combining items
- Item drops from defeated enemies
- Object interaction for resource gathering

## Frontend Interface

### Passing Game State

The backend serializes game state into a discriminated union sent to the frontend:

```rust
pub struct GameStateResponse {
    pub core_state: CoreGameState,      // Discriminated union
    pub character: SerializableCharacter, // Always included
}

pub enum CoreGameState {
    Terrain(TerrainGameState),  // All lands with biomes + enemy info
    Land(LandGameState),        // Current land's 8x8 tile grid
    Combat(CombatGameState),    // Player + enemy combat stats
}
```

**API Endpoints:**
- `GET /api/state` - Returns current game state
- `POST /api/command` - Executes command, returns updated state

This design ensures:
- Frontend only receives data relevant to current view
- Type safety through discriminated unions
- Single source of truth (backend owns all state)
- Stateless frontend (can reconstruct from response)

### Issuing Commands

Commands are issued as simple string commands through a unified interface:

**Command Processing:**
1. Frontend/CLI sends command string (e.g., "u", "attack", "status")
2. Backend executes via `execute_command(&mut state, &str) -> (bool, String)`
3. Returns (success, message) tuple
4. Frontend updates based on new state from `/api/state`

**Command Interface Modes:**
- **Web**: HTTP POST to `/api/command` with JSON `{"command": "u"}`
- **CLI REPL**: Interactive stdin loop calling `execute_command` directly
- **Script**: File with one command per line, executed sequentially

This unified interface enables:
- Complete e2e testing via scripts
- Consistent behavior across interfaces
- Easy debugging and replay of game sessions

---

# Gameplay Components

## Terrain

The terrain view shows the world as a 5x5 grid of lands, where each land is represented by its center biome.

**Navigation:**
- Move between lands using `U`, `D`, `L`, `R` commands
- Coordinates clamped to (0,0) through (4,4) range
- Character position tracked at land level

**Visual Representation:**
- Each land displays as single colored cell based on center biome
- Enemy presence indicated by crossed-swords icon overlay
- Character shown as player icon with bold black outline

**Biomes:**
- Forest (green) - Trees and natural resources
- Desert (yellow) - Harsh, sparse environment
- Mountain (gray) - Rocky terrain with minerals
- Grassland (light green) - Open plains

## Land

Each land contains an 8x8 grid of tiles with varying biomes based on position within the land.

**Structure:**
- 9 biome zones: 4 corners, 4 edges, 1 center
- Each tile has substrate (grass, stone, sand, etc.)
- Tiles can contain objects (trees, rocks, sticks)

**Navigation:**
- Enter land with `E` command from terrain view
- Move within land using `U`, `D`, `L`, `R` commands
- Coordinates clamped to (0,0) through (7,7) range
- Exit back to terrain view with `E` command

**Objects:**
- Trees: Resource for crafting, represented as green pine-tree icon
- Rocks: Mining resource, represented as gray gem icon
- Sticks: Basic crafting material, represented as brown bowling-pin icon

## Crafting

**Current Status:** Foundation implemented, core mechanics in progress

**Implemented:**
- Inventory data structure (simple list, no stacking yet)
- Inventory display in UI (toggle with backtick key in web, `inv` in CLI)
- Item stat bonuses (e.g., stick grants +1 attack when equipped)
- Craftable query command to check recipes based on inventory + workstations

**Planned Features:**
- Item pickup system (`P` command to collect objects from tiles)
- Enemy drops (items awarded on combat victory)
- Crafting recipes (combine items to create equipment)
- Crafting GUI for item combination
- Object placement/interaction

## Combat

Combat is triggered when entering a land that contains an undefeated enemy.

**Combat Flow:**
1. Enter land with enemy → automatic combat mode transition
2. Simultaneous attack each round (both deal damage)
3. Combat continues until player or enemy reaches 0 health
4. Victory: Enemy defeated, enter land view to explore
5. Defeat: Return to terrain view, restore to half health
6. Draw (both die): Counts as player defeat

**Combat Commands:**
- `A`, `ATTACK` - Execute one combat round
- `E`, `ENTER` - Flee combat (return to terrain, health persists)

**Enemy Scaling:**
- Weak enemies near start (lower health/attack)
- Medium enemies in middle areas
- Strong enemies toward end
- Boss at position (4,4)

**Health Mechanics:**
- Player health persists across all battles
- Player restored to half health after death
- Enemies always start at full health when combat begins
- Enemies restored to full health if player flees

**Visual Display:**
- Side-by-side player/enemy panels
- Health bars for both combatants
- Round counter
- Combat log showing damage dealt

---

# Art

The game uses [RPG-Awesome](https://nagoshiashumari.github.io/Rpg-Awesome/) font icons for all visual assets.

**Asset Composition:**

**Character Sprites:**
- Base icons: `ra-player`, `ra-player-king`, `ra-player-pyromaniac`
- Style: Bold black outline effect for visibility
- Cycling: Press `C` to cycle through character appearances

**Objects:**
- Tree: `ra-pine-tree` (green, #2d5016)
- Rock: `ra-gem` (gray, #808080)
- Stick: `ra-bowling-pin` (brown, bold, scaled)

**Combat/Enemy Indicators:**
- Terrain view: `ra-crossed-swords` (indicates enemy presence)
- Combat view: `ra-monster-skull` (enemy sprite)

**UI Icons:**
- Death screen: `ra-skull`
- Victory screen: `ra-trophy`

**Color Scheme:**
- Biomes use natural colors (green forest, yellow desert, gray mountain)
- Objects colored to match their material type
- UI elements use contrasting colors for visibility

**Files Location:**
- Font files: `static/rpg-awesome/fonts/`
- CSS: `static/rpg-awesome/css/rpg-awesome.min.css`
- Served via `/static` route in web server

---

# Testing

## REPL-Based E2E Testing

**Critical Design Decision:** The REPL/command interface enables complete end-to-end testing of all game logic without requiring a browser or frontend.

**Why This Matters:**
- All gameplay can be represented as a sequence of commands
- Tests are deterministic and repeatable
- Backend logic fully testable in isolation
- Fast execution (no browser overhead)
- Easy debugging (commands are human-readable)

**⚠️ IMPORTANT FOR LLMs:**
When writing or running tests:
1. **ALWAYS use the CLI/script interface**: `cargo run script tests/test_name.txt`
2. **DO NOT test via web browser** - the REPL is the primary testing interface
3. **If the REPL is only accessible through the webpage and not through the command line, STOP AND TELL THE USER IMMEDIATELY** - this breaks the testing architecture

## Running Tests

**Run full test suite:**
```bash
./run_tests.sh
```

**Run individual test:**
```bash
cargo run script tests/e2e_basic_movement.txt
cargo run script tests/e2e_combat_victory.txt
cargo run script tests/e2e_edge_cases.txt
```

**Test Coverage:**
- `e2e_basic_movement.txt` - Movement mechanics and navigation
- `e2e_combat_victory.txt` - Combat system and victory conditions
- `e2e_combat_flee.txt` - Fleeing mechanics and health persistence
- `e2e_inventory_status.txt` - Information commands and displays
- `e2e_edge_cases.txt` - Boundary conditions and error handling
- `e2e_full_playthrough.txt` - Comprehensive integration test

See `tests/README_TESTS.md` for detailed test documentation.

## Writing Good Tests

**Test Script Format:**
```
# Comment explaining test purpose
command1
command2
status    # Verify state
command3
# More comments
```

**Best Practices:**
1. **Use comments** to explain what each section tests
2. **Include status checks** at key points to verify state
3. **Test edge cases** (boundaries, invalid input, error conditions)
4. **Keep tests focused** - one test per feature/scenario
5. **Name descriptively** - `e2e_<feature>_<scenario>.txt`

**Non-Trivial Test Characteristics:**
- Tests multiple related features together
- Verifies state persistence across mode transitions
- Includes both success and failure paths
- Tests boundary conditions thoroughly
- Validates error messages are helpful

---

# Project Structure

```
subsystems/mvp/
├── Cargo.toml                    # Rust dependencies and project config
├── README.md                     # This file
├── MVP_BRAINSTORM.md            # Original design document
├── INVENTORY_TEST.md            # Inventory testing guide
├── run_tests.sh                 # Automated test runner
├── static/
│   ├── index.html               # Web interface (single HTML file)
│   └── rpg-awesome/             # RPG-Awesome font icon library
│       ├── css/
│       │   └── rpg-awesome.min.css
│       └── fonts/
│           └── rpgawesome-webfont.*
├── tests/
│   ├── README_TESTS.md          # Test documentation
│   ├── e2e_basic_movement.txt
│   ├── e2e_combat_victory.txt
│   ├── e2e_combat_flee.txt
│   ├── e2e_inventory_status.txt
│   ├── e2e_edge_cases.txt
│   └── e2e_full_playthrough.txt
└── src/
    ├── main.rs                  # Entry point with CLI arg parsing
    ├── lib.rs                   # Module exports
    ├── cli.rs                   # CLI REPL and script execution
    ├── game/
    │   ├── mod.rs              # Game module exports
    │   ├── commands.rs         # Command execution (shared by web/CLI)
    │   ├── character.rs        # Character with stats and inventory
    │   ├── combat.rs           # Combat system and resolution
    │   ├── game_state.rs       # GameState and movement logic
    │   └── world/
    │       ├── mod.rs          # World module exports
    │       ├── types.rs        # Core data types (World, Land, Tile, etc.)
    │       ├── world.rs        # Hardcoded world generation
    │       ├── camera.rs       # CameraCore base implementation
    │       ├── terrain_view.rs # TerrainCamera (land-level view)
    │       └── land_view.rs    # LandCamera (tile-level view)
    └── web/
        ├── mod.rs              # Web server, API, and HTTP handlers
        ├── types.rs            # Serialization types and API models
        ├── state_builder.rs    # Game state to API response builders
        ├── serialization.rs    # Item serialization helpers
        └── display.rs          # Text-based display utilities
```

**Key Files:**
- `game_state.rs` - Core game logic, movement, mode transitions
- `commands.rs` - Unified command processing for all interfaces
- `web/mod.rs` - HTTP server and routing
- `web/types.rs` - API request/response models
- `web/state_builder.rs` - Game state to JSON converters
- `cli.rs` - REPL and script execution
- `main.rs` - Argument parsing, mode selection

---

# Usage

## Building and Running

```bash
cd subsystems/mvp
cargo build
cargo run              # Web server (default)
cargo run cli          # Interactive CLI REPL
cargo run script <file>  # Execute script file
cargo run -- --help    # Show all options
```

**Modes:**
- **Web Mode (default):** Server at `http://127.0.0.1:3000` with visual UI
- **CLI Mode:** Interactive REPL for command-line play (type `quit` to exit)
- **Script Mode:** Execute commands from text file for testing/automation

## Commands

**Movement:**
- `M <direction>`, `MOVE <direction>` - Move in direction (e.g., `m u` for up, `move down` for down)
  - Directions: `u`/`up`, `d`/`down`, `l`/`left`, `r`/`right`
- `X`, `ENTER`, `EXIT` - Context-dependent: Enter/Exit land, Flee combat

**Combat:**
- `A`, `ATTACK` - Attack enemy (execute one combat round)
- `X`, `EXIT` - Flee combat (return to terrain view)

**Equipment:**
- `E <index>`, `EQUIP <index>` - Equip item from inventory (e.g., `e 0` to equip first item)
- `UNEQUIP` - Unequip current item

**Information:**
- `STATUS`, `STATS`, `S` - Show character health, attack, position, mode
- `INV`, `INVENTORY`, `I` - Show inventory contents (CLI only)
- `H`, `HELP`, `?` - Show context-aware help
- `RECIPES` - List all crafting recipes
- `CRAFTABLE`, `CAN`, `AVAILABLE` - Show recipes that can be crafted now

**Crafting:**
- `CRAFT <recipe>`, `C <recipe>` - Craft item from recipe (e.g., `craft knap_flint_blade`)
- `PICKUP`, `P` - Pick up item from current tile (only pickupable items)
- `DROP`, `D` - Drop first item from inventory
- `PLACE <index>`, `L <index>` - Place item as world object (e.g., `l 0` to place forge)

**Web-Only:**
- `` ` `` (backtick) - Toggle inventory overlay

---

# Planned Features

**Step 3: Crafting System** (In Progress)
- ✅ Inventory data structure
- ✅ Inventory UI display
- 🔄 Item pickup system (`P` command)
- 🔄 Mob drops on combat victory
- 🔄 Crafting recipes and GUI
- 🔄 Object placement and interaction

**Step 4: Equipment System**
- Equippable items (weapon, armor, accessories)
- Equipment slots in character
- Stat bonuses from equipment
- Equipment display in UI

**Step 5: Content and Progression**
- More varied enemies and biomes
- Larger world (beyond 5x5)
- Quest/objective system
- Progression mechanics
- Save/load system

**Future Enhancements:**
- Procedural world generation
- More crafting recipes
- Advanced combat mechanics
- Multiplayer support

---

## Dependencies

- `serde` (1.0) - Serialization with derive macros
- `serde_json` (1.0) - JSON serialization for API
- `axum` (0.7) - Web framework for HTTP server
- `tokio` (1.x) - Async runtime (full features)
- `tower-http` (0.5) - Static file serving and middleware
- `clap` (4.5) - Command-line argument parsing with derive

---

For implementation details and design rationale, see `MVP_BRAINSTORM.md`.
