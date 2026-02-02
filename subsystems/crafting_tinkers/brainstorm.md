# Crafting Subsystem Brainstorm

## Core Concepts

### Recipe & Construction System
- **Recipe**: A blueprint for constructing an item
- **Construction**: The actual method used to create an item (each item has one construction)

**Construction Structure:**
```rust
Construction {
    tool: Option<Tool>,
    world_object: Option<WorldObject>, // Can be ResourceNode or CraftingStation
    // ... other fields
}
```

**WorldObject Types:**
- **ResourceNode**: Source of raw materials (e.g., iron ore boulder, tree)
- **CraftingStation**: Specialized workspace for crafting (e.g., anvil, magic imbuing station)

This allows for flexible construction types:
- Tool only (e.g., whittling with a knife)
- World object only (e.g., picking berries from a bush resource node)
- Both tool and world object (e.g., mining ore: pickaxe + ore node, or forging: hammer + anvil)
- Neither (e.g., combining items by hand)

### Tool Quality Levels
Tools have quality tiers that affect crafting:
- Makeshift
- Crude
- Common
- Uncommon
- Rare
- Epic
- Legendary

### Item Type Categories
Items can belong to multiple categories simultaneously:
- **Material** - can be used as a component in crafting
- **Tool** - can be used to craft other items
- **Placeable Object** - has physical presence in the game world

**Example**: A pike could be:
- Placed on the ground as decoration/defense
- Used as a weapon/tool
- Used as a material component in a "spear wall" construction

## Multi-Component System

### Tinker's Construct-Style Crafting
- Items can have multiple components
- Each component can be made from different materials
- Material choice affects the final item's properties

### Special Material Properties
Certain materials grant special qualities to crafted items:
- **Example**: Using mana-steel in a sword blade → grants bonus magic attack stats
- Materials can have inherent properties that transfer to the final item

## Content Categories

### Initial Content Set: 100 Items

**Inspiration Source**: Old School RuneScape

**Category Distribution:**

| Category | Count | Notes |
|----------|-------|-------|
| Resource Nodes | 12 | Mining nodes, trees, harvestable plants |
| Creatures | 8 | Common to special beasts (sources for materials) |
| Raw Materials | 25 | Ores, logs, fibers, creature parts, herbs |
| Processed Materials | 20 | Bars, refined wood, textiles, components, reagents |
| Crafting Stations | 8 | Basic to special (infrastructure for crafting) |
| Tools | 10 | Item definitions with component slots |
| Weapons | 8 | Item definitions with component slots |
| Armor | 6 | Item definitions with component slots |
| Consumables | 3 | Food and potions |
| **TOTAL** | **100** | |

**Quality Tier Distribution (for crafted items):**
- Crude: ~8 items (basic tools, simple weapons)
- Common: ~8 items (iron tier equipment)
- Uncommon: ~5 items (steel tier equipment)
- Rare: ~3 items (mithril tier, special crafting)
- Epic: ~2 items (manasteel components, enchanted)
- Legendary: ~1 item (complex provenance showcase)

**Key Design Principle:**
- No generic versions (no "normal_wood", only oak/willow/yew/elder)
- Item definitions have component slots (one `sword` definition, not `bronze_sword` vs `iron_sword`)
- Material choice creates variety (sword with bronze blade vs steel blade)
- Creatures enable provenance tracking ("slain with X" requirements)

## Scope & Boundaries

### What This Subsystem Includes
- Crafting mechanics
- Resource gathering
- Recipe/blueprint definitions
- Construction requirements
- Material properties (as they relate to crafting)

### What This Subsystem Does NOT Include
- Stats systems
- Combat mechanics
- Other game systems outside of crafting

**Focus**: Keep this subsystem isolated and focused purely on crafting mechanics.

## In-World Objects

### Resource Nodes
- Physical objects in the game world
- Source of raw materials
- Can be part of a construction requirement
- **Example**: Iron ore boulder node + pickaxe tool → produces iron ore material

### Crafting Stations
- Physical objects in the game world
- Required for certain constructions
- Can themselves be crafted items

### Construction Example: Resource Gathering
```
Item: Iron Ore
Construction:
  tool: Some(Pickaxe) // common quality or higher
  world_object: Some(IronOreBoulder) // ResourceNode type
  materials: []
Result: Iron Ore (material)
```

### Construction Example: Multi-Stage Crafting

**Item: Totem of Ambitious Mining**

**Stage 1 - Whittling**
```
Intermediate Item: Base Powerless Totem
Construction:
  tool: Some(Knife) // quality affects crafting success/final quality
  world_object: None
  materials: [Wood] // quality affects base totem quality
Result: Base Powerless Totem
```

**Stage 2 - Imbuing**
```
Final Item: Totem of Ambitious Mining
Construction:
  tool: None
  world_object: Some(MagicCraftingStation) // CraftingStation type
  materials: [
    BasePowerlessTotem, // from Stage 1
    IronOre,
    PowderedBone // must meet minimum quality threshold
  ]
Result: Totem of Ambitious Mining (final quality determined by all components)
```

**Quality Cascade:**
- Wood quality → affects base totem
- Knife quality → affects whittling outcome
- Powdered bone quality → must meet threshold for final item quality
- All qualities combine to determine final item properties

**Open Design Questions:**
- Is a totem a tool with durability that degrades over time?
- Is a totem consumable with one-time use that has a duration effect?
- Can totems be recharged/repaired for less than the initial construction cost?
- How do usage mechanics affect the crafting system design?

These decisions will impact:
- Recipe design (permanent vs. consumable items)
- Resource economy (one-time vs. renewable costs)
- Recharge/repair recipes (if applicable)

## Design Goals

### LLM Content Generation
- Create a robust and expressive system
- Design with LLM-generated content in mind
- System should be structured enough to guide LLM content creation
- Flexible enough to allow creative item and recipe generation

### Crafting Traceability & Provenance
- **Informationally Lossless**: Full construction history must be preserved
- **Deep Traceability**: Track all components through the entire crafting chain
- **Example Chain**:
  - Final item: Enchanted Sword
  - Component: Potion (splashed on blade during construction)
  - Sub-component: Powdered eyeball (ingredient in potion)
  - Source: Rat eyeball (original material source)
  - **Query Result**: System can determine that rat material was involved in sword construction

This allows for:
- Quest requirements ("craft something using dragon scales")
- Material restrictions ("contains no animal products")
- Lore/history tracking for unique items
- Debugging/testing of crafting chains

### Modularity
- Keep crafting subsystem independent
- Clear interfaces for other systems to query crafting data
- Don't implement features that belong in other subsystems

---

## Item Content

See [items_list.md](./items_list.md) for the complete 100-item catalog, including:
- Complete item list with descriptions
- Component slot definitions for all multi-component items
- Complex provenance examples (7 legendary items)
- Crafting chains and material dependencies

---

# Combat System Brainstorm

## Core Design

### One-v-One Combat System
- **Simultaneous Resolution**: Both combatants' attacks resolve at the same time (similar to Super Auto Pets)
- **Round-Based**: Combat progresses in rounds where both combatants attack each other simultaneously
- **Simple Stats**: Each combatant has:
  - **Health**: Current hit points
  - **Attack**: Damage dealt per attack

### Combat Flow
1. Each round, both combatants attack simultaneously
2. Each combatant deals damage equal to their attack stat
3. Damage is applied to the opponent's health
4. Combat continues until one or both combatants reach 0 or below health
5. Combat ends when at least one combatant is defeated

### Design Principles
- **Simplicity**: Minimal stats (just health and attack)
- **Simultaneous**: No turn order, both attacks happen at once
- **Deterministic**: Same inputs produce same results (no randomness for now)

## Type Design

### Combatant
```rust
pub struct Combatant {
    pub health: i32,
    pub attack: i32,
}
```

### Combat State
```rust
pub struct CombatState {
    pub combatant1: Combatant,
    pub combatant2: Combatant,
    pub round: u32,
}
```

### Combat Result
```rust
pub enum CombatResult {
    Ongoing,           // Combat continues
    Combatant1Wins,   // Combatant 1 defeated combatant 2
    Combatant2Wins,   // Combatant 2 defeated combatant 1
    Draw,             // Both defeated simultaneously
}
```

## Function Design

### Core Functions
1. **`new_combatant(health: i32, attack: i32) -> Combatant`**
   - Create a new combatant with specified stats

2. **`new_combat_state(combatant1: Combatant, combatant2: Combatant) -> CombatState`**
   - Initialize a new combat state with two combatants

3. **`execute_round(state: &mut CombatState) -> CombatResult`**
   - Execute one round of combat (both attack simultaneously)
   - Update both combatants' health
   - Increment round counter
   - Return the result (Ongoing, or winner/draw)

4. **`simulate_combat(mut state: CombatState) -> (CombatState, CombatResult)`**
   - Run combat to completion
   - Execute rounds until someone wins or draw
   - Return final state and result

### Helper Functions
- **`is_defeated(combatant: &Combatant) -> bool`**
  - Check if a combatant is defeated (health <= 0)

- **`get_combat_result(state: &CombatState) -> CombatResult`**
  - Determine the current combat result without executing a round

