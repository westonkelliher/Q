# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

⚠️ Pending frontend changes - see "Pending Items" below

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)
- ✅ Interactive Equip Selector: Type 'E' to open visual selector, use arrow keys to navigate, Enter to equip (2026-02-04)
- ✅ Frontend Code Split: Separated index.html into index.html (103 lines), style.css (764 lines), app.js (1019 lines) for better maintainability (2026-02-04)
- ✅ Movement Command Update: Arrow keys now send "m u/d/l/r" instead of "u/d/l/r", updated all help text and placeholders (2026-02-04)

---

## Pending Items

### Add Drop Alias and Pickupable Field (Backend Complete)

**Backend Changes:**
- Added 'd' as shorthand for 'drop' command
- Added pickupable field to ItemDefinition (bool)
- Trees marked as pickupable: false (cannot pick up directly)
- Pickup command now checks pickupable field
- Error message: "[Item] cannot be picked up. You may need to use a tool to harvest it."

**Frontend Tasks:**
- No immediate changes required
- Future: Could show visual indicator for non-pickupable items

**Testing:**
- CLI: 'd' command drops items ✅
- CLI: 'drop' command still works ✅
- CLI: Attempting to pick up tree shows error ✅
- New test: tests/e2e_drop_pickup.txt ✅

### Add Place Command Support (Backend Complete)

**Backend Changes:**
- Added `place <index>` / `l <index>` command to place items from inventory as world objects
- Added world_objects field to Tile struct (Vec<WorldObjectInstanceId>)
- Added placeable field to ItemDefinition (Option<WorldObjectKind>)
- Created three crafting station items: forge, workbench, anvil
- Added recipes to build crafting stations (build_forge, build_workbench, build_anvil)
- Placement only works in Land mode

**Frontend Tasks:**
- No immediate changes required (CLI-only command)
- Future enhancement: Display world objects (crafting stations) on tiles in Land view with icons

**Testing:**
- CLI: `l 0` places first inventory item if placeable ✅
- CLI: Error shown for non-placeable items ✅
- New test file: tests/e2e_placement.txt ✅

---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date (one line per completed item)
5. Update "Current Status" when all tasks are complete
