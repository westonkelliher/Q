# Crafting System CLI Usage Guide

The crafting system now has a fully functional REPL-based CLI for testing and exploration.

## Quick Start

### Interactive Mode

```bash
cargo run
```

Or with human-readable output:

```bash
cargo run -- --human-readable
# or use the short form:
cargo run -- -h
```

Then type commands:
```
> help
> list items
> new copper_ore common
> show instance 0
> exit
```

### Script Mode

Run pre-written test scripts:

```bash
cargo run < test_scripts/basic_crafting.txt
```

With human-readable output:

```bash
cargo run -- --human-readable < test_scripts/basic_crafting.txt
```

## Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `list items` | List all item definitions | `list items` |
| `list recipes` | List all recipes | `list recipes` |
| `list instances` | List all item instances | `list instances` |
| `show item <id>` | Show detailed item definition | `show item copper_ore` |
| `show recipe <id>` | Show recipe requirements | `show recipe smelt_bronze_bar` |
| `show instance <id>` | Show instance with provenance | `show instance 0` |
| `new <item> [quality]` | Create raw material instance (defaults to common) | `new copper_ore` or `new copper_ore rare` |
| `craft <recipe> <ids...>` | Execute recipe | `craft smelt_bronze_bar 0 1 2` |
| `trace <id>` | Show full provenance tree | `trace 3` |
| `help` | Show all commands | `help` |
| `exit` / `quit` | Exit REPL | `exit` |

## Quality Levels

- `makeshift` - Lowest quality
- `crude` - Basic quality
- `common` - Standard quality
- `uncommon` - Above average
- `rare` - High quality
- `epic` - Very high quality
- `legendary` - Highest quality

## Output Formats

### JSON Output (Default)

By default, all commands return structured JSON:

**Success:**
```json
{
  "status": "success",
  "data": { ... }
}
```

**Error:**
```json
{
  "status": "error",
  "message": "error description"
}
```

### Human-Readable Output

Use the `--human-readable` flag (or `-h` for short) to get formatted, human-friendly output:

```bash
cargo run -- --human-readable
# or use the short form:
cargo run -- -h
```

**Example:**
```
Items (31):
  - Knife (knife) [Composite]
  - Copper Ore (copper_ore) [Simple]
  - Wolf Bone (wolf_bone) [Simple (Submaterial)]
  ...
```

The human-readable format is easier to read for interactive use, while JSON format is better for scripting and automation.

## Example Workflow

### Basic Smelting

```bash
# Create raw materials (defaults to common quality)
new copper_ore
new copper_ore
new tin_ore

# Smelt bronze bar (needs 2 copper + 1 tin)
craft smelt_bronze_bar 0 1 2

# Show the result
show instance 3

# Trace the crafting history
trace 3
```

### Multi-Component Crafting

```bash
# Create materials (quality defaults to common)
new bronze_bar
new oak_planks

# Craft pickaxe
craft craft_pickaxe 0 1

# Show the pickaxe with components
show instance 2
```

## Test Scripts

Pre-built test scripts in `test_scripts/`:

- `basic_crafting.txt` - Basic ore smelting workflow
- `multi_component.txt` - Complex items with components
- `provenance_chain.txt` - Deep crafting chains
- `quality_test.txt` - Quality tier testing
- `exploration.txt` - General exploration

Run any script:
```bash
cargo run < test_scripts/basic_crafting.txt
```

Parse JSON output:
```bash
cargo run < test_scripts/basic_crafting.txt | jq '.data'
```

## Testing

### Unit Tests

Test command parsing and core functionality:
```bash
cargo test --lib
```

### Integration Tests

Test end-to-end scenarios:
```bash
cargo test --test cli_tests
```

### All Tests

Run everything:
```bash
cargo test
```

Result: **39 tests pass** ✓

## Sample Content

The system comes pre-loaded with sample content:

- **9 item definitions** (copper_ore, tin_ore, bronze_bar, pickaxe, sword, cap, etc.)
- **7 recipes** (mining, smelting, crafting tools/weapons/armor)
- Multi-component items with slots (pickaxe: head + handle, sword: blade + handle + pommel)

## Tips

1. **Comments in scripts**: Lines starting with `#` are ignored
2. **Instance IDs**: Start at 0 and increment sequentially
3. **Recipe quantities**: Some recipes need multiple units (e.g., 2 copper_ore)
4. **JSON parsing**: Use `jq` for easy JSON filtering
5. **Error debugging**: Check the error message field for details

## Architecture

- **CLI Module** (`src/cli.rs`) - Command parsing and execution
- **Main Binary** (`src/main.rs`) - Initializes registry and runs REPL
- **Unit Tests** (`src/cli.rs`) - 21 command parsing tests
- **Integration Tests** (`tests/cli_tests.rs`) - 18 end-to-end scenarios
- **Test Scripts** (`test_scripts/`) - 5 example workflows

## Benefits

1. ✓ **Manual exploration** - Interactive REPL for hands-on testing
2. ✓ **Automated tests** - Script-based testing via stdin
3. ✓ **CI/CD friendly** - JSON output easy to parse
4. ✓ **Debugging** - Full provenance tracing
5. ✓ **Regression testing** - Reusable test scripts
