# Test Scripts

This directory contains example command scripts for testing the crafting system CLI.

## Running Scripts

Execute a script by piping it to the CLI:

```bash
cargo run < test_scripts/basic_crafting.txt
```

Or using the compiled binary:

```bash
./target/debug/crafting < test_scripts/basic_crafting.txt
```

## Available Scripts

### basic_crafting.txt
Tests the fundamental crafting flow: creating raw materials and smelting them into bars.
- Creates copper ore and tin ore
- Crafts bronze bar
- Shows instance details and provenance

### multi_component.txt
Tests crafting items with multiple component slots (like Tinker's Construct).
- Creates materials for pickaxe components
- Crafts pickaxes with different material combinations
- Demonstrates component tracking

### provenance_chain.txt
Tests deep provenance tracking through multiple crafting stages.
- Creates raw ores
- Smelts bars
- Crafts tools from bars
- Traces full crafting history

### quality_test.txt
Tests quality tier system and quality propagation.
- Creates materials at different quality levels
- Tests quality inheritance in crafted items
- Compares results across quality tiers

### exploration.txt
General exploration script for manual testing.
- Lists all items and recipes
- Shows example item definitions
- Creates basic materials for experimentation

## Output Format

All commands output JSON for easy parsing:

```json
{
  "status": "success",
  "data": { ... }
}
```

Or on error:

```json
{
  "status": "error",
  "message": "error description"
}
```

## Automated Testing

These scripts can be used in automated tests:

```bash
# Run and capture output
OUTPUT=$(cargo run < test_scripts/basic_crafting.txt)

# Parse JSON output with jq
echo "$OUTPUT" | jq '.status'
```

## Creating New Scripts

Scripts support:
- Comments (lines starting with `#`)
- Blank lines (ignored)
- All CLI commands

Example:

```
# Create materials
new copper_ore common

# Craft something
craft make_bronze_bar 0 1

# Check result
show instance 2
```
