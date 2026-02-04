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

---

## Pending Items

### Refactor Movement Commands

**Backend Changes:**
- Replaced individual movement keys (U/D/L/R) with unified `M <direction>` command
- Commands: `m u`, `m d`, `m l`, `m r` (or `move up`, `move down`, etc.)
- Old single-letter commands (u/d/l/r) now show helpful error directing to new syntax
- All CLI tests updated to use new syntax

**Frontend Tasks:**
1. Remove arrow key bindings for direct movement (ArrowUp, ArrowDown, ArrowLeft, ArrowRight)
2. Arrow keys should no longer trigger automatic commands
3. Update all help text to remove references to arrow keys / U/D/L/R movement
4. Users must type commands in the input box (e.g., "m u" to move up)
5. Update command placeholders to show "m <dir>" instead of "U/D/L/R"

**Rationale:**
- Frees up keyboard real estate (U/D/L/R keys no longer reserved)
- Consistent command-based interface (everything goes through text input)
- Simpler to explain: one movement command instead of four

**Testing:**
- CLI: Type `m u` should move up ✅
- CLI: Type `u` alone should show error message directing to `m u` ✅
- Web: Arrow keys should NOT automatically move character
- Web: Must type "m u" or "move up" in command input

---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date
5. Update "Current Status" when all tasks are complete
