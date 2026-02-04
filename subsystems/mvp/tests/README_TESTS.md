# E2E Test Scripts

This directory contains end-to-end test scripts for the MVP game that can be executed using the CLI script mode.

## Running Tests

Execute any test script with:

```bash
cargo run script tests/<test_name>.txt
```

## Test Suite

### 1. `e2e_basic_movement.txt`
Tests fundamental movement mechanics:
- Terrain-level movement (between lands)
- Land entry and exit
- Tile-level movement (within a land)
- Position tracking and status display

**Expected**: All movements succeed, coordinates update correctly, mode transitions work.

### 2. `e2e_combat_victory.txt`
Tests combat mechanics and victory conditions:
- Entering combat when moving to enemy land
- Attack command execution
- Winning combat and transitioning to land view
- Post-combat land exploration

**Expected**: Player defeats weak enemy, enters land view, can explore the land.

### 3. `e2e_combat_flee.txt`
Tests fleeing from combat:
- Health damage persistence after fleeing
- Enemy health restoration after flee
- Re-entering combat with same enemy
- Multiple flee attempts

**Expected**: Player health persists, enemy resets to full health each combat.

### 4. `e2e_inventory_status.txt`
Tests information display commands:
- Status command and aliases (status, stats, s)
- Inventory command and aliases (inventory, inv, i)
- Help command and aliases (help, h, ?)
- Status display in different view modes

**Expected**: All commands display appropriate information for current context.

### 5. `e2e_edge_cases.txt`
Tests boundary conditions and error handling:
- Moving beyond terrain boundaries (coordinate clamping to 0-4)
- Moving beyond tile boundaries (coordinate clamping to 0-7)
- Invalid commands (should show error messages)
- Context-inappropriate commands (combat commands outside combat)

**Expected**: Boundaries enforced, invalid commands rejected with helpful errors.

### 6. `e2e_full_playthrough.txt`
Comprehensive integration test:
- Multiple combat encounters
- Mixed victory and flee scenarios
- Extensive exploration across lands
- Health management throughout session
- All command types in realistic sequence

**Expected**: Complete playthrough without crashes, state consistency maintained.

## Success Criteria

All tests should:
1. Execute without errors or panics
2. Complete all commands successfully (except intentional invalid commands)
3. Maintain consistent game state throughout
4. Show appropriate responses for each command

## Adding New Tests

When adding new test scripts:
1. Use `.txt` extension
2. Start with descriptive comment header
3. Use `#` for comments explaining test steps
4. Include status checks at key points
5. Document expected outcomes in this README
