# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

⚠️ Pending frontend changes - see "Pending Items" below

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)

---

## Pending Items

### Remove Character Cycling References

**Backend Changes:**
- Added standalone `c` command handler that shows craft usage message
- No backend character cycling code exists (was never implemented)

**Frontend Tasks:**
1. Remove all "Cycle character appearance" references from help text (3 locations in dynamic help content)
2. Remove any character cycling keybindings or handlers if they exist
3. The `C` key should not do anything in the frontend (crafting is CLI-only, uses text input)

**Details:**
- Search for "cycle" and "character appearance" in `static/index.html`
- These appear in the help content for Combat, Land, and Terrain views
- Simply delete those lines from the help text

**Testing:**
- CLI: Type `c` alone should show usage message
- CLI: Type `c knap_flint_blade` should work for crafting (when items available)
- Web: `C` key should have no effect

---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date
5. Update "Current Status" when all tasks are complete
