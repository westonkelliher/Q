# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

⚠️ New backend features added - no frontend changes required (CLI-only commands)

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)
- ✅ Interactive Equip Selector: Type 'E' to open visual selector, use arrow keys to navigate, Enter to equip (2026-02-04)
- ✅ Frontend Code Split: Separated index.html into index.html (103 lines), style.css (764 lines), app.js (1019 lines) for better maintainability (2026-02-04)
- ✅ Movement Command Update: Arrow keys now send "m u/d/l/r" instead of "u/d/l/r", updated all help text and placeholders (2026-02-04)

---

## Pending Items

_No immediate frontend tasks required. Recent backend features are CLI-only._

## Recent Backend Features (No Frontend Action Needed)

**Craftable Query Command** (commit pending):
- Added `craftable` / `can` / `available` command
- Shows recipes that can be crafted with current inventory + workstations in land
- CLI-only, no frontend UI required

**Place Command & World Objects** (commits 80221a7, c7db0b5, 36110f1):
- Added `l <index>` / `place <index>` command to place items as world objects
- Added `d` alias for drop command
- Tile struct refactored: `world_object: Option<>`, `items: Vec<>`
- Added pickupable field - trees cannot be picked up
- CLI-only, future enhancement could display world objects in UI

## Future Enhancements



---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date (one line per completed item)
5. Update "Current Status" when all tasks are complete
