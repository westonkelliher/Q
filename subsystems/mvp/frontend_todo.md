# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

All frontend changes are up to date as of commit `de52097`.

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)
- ✅ Interactive Equip Selector: Type 'E' to open visual selector, use arrow keys to navigate, Enter to equip (2026-02-04)
- ✅ Frontend Code Split: Separated index.html into index.html (103 lines), style.css (764 lines), app.js (1019 lines) for better maintainability (2026-02-04)

---

## Pending Items

_No pending frontend integration tasks at this time._

---

## Notes for Frontend Implementor

When implementing changes:
1. Check the "Pending Items" section for new tasks
2. Review API response structures in `src/web/mod.rs` 
3. Test commands via CLI first to understand expected behavior
4. Move completed items to "Completed Items" section with date
5. Update "Current Status" when all tasks are complete
