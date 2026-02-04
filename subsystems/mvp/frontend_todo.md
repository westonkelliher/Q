# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

✅ All backend changes implemented on frontend as of commit `1006319`.

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)
- ✅ Interactive Equip Selector: Type 'E' to open visual selector, use arrow keys to navigate, Enter to equip (2026-02-04)
- ✅ Frontend Code Split: Separated index.html into index.html (103 lines), style.css (764 lines), app.js (1019 lines) for better maintainability (2026-02-04)
- ✅ Movement Command Update: Arrow keys now send "m u/d/l/r" instead of "u/d/l/r", updated all help text and placeholders (2026-02-04)

---

## Pending Items

_No pending frontend integration tasks at this time._

## Future Enhancements

These backend features work in CLI but could have visual improvements in the web UI:

### Display World Objects (Crafting Stations)
- **Status**: Backend complete, API updates needed
- **What**: Show forge/workbench/anvil icons on tiles in Land view
- **Blocked by**: SerializableTile needs world_object field added to API
- **Implementation**: Once API updated, add world object graphics and render them on tiles

### Visual Indicator for Non-Pickupable Items  
- **Status**: Backend complete, frontend enhancement optional
- **What**: Show visual cue that trees can't be picked up directly
- **Note**: Backend already provides proper error messages, so not critical

---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date (one line per completed item)
5. Update "Current Status" when all tasks are complete
