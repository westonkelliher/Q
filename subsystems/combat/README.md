# Combat Subsystem

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `cdf4bd6`  
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

A simple one-v-one combat system with simultaneous attack resolution, similar to Super Auto Pets combat mechanics. The system supports both interactive REPL mode (with in-memory state) and one-shot command-line execution.

## Features

- **Interactive REPL Mode**: Start the program and run multiple commands in a single session with persistent in-memory state
- **State Management**: Set up combatants for each side and run multiple combats without re-entering stats
- **Predefined Combatants**: Static constants for common combatant archetypes (Tank, Glass Cannon, Balanced, etc.)
- **One-shot Commands**: Direct combat simulation without interactive mode

## File Structure

```
src/
├── main.rs    # CLI entry point with interactive REPL and one-shot commands
└── lib.rs     # Core combat logic and predefined combatants
```

## Usage

### Interactive Mode (Default)

Run without arguments to start interactive mode:

```bash
cargo run
```

Or explicitly:

```bash
cargo run -- interactive
```

**Available Commands:**

- `set-side1 <health> <attack>` (alias: `side1`) - Set combatant 1 stats
- `set-side2 <health> <attack>` (alias: `side2`) - Set combatant 2 stats
- `show` (aliases: `status`, `s`) - Display current combatant states
- `fight` (aliases: `go`, `rip`, `f`) - Run combat with saved combatants
- `help` (alias: `h`) - Show help message
- `quit` (aliases: `exit`, `q`) - Exit the program

**Example Session:**

```
combat> set-side1 10 5
Side 1 set: HP=10, ATK=5
combat> set-side2 8 3
Side 2 set: HP=8, ATK=3
combat> show
Current Combat State:
  Side 1: HP=10, ATK=5
  Side 2: HP=8, ATK=3
combat> fight
Combat Result: Combatant 1 Wins
Total Rounds: 2
...
combat> quit
```

### One-shot Commands

For direct combat simulation without interactive mode:

```bash
# Full combat simulation
cargo run -- combat <health1> <attack1> <health2> <attack2>
cargo run -- combat 10 5 8 3

# Single round execution
cargo run -- combat-round <health1> <attack1> <health2> <attack2>
cargo run -- combat-round 10 5 8 3
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

- **Simultaneous Attacks**: Both combatants attack at the same time each round
- **Round Resolution**: Damage is applied simultaneously, then victory conditions are checked
- **Victory Conditions**:
  - Combatant 1 wins if only Combatant 2 is defeated
  - Combatant 2 wins if only Combatant 1 is defeated
  - Draw if both are defeated simultaneously
  - Ongoing if both are still alive

## API

### Core Types

- `Combatant`: Represents a combatant with health and attack stats
- `CombatState`: Manages the state of combat between two combatants
- `CombatResult`: Enum representing the outcome of combat (Ongoing, Combatant1Wins, Combatant2Wins, Draw)

### Key Methods

- `Combatant::new(health, attack)` - Create a new combatant
- `CombatState::new(combatant1, combatant2)` - Create a new combat state
- `CombatState::execute_round()` - Execute one round of combat
- `CombatState::simulate_combat()` - Simulate combat to completion
