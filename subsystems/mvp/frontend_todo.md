# Frontend Integration TODO

This file tracks backend changes that need corresponding frontend updates.

---

## Current Status

✅ All pending tasks complete

---

## Completed Items

- ✅ Keybinding Refactor: E for equip, X for enter/exit/flee, removed 'i' inventory toggle (2026-02-04, commit 90941d7)
- ✅ Interactive Equip Selector: Type 'E' to open visual selector, use arrow keys to navigate, Enter to equip (2026-02-04)
- ✅ Frontend Code Split: Separated index.html into index.html (103 lines), style.css (764 lines), app.js (1019 lines) for better maintainability (2026-02-04)
- ✅ Movement Command Update: Arrow keys now send "m u/d/l/r" instead of "u/d/l/r", updated all help text and placeholders (2026-02-04)
- ✅ Display Stat Bonuses with Breakdown: Attack display shows breakdown like "6 (5 + 1)" when equipped items grant bonuses (2026-02-04)

---

## Pending Items

_No pending frontend integration tasks at this time._

---

## Recently Completed

### Display Stat Bonuses with Breakdown

**Backend Changes (commit bb2b628):**
- Added StatBonuses struct to ItemDefinition (health and attack bonuses)
- Stick now grants +1 attack when equipped
- GameState::get_total_attack() calculates base + bonus
- API now returns total attack (base + bonuses)

**Frontend Implementation (2026-02-04):**
- Updated character stats display in updateStatus() function
- Shows "6 (5 + 1)" format when equipped items grant attack bonuses
- Shows "5" format when no bonuses active
- Base attack hardcoded as 5 for MVP

---

## Recent Backend Features (No Frontend Action Needed)

**Enemy Carcass Drops** (commit pending):
- Enemies now drop carcasses when defeated
- Carcass placed on center tile (4,4) after combat victory
- Added carcass items for all enemy types: rabbit, fox, wolf, spider, snake, lion, dragon
- Carcasses can be picked up and used in crafting recipes
- No frontend changes needed (automatic behavior)

**Craftable Query Command** (commit b95d715):
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
