# Quick Start Guide

## Running the CLI

### Interactive Mode (Recommended)

Simply run:
```bash
cargo run
```

This starts an interactive REPL where you can type commands. The output is automatically formatted in human-readable mode.

### JSON Mode

If you prefer JSON output (useful for scripting):
```bash
cargo run -- --json
```

## Quick Examples

### Explore Items and Recipes

```bash
> list items          # See all available items
> list recipes        # See all recipes
> show item scimitar  # Get details about an item
> show recipe assemble_scimitar  # See recipe requirements
```

### Create Items

```bash
> new copper_ore      # Create a raw material
> inventory          # See your inventory (shows indices)
> show instance 0    # View the item you just created
```

### Craft Items

```bash
# First, create some materials
> new iron_ore
> new iron_ore

# Then craft something (use indices from inventory)
> craft smelt_iron_bar 0 1

# Check the result
> inventory
> show instance 2
```

### Combat System

```bash
# Simulate full combat: Combatant 1 (10 HP, 5 ATK) vs Combatant 2 (8 HP, 3 ATK)
> combat 10 5 8 3

# Execute just one round
> combat-round 10 5 8 3

# Try different scenarios
> combat 20 3 15 2    # Longer fight
> combat 5 5 5 5      # Draw scenario
```

## Common Commands

| Command | Shorthand | Description |
|---------|-----------|-------------|
| `help` | `h` or `?` | Show all commands |
| `inventory` | `i`, `inv`, `ls` | Show your inventory |
| `list items` | `li` | List all item definitions |
| `list recipes` | `lr` | List all recipes |
| `new <item_id>` | `n` | Create a new item |
| `combat <h1> <a1> <h2> <a2>` | `comb` | Simulate combat |
| `combat-round <h1> <a1> <h2> <a2>` | `cr` | Execute one round |
| `exit` | `q` | Exit the CLI |

## Tips

1. **Use shorthands**: Most commands have short forms (e.g., `lr` for `list recipes`)
2. **Check inventory first**: Use `inventory` to see item indices before crafting
3. **Combat is fun**: Try different stat combinations to see how combat resolves
4. **Type `help`**: Always available to see all commands

## Example Session

```
> list recipes
Recipes (15):
  [0] [smelt_iron_bar] Smelt Iron Bar [Simple] -> iron_bar x1
  [1] [craft_handle] Craft Handle [Component] -> handle
  ...

> new iron_ore
Created instance #0 for item: iron_ore

> new iron_ore
Created instance #1 for item: iron_ore

> inventory
Inventory (2):
  [0] Instance #0 (id: 0) [Simple] -> iron_ore
  [1] Instance #1 (id: 1) [Simple] -> iron_ore

> craft smelt_iron_bar 0 1
Crafted Simple instance #2 using recipe: smelt_iron_bar

> combat 10 5 8 3
Combat Result: Combatant 1 Wins
Total Rounds: 2
Combatant 1: HP=4, ATK=5
Combatant 2: HP=-2, ATK=3

Round History:
  Round 1: C1 10 -> 7, C2 8 -> 3
  Round 2: C1 7 -> 4, C2 3 -> -2

> exit
```

Enjoy exploring the crafting and combat systems!
