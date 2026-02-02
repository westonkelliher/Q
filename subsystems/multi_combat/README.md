# Multi-Combat Subsystem

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `aa1ad06`  
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

A multi-combatant combat system with simultaneous attack resolution, similar to Super Auto Pets combat mechanics. Each side can have multiple combatants (pets) that attack in formation order. The system supports both interactive REPL mode (with in-memory state), one-shot command-line execution, and a graphical user interface.

This subsystem expands on the base combat subsystem to support team-based combat where multiple combatants fight on each side, making it more similar to Super Auto Pets gameplay.

## Features

- **Multi-Combatant Support**: Each side can have multiple combatants (pets) arranged in formation order
- **Front-to-Back Combat**: Combatants attack in order, with each combatant targeting the front-most enemy
- **Simultaneous Resolution**: All attacks resolve simultaneously each round
- **Automatic Formation Management**: Defeated combatants are removed, remaining combatants shift forward
- **Interactive REPL Mode**: Start the program and run multiple commands in a single session with persistent in-memory state
- **State Management**: Set up teams of combatants for each side and run multiple combats without re-entering stats
- **Predefined Combatants**: Static constants for common combatant archetypes (Tank, Glass Cannon, Balanced, etc.)
- **One-shot Commands**: Direct combat simulation without interactive mode
- **Graphical User Interface**: Super Auto Pets-style visual combat simulator with cute pet sprites, health bars, round-by-round execution, and auto-play mode

## File Structure

```
src/
├── main.rs    # CLI entry point with interactive REPL and one-shot commands
├── gui.rs     # GUI application using macroquad
└── lib.rs     # Core combat logic and predefined combatants
```

## Usage

### GUI Mode

Run the GUI application:

```bash
cargo run --bin combat-gui
```

**GUI Features:**
- **Super Auto Pets Style**: Cute, colorful pet sprites with rounded UI elements matching the Super Auto Pets aesthetic
- **Multi-Pet Display**: Show multiple pets per side arranged in formation order
- **Visual Pet Sprites**: Each preset combatant has a unique color-coded pet sprite with eyes and expressions
- **Team Management**: Add/remove combatants to build teams for each side
- **Combatant Selection**: Choose from predefined combatant types (color-coded) or create custom stats
- **Visual Health Bars**: Real-time health visualization with color-coded status (green/yellow/red) displayed below each pet
- **Attack Indicators**: Red attack badges showing each pet's attack value
- **Formation Visualization**: Clear indication of front-to-back order
- **Round-by-Round Control**: Execute combat rounds manually or use auto-play mode
- **Attack Animations**: Visual feedback when pets attack (shaking animation)
- **Defeat Animation**: Defeated pets show X eyes and darker colors, then disappear
- **Interactive Input**: Click on health/attack fields to enter custom values using keyboard

**Controls:**
- Click preset buttons to add predefined combatants to teams
- Toggle "Custom: ON/OFF" to switch between presets and custom stats
- Click on health/attack input fields to edit values (use number keys and backspace)
- "Add to Side 1/2" - Add a combatant to the selected side
- "Remove from Side 1/2" - Remove the last combatant from a side
- "Clear Side 1/2" - Remove all combatants from a side
- "Start Combat" - Begin a new combat simulation
- "Next Round" - Execute one round of combat
- "Auto Play" - Automatically execute rounds (1 second per round)
- "Reset" - Clear current combat state
- ESC - Exit the application

### CLI Interactive Mode (Default)

Run without arguments to start interactive mode:

```bash
cargo run
```

Or explicitly:

```bash
cargo run -- interactive
```

**Available Commands:**

- `add-side1 <health> <attack>` (alias: `add1`) - Add a combatant to side 1
- `add-side2 <health> <attack>` (alias: `add2`) - Add a combatant to side 2
- `clear-side1` (alias: `clear1`) - Clear all combatants from side 1
- `clear-side2` (alias: `clear2`) - Clear all combatants from side 2
- `show` (aliases: `status`, `s`) - Display current team states
- `fight` (aliases: `go`, `rip`, `f`) - Run combat with saved teams
- `help` (alias: `h`) - Show help message
- `quit` (aliases: `exit`, `q`) - Exit the program

**Example Session:**

```
combat> add-side1 10 5
Added to Side 1: HP=10, ATK=5 (Position 1)
combat> add-side1 8 3
Added to Side 1: HP=8, ATK=3 (Position 2)
combat> add-side2 12 4
Added to Side 2: HP=12, ATK=4 (Position 1)
combat> show
Current Combat State:
  Side 1: [HP=10, ATK=5] [HP=8, ATK=3]
  Side 2: [HP=12, ATK=4]
combat> fight
Combat Result: Side 1 Wins
Total Rounds: 3
...
combat> quit
```

### One-shot Commands

For direct combat simulation without interactive mode:

```bash
# Full combat simulation (multiple combatants per side)
# Format: side1_health1 side1_attack1 [side1_health2 side1_attack2 ...] -- side2_health1 side2_attack1 [side2_health2 side2_attack2 ...]
cargo run -- combat 10 5 8 3 -- 12 4
cargo run -- combat 10 5 8 3 6 2 -- 12 4 9 3

# Single round execution
cargo run -- combat-round 10 5 8 3 -- 12 4
```

## Predefined Combatants

The library provides static combatant constants for common archetypes:

- `Combatant::TANK` - 20 HP, 2 ATK (high health, low attack)
- `Combatant::GLASS_CANNON` - 5 HP, 8 ATK (low health, high attack)
- `Combatant::BALANCED` - 10 HP, 5 ATK (balanced stats)
- `Combatant::BRUISER` - 15 HP, 6 ATK (high health, medium attack)
- `Combatant::ASSASSIN` - 3 HP, 10 ATK (very low health, very high attack)
- `Combatant::DEFENDER` - 25 HP, 1 ATK (very high health, very low attack)

## Combat Mechanics

- **Formation Order**: Combatants are arranged in order on each side. The first combatant in each team is the "front" position
- **Front-to-Back Attacking**: Each combatant attacks the front-most enemy combatant (position 0 on the opposing side)
- **Simultaneous Attacks**: All combatants attack at the same time each round
- **Round Resolution**: 
  1. All combatants deal damage simultaneously to their targets
  2. Defeated combatants (health <= 0) are removed from their teams
  3. Remaining combatants shift forward to fill gaps
  4. Victory conditions are checked
- **Victory Conditions**:
  - Side 1 wins if all Side 2 combatants are defeated
  - Side 2 wins if all Side 1 combatants are defeated
  - Draw if all combatants on both sides are defeated simultaneously
  - Ongoing if both sides still have living combatants
- **Target Selection**: Each combatant always targets the front-most enemy (index 0). If the front enemy is defeated, the next combatant becomes the new target for subsequent rounds

## API

### Core Types

- `Combatant`: Represents a combatant with health and attack stats
- `CombatState`: Manages the state of combat between two teams of combatants
- `CombatResult`: Enum representing the outcome of combat (Ongoing, Side1Wins, Side2Wins, Draw)

### Key Methods

- `Combatant::new(health, attack)` - Create a new combatant
- `CombatState::new(side1: Vec<Combatant>, side2: Vec<Combatant>)` - Create a new combat state with teams
- `CombatState::execute_round()` - Execute one round of combat (all combatants attack simultaneously)
- `CombatState::simulate_combat()` - Simulate combat to completion
- `CombatState::get_front_combatant(side: usize)` - Get the front-most combatant for a side (returns Option)
- `CombatState::remove_defeated()` - Remove all defeated combatants and shift remaining forward

### Example: Multi-Combatant Combat

```rust
use combat::{Combatant, CombatState};

// Create teams
let side1 = vec![
    Combatant::new(10, 5),  // Front position
    Combatant::new(8, 3),   // Back position
];
let side2 = vec![
    Combatant::new(12, 4),  // Front position
];

let mut state = CombatState::new(side1, side2);

// Execute rounds until completion
let (final_state, result) = state.simulate_combat();
```
