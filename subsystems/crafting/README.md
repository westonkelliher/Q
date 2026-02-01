# Crafting Subsystem

A crafting system with tag-based material compatibility, multi-component items, and full provenance tracking.

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `0f88afd`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

## Features

- **Tag-based material compatibility**: Materials have tags (e.g., `metal`, `wood`), and component slots accept materials with matching tags
- **Tag-based world object requirements**: Recipes can require specific world objects OR any with matching tags (e.g., `high_heat` matches forge, kiln, bonfire)
- **Multi-component items**: Tinker's Construct-style items with named slots (blade, handle, pommel)
- **Quality tiers**: Makeshift → Crude → Common → Uncommon → Rare → Epic → Legendary
- **Lossless provenance tracking**: Full traceability of crafting chains for quests and lore
- **LLM-friendly**: String-based IDs designed for content generation

## Architecture

```
ItemDefinition  ─┬─> ComponentSlot ─> MaterialTag
                 └─> ItemCategories (material, tool, placeable, consumable)

Recipe ─> Construction ─┬─> ToolRequirement
                        ├─> WorldObjectRequirement (kind OR tags)
                        └─> MaterialInput (item_id OR tags)

ItemInstance ─┬─> ComponentInstance (slot → material)
              └─> Provenance (recipe, consumed inputs, tool, world object instance)
```

## Modules

| Module | Purpose |
|--------|---------|
| `ids` | Identifier types (`ItemId`, `RecipeId`, `MaterialTag`, `WorldObjectTag`, `ItemInstanceId`, `WorldObjectInstanceId`) |
| `quality` | Quality tier enum |
| `world_object` | `WorldObjectKind` (ResourceNode, CraftingStation) |
| `item_def` | Item definitions with component slots and categories |
| `recipe` | Recipes, constructions, material/world object requirements |
| `instance` | Runtime item instances with component tracking |
| `provenance` | Crafting history tracking (immediate inputs) |
| `registry` | Central storage for definitions and instances |

## Key Types

### Requirements (for recipes)

**MaterialInput**: Specifies what materials a recipe consumes
```rust
MaterialInput {
    item_id: Option<ItemId>,        // specific item, OR
    required_tags: Vec<MaterialTag>, // any item with ALL these tags
    quantity: u32,
    min_quality: Option<Quality>,
}
```

**WorldObjectRequirement**: Specifies what world object is needed
```rust
WorldObjectRequirement {
    kind: Option<WorldObjectKind>,      // specific kind, OR
    required_tags: Vec<WorldObjectTag>, // any with ALL these tags
}
```

### Provenance (for traceability)

Tracks immediate inputs only - recursive queries traverse the chain:
```rust
Provenance {
    recipe_id: RecipeId,
    consumed_inputs: Vec<ConsumedInput>,  // ItemInstanceId + quantity
    tool_used: Option<ItemInstanceId>,
    world_object_used: Option<WorldObjectInstanceId>,
    crafted_at: i64,
}
```

The recipe's `Construction.world_object` tells you the *kind* requirement; provenance tells you *which specific instance* was used.

## Usage

```rust
use crafting::{ItemDefinition, Recipe, Registry, Quality, MaterialTag};

let mut registry = Registry::new();

// Register item definitions, recipes, then craft instances
// Provenance is tracked automatically through the crafting chain
```

## Design Decisions

### Why String IDs?
LLM content generation produces strings naturally. Newtype wrappers (`ItemId(String)`) provide type safety while keeping serialization simple.

### Why Immediate Provenance Only?
Storing full recursive history would duplicate data. Storing immediate inputs allows reconstruction via registry queries while keeping `ItemInstance` lightweight.

### Why Separate Tags for Materials and World Objects?
Different namespaces prevent confusion (a `metal` material tag vs a `metal` world object tag could mean different things).

---

## Notes for LLMs

When modifying this subsystem:

1. **Maintain Type Safety**: Use the newtype IDs, don't pass raw strings
2. **Preserve Losslessness**: Provenance must capture all information needed to reconstruct crafting history
3. **Keep It Isolated**: This subsystem should not depend on stats, combat, or other game systems
4. **Update Tests**: Add tests for new features
5. **Update This README**: Keep documentation current with changes
6. **Update Commit Hash Before Committing**: When asked to commit, update the "Previous Commit" hash at the top of this file to reference the commit that existed BEFORE the changes being committed

### Commit Workflow

When the user says "commit":
1. Check `git log --oneline -1` for current commit hash
2. Update "Previous Commit" in this README to that hash
3. Update "Last Updated" date if significant changes
4. Stage changes and commit with descriptive message

---

## File Structure

```
src/
├── lib.rs          # Module exports and re-exports
├── ids.rs          # ItemId, RecipeId, MaterialTag, WorldObjectTag, etc.
├── quality.rs      # Quality enum
├── world_object.rs # WorldObjectKind (ResourceNode, CraftingStation)
├── item_def.rs     # ItemDefinition, ComponentSlot, ItemCategories, ToolType
├── recipe.rs       # Recipe, Construction, MaterialInput, WorldObjectRequirement
├── instance.rs     # ItemInstance, ComponentInstance
├── provenance.rs   # Provenance, ConsumedInput
└── registry.rs     # Registry for definitions and instances
```
