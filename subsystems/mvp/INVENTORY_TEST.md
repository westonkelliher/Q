# Inventory System Test Guide

## Overview

The inventory system has been implemented as the first step of Step 3 in the MVP brainstorming document. The inventory can be toggled open and closed using the backtick (`` ` ``) key.

## What's Implemented

### ✅ Completed Features

1. **Inventory Data Structure**
   - Simple list-based inventory (no stacking)
   - Infinite capacity
   - Located in `src/game/character.rs`

2. **Toggle Mechanism**
   - Press `` ` `` (backtick) or `I` to toggle the inventory
   - Works in any view mode (Terrain, Land, Combat)
   - State is non-persistent (resets on page refresh)

3. **UI Display**
   - Overlay that appears on top of the current view
   - Shows all items in a grid layout
   - Displays item count
   - Shows item icons using the graphics system
   - Instructions for closing the inventory

4. **Test Items**
   - Character starts with 3 test items:
     - 1x Stick
     - 1x Rock
     - 1x Tree

### ⚠️ Not Yet Implemented

1. **Item Pickup** - The `P` command to pick up items from tiles
2. **Item Dropping** - Placing items from inventory onto tiles
3. **Mob Drops** - Items dropping after defeating enemies
4. **Crafting Integration** - Using items for crafting

## How to Test

1. **Start the server:**
   ```bash
   cd subsystems/mvp
   cargo run
   ```

2. **Open your browser:**
   Navigate to `http://127.0.0.1:3000`

3. **Open the inventory:**
   - Press the `` ` `` (backtick) key on your keyboard
   - OR type `` ` `` in the command input and press Execute
   - OR type `I` in the command input and press Execute

4. **Expected Result:**
   - An inventory overlay should appear showing 3 items:
     - Stick (with bowling pin icon)
     - Rock (with gem icon)
     - Tree (with pine tree icon)
   - The overlay should have a dark background with a blue border
   - Instructions for closing should be shown at the bottom

5. **Close the inventory:**
   - Press `` ` `` again
   - OR press `I`
   - OR press `E` or `ENTER`

## Technical Details

### File Changes

1. **`src/game/character.rs`**
   - Added `Inventory` struct with methods
   - Added `inventory` field to `Character`
   - Character starts with test items

2. **`src/game/game_state.rs`**
   - Added `DisplayOverlay::Inventory` variant
   - Added `toggle_inventory()` method

3. **`src/web/mod.rs`**
   - Updated `SerializableCharacter` to include inventory
   - Added backtick command handler
   - Updated help text to mention inventory

4. **`static/index.html`**
   - Added inventory overlay CSS styles
   - Added `renderInventoryOverlay()` JavaScript function
   - Added backtick key event handler
   - Updated help section to show inventory commands

### Keyboard Shortcuts

The backtick key (`` ` ``) is handled globally and works in two ways:

1. **Direct keyboard event:** When focus is NOT on the command input, pressing `` ` `` sends the command immediately
2. **Command input:** You can type `` ` `` or `I` in the command box

Note: When typing in the command input, the key handler is skipped to allow normal typing.

## Next Steps

According to the MVP brainstorming document, the next features to implement are:

1. **Item Pickup (P command)**
   - Check if player is on tile with pickupable object
   - Add object to inventory
   - Remove object from tile

2. **Mob Drops**
   - Add items to inventory after combat victory
   - Or (stretch goal) place carcass on land that can be processed

3. **Crafting GUI**
   - Show available recipes on right side
   - Show inventory items on left side
   - Enable crafting when materials and workstations are available

4. **Object Placement**
   - Place workstations from inventory onto tiles
   - Enable crafting when near workstations
