# Crafting Subsystem

A crafting system with tag-based material compatibility, multi-component items, and full provenance tracking.

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `af4070d`  
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

**MaterialInput**: Specifies what materials a recipe consumes, with optional recursive provenance requirements
```rust
MaterialInput {
    item_id: Option<ItemId>,              // specific item, OR
    required_tags: Vec<MaterialTag>,      // any item with ALL these tags
    quantity: u32,
    min_quality: Option<Quality>,
    component_reqs: Vec<ComponentRequirement>,      // requirements on item's components
    provenance_reqs: Option<Box<ProvenanceRequirements>>,  // recursive!
}
```

**ComponentRequirement**: Requirements on a specific component of a multi-part item
```rust
ComponentRequirement {
    slot_name: String,                      // e.g., "blade"
    required_material_tags: Vec<MaterialTag>, // e.g., ["manasteel"]
}
```

**ProvenanceRequirements**: Recursive requirements on how an item was made
```rust
ProvenanceRequirements {
    consumed_inputs: Vec<MaterialInput>,  // requirements on consumed materials
    tool: Option<MaterialInput>,          // requirements on tool used (recursive!)
    world_object: Option<MaterialInput>,  // requirements on world object used (recursive!)
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

## Translating Natural Language to MaterialInput

Recipe requirements are often expressed in natural language. Here's how to translate them into `MaterialInput` structures.

### Grammar Pattern

Natural language requirements follow this pattern:
```
[item] (whose [source] came from [item] (that was [verb] with [tool/material]))
```

Each nested clause maps to a level of `ProvenanceRequirements`.

### Translation Rules

| Natural Language | Code Structure |
|-----------------|----------------|
| "a heart" | `required_tags: vec![MaterialTag("heart")]` |
| "whose source..." | `provenance_reqs: Some(Box::new(...))` |
| "came from [X]" (world object) | `world_object: Some(MaterialInput { ... })` |
| "made with [X]" (consumed) | `consumed_inputs: vec![MaterialInput { ... }]` |
| "using [tool]" | `tool: Some(MaterialInput { ... })` |
| "whose [slot] is [material]" | `component_reqs: vec![ComponentRequirement { slot_name, required_material_tags }]` |

### Example: Complex Provenance Requirement

**Natural language**: 
> "A heart, whose source carcass came from a wolf slain with a weapon whose blade is manasteel."

**Parse tree**:
```
A heart
└── whose source carcass came from
    └── a wolf
        └── slain with
            └── a weapon
                └── whose blade is manasteel
```

**Translation**:
```rust
MaterialInput {
    // "A heart"
    required_tags: vec![MaterialTag("heart".into())],
    quantity: 1,
    
    // "whose source carcass came from..."
    provenance_reqs: Some(Box::new(ProvenanceRequirements {
        // "came from" a world object (the placed carcass)
        world_object: Some(MaterialInput {
            // "a wolf" (wolf_carcass)
            required_tags: vec![MaterialTag("wolf_carcass".into())],
            
            // "slain with..."
            provenance_reqs: Some(Box::new(ProvenanceRequirements {
                // "slain with a weapon"
                tool: Some(MaterialInput {
                    required_tags: vec![MaterialTag("weapon".into())],
                    
                    // "whose blade is manasteel"
                    component_reqs: vec![
                        ComponentRequirement {
                            slot_name: "blade".into(),
                            required_material_tags: vec![MaterialTag("manasteel".into())],
                        }
                    ],
                    ..Default::default()
                }),
                ..Default::default()
            })),
            ..Default::default()
        }),
        ..Default::default()
    })),
    ..Default::default()
}
```

### More Examples

**"Iron ore"** (simple):
```rust
MaterialInput {
    required_tags: vec![MaterialTag("iron_ore".into())],
    quantity: 1,
    ..Default::default()
}
```

**"A gem polished with a rare or better tool"**:
```rust
MaterialInput {
    required_tags: vec![MaterialTag("gem".into())],
    provenance_reqs: Some(Box::new(ProvenanceRequirements {
        tool: Some(MaterialInput {
            min_quality: Some(Quality::Rare),
            ..Default::default()
        }),
        ..Default::default()
    })),
    ..Default::default()
}
```

**"Leather from a beast killed at a sacrificial altar"**:
```rust
MaterialInput {
    required_tags: vec![MaterialTag("leather".into())],
    provenance_reqs: Some(Box::new(ProvenanceRequirements {
        consumed_inputs: vec![
            MaterialInput {
                required_tags: vec![MaterialTag("hide".into())],
                provenance_reqs: Some(Box::new(ProvenanceRequirements {
                    world_object: Some(MaterialInput {
                        required_tags: vec![MaterialTag("beast_carcass".into())],
                        provenance_reqs: Some(Box::new(ProvenanceRequirements {
                            world_object: Some(MaterialInput {
                                required_tags: vec![MaterialTag("sacrificial_altar".into())],
                                ..Default::default()
                            }),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    ..Default::default()
                })),
                ..Default::default()
            }
        ],
        ..Default::default()
    })),
    ..Default::default()
}
```

### Translation Checklist

1. **Identify the base item** → `required_tags` or `item_id`
2. **Find "whose/from/with" clauses** → each creates a `ProvenanceRequirements` level
3. **Determine relationship type**:
   - "made from/using [material]" → `consumed_inputs`
   - "made with [tool]" → `tool`
   - "at/from [place/object]" → `world_object`
4. **Check for component requirements** → "whose [slot] is [material]" → `component_reqs`
5. **Recurse** for nested clauses

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
