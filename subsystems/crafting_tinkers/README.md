# Crafting Subsystem

A Tinker's Construct-style crafting system with three-tier material hierarchy, multi-component items, and full provenance tracking.

> **Last Updated**: 2026-02-01  
> **Previous Commit**: `3325792`  
> Check this commit hash against the previous commit to verify documentation is up-to-date.

## Features

- **Three-tier crafting system**: Submaterial → Component → Composite
- **Material hierarchy**: Materials (broad categories) → Submaterials (specific variants) → Component Kinds
- **Multi-component items**: Composites assembled from components (blade, handle, binding)
- **Quality tiers**: Makeshift → Crude → Common → Uncommon → Rare → Epic → Legendary
- **Lossless provenance tracking**: Full traceability of crafting chains
- **World object requirements**: Recipes can require specific stations or any with matching tags
- **LLM-friendly**: String-based IDs designed for content generation

## Three-Tier System

The crafting flow is strictly hierarchical:

```
Submaterial (Simple) → Component → Composite
```

1. **Simple items**: Raw submaterials (e.g., `iron_metal`, `oak_wood`, `deer_leather`), consumables, resource nodes, carcasses
2. **Components**: Crafted parts from submaterials (e.g., `handle`, `blade`, `binding`)
3. **Composites**: Final assembled items (e.g., `scimitar`, `pickaxe`) assembled from components

Each tier uses different recipe types:
- `SimpleRecipe`: Creates simple items from other simple items
- `ComponentRecipe`: Crafts a component from a submaterial
- `CompositeRecipe`: Assembles a composite from components

## Material Hierarchy

- **Material**: Broad category (e.g., `leather`, `wood`, `metal`)
- **Submaterial**: Specific variant (e.g., `deer_leather`, `oak_wood`, `iron_metal`)
- **ComponentKind**: Defines what materials a component can accept (e.g., `handle` accepts `wood` OR `leather`)

Components are crafted from submaterials whose parent material matches the ComponentKind's `accepted_materials`.

## Quality System

Quality represents **craftsmanship and process**, not material superiority.

### Quality Tiers

**Makeshift** → **Crude** → **Common** → **Uncommon** → **Rare** → **Epic** → **Legendary**

### Understanding Quality

**Makeshift Quality** represents using **substitute items or alternate recipes** for early-game progression:
- Using a `flint_blade` item **as** a knife (substitution, not crafting)
- Using a `stone` item **as** a hammer
- Using a `sharp_bone` **as** an awl

Makeshift items are **hugely disadvantageous**:
- ~25% normal durability (1/4th)
- ~2x slower to use
- May have reduced effectiveness

**No item which is a material only is inherently Makeshift** - Makeshift applies to items used in substitute roles or crafted via alternate early-progression recipes.

### Quality and Crafting Process

Quality is determined by the **crafting process**, not the materials used.

**General guidelines** (not hard rules):
- **Crude Quality**: Typically hand-crafted items (no crafting station, no tools)
- **Common Quality**: Standard craftsmanship (proper workstation + tools) - the default
- **Uncommon+ Quality**: Enhanced craftsmanship through special processes

Materials affect effectiveness and properties, NOT quality tier. A steel sword may be functionally superior to an iron sword, but if both are properly crafted, both are Common quality.

## Multi-Component Items: Key Concept

**You define ONE composite item with component slots, not separate items per material.**

For example, you define a single `scimitar` composite with slots for `blade`, `handle`, and `binding`. Different scimitars are created by choosing different components:

```rust
ItemDefinition {
    id: "scimitar",
    kind: ItemKind::Composite(CompositeDef {
        slots: vec![
            CompositeSlot { name: "blade", component_kind: "scimitar_blade" },
            CompositeSlot { name: "handle", component_kind: "handle" },
            CompositeSlot { name: "binding", component_kind: "binding" },
        ],
        category: CompositeCategory::Weapon,
        tool_type: None,
    }),
    // ...
}

// Many instances from the SAME definition
Instance 1: blade=steel_metal, handle=oak_wood, binding=deer_leather
Instance 2: blade=manasteel_metal, handle=ebony_wood, binding=wolf_leather
```

**There is no `steel_scimitar` item definition** - only a `scimitar` assembled with steel components.

## Everything is an Item: Key Concept

**All game objects in this system are items** - ores, tools, weapons, crafting stations, carcasses, and resource nodes.

Some items are **placeable**. When placed in the world, they become **world object instances**:
- **Crafting stations**: A `forge` is an item. Craft it, place it, then use the placed instance to smelt ores.
- **Resource nodes**: A `copper_boulder` is an item definition. Instances exist as placed world objects that can be mined.
- **Carcasses**: A `wolf_carcass` is an item. When a wolf dies, a carcass item instance is placed in the world and can be harvested.

```
ItemDefinition (forge)
    ↓ craft
ItemInstance (a specific forge you made)
    ↓ place in world
WorldObjectInstance (the forge at position X,Y - referenced by WorldObjectInstanceId)
    ↓ use for smelting
Provenance records which WorldObjectInstanceId was used
```

## Architecture

```
Material → Submaterial → ComponentKind
                           ↓
                    ComponentRecipe
                           ↓
                    ComponentInstance
                           ↓
                    CompositeRecipe
                           ↓
                    CompositeInstance

ItemDefinition ─┬─> Simple { submaterial: Option<SubmaterialId> }
                 ├─> Component { component_kind: ComponentKindId }
                 └─> Composite(CompositeDef { slots, category, tool_type })

Recipe ─┬─> SimpleRecipe { inputs: Vec<SimpleInput>, tool, world_object }
        ├─> ComponentRecipe { tool, world_object }  // input implicit: submaterial
        └─> CompositeRecipe { tool, world_object }  // inputs implicit: component slots

ItemInstance ─┬─> SimpleInstance
              ├─> ComponentInstance { component_kind, submaterial }
              └─> CompositeInstance { components: HashMap<slot, ComponentInstance> }
```

## Modules

| Module | Purpose |
|--------|---------|
| `ids` | Identifier types (`ItemId`, `RecipeId`, `MaterialId`, `SubmaterialId`, `ComponentKindId`, `WorldObjectTag`, `ItemInstanceId`, `WorldObjectInstanceId`) |
| `materials` | Material hierarchy (`Material`, `Submaterial`, `ComponentKind`) |
| `quality` | Quality tier enum |
| `world_object` | `WorldObjectKind` (ResourceNode, CraftingStation), `WorldObjectInstance` |
| `item_def` | Item definitions (`ItemDefinition`, `ItemKind`, `CompositeDef`, `CompositeSlot`, `CompositeCategory`, `ToolType`) |
| `recipe` | Recipe types (`SimpleRecipe`, `ComponentRecipe`, `CompositeRecipe`, `SimpleInput`, `ToolRequirement`, `WorldObjectRequirement`) |
| `instance` | Runtime item instances (`ItemInstance`, `SimpleInstance`, `ComponentInstance`, `CompositeInstance`) |
| `provenance` | Crafting history tracking (`Provenance`, `ConsumedInput`) |
| `registry` | Central storage for definitions and instances |
| `content` | Sample content registration |
| `cli` | REPL-based CLI for testing |

## Key Types

### Recipes

**SimpleRecipe**: Creates simple items from other simple items
```rust
SimpleRecipe {
    id: RecipeId,
    output: ItemId,
    output_quantity: u32,
    inputs: Vec<SimpleInput>,  // item_id + quantity
    tool: Option<ToolRequirement>,
    world_object: Option<WorldObjectRequirement>,
}
```

**ComponentRecipe**: Crafts a component from a submaterial (input is implicit)
```rust
ComponentRecipe {
    id: RecipeId,
    output: ComponentKindId,
    tool: Option<ToolRequirement>,
    world_object: Option<WorldObjectRequirement>,
}
```

**CompositeRecipe**: Assembles a composite from components (inputs are implicit - whatever slots the composite requires)
```rust
CompositeRecipe {
    id: RecipeId,
    output: ItemId,
    tool: Option<ToolRequirement>,
    world_object: Option<WorldObjectRequirement>,
}
```

**WorldObjectRequirement**: Specifies what world object is needed
```rust
WorldObjectRequirement {
    kind: Option<WorldObjectKind>,      // specific kind, OR
    required_tags: Vec<WorldObjectTag>, // any with ALL these tags
}
```

### Item Definitions

**ItemDefinition**: Defines what an item is
```rust
ItemDefinition {
    id: ItemId,
    name: String,
    description: String,
    kind: ItemKind,  // Simple, Component, or Composite
}
```

**CompositeSlot**: A slot in a composite that accepts a specific component kind
```rust
CompositeSlot {
    name: String,                    // e.g., "blade", "handle"
    component_kind: ComponentKindId, // which component type fits here
}
```

### Instances

**SimpleInstance**: A simple item instance
```rust
SimpleInstance {
    id: ItemInstanceId,
    definition: ItemId,
    provenance: Provenance,
}
```

**ComponentInstance**: A component instance (tracks which submaterial was used)
```rust
ComponentInstance {
    id: ItemInstanceId,
    component_kind: ComponentKindId,
    submaterial: SubmaterialId,
    provenance: Provenance,
}
```

**CompositeInstance**: A composite instance (tracks which components were used)
```rust
CompositeInstance {
    id: ItemInstanceId,
    definition: ItemId,
    quality: Quality,
    components: HashMap<String, ComponentInstance>,  // slot → component
    provenance: Provenance,
}
```

### Provenance

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

## Usage

### Interactive CLI (Recommended for Exploration)

The easiest way to explore the system is through the interactive CLI:

```bash
cargo run
```

This starts a REPL where you can:
- List items and recipes
- Create items and craft new ones
- Simulate combat between combatants
- Explore the crafting system interactively

See [QUICK_START.md](./QUICK_START.md) for a quick guide, or [CLI_USAGE.md](./CLI_USAGE.md) for detailed documentation.

### Programmatic Usage

```rust
use crafting::{Registry, Material, Submaterial, ComponentKind, ItemDefinition, SimpleRecipe, ComponentRecipe, CompositeRecipe};

let mut registry = Registry::new();

// Register materials, submaterials, component kinds
registry.register_material(Material { id: MaterialId("metal".into()), ... });
registry.register_submaterial(Submaterial { id: SubmaterialId("iron_metal".into()), material: MaterialId("metal".into()), ... });

// Register item definitions and recipes
registry.register_item_definition(ItemDefinition { ... });
registry.register_simple_recipe(SimpleRecipe { ... });

// Craft instances - provenance is tracked automatically
let instance_id = registry.craft_simple_recipe(...)?;
```

## Design Decisions

### Why String IDs?
LLM content generation produces strings naturally. Newtype wrappers (`ItemId(String)`) provide type safety while keeping serialization simple.

### Why Three-Tier System?
Enforces a clear crafting progression: materials → parts → final items. Prevents invalid crafting chains and simplifies validation.

### Why Immediate Provenance Only?
Storing full recursive history would duplicate data. Storing immediate inputs allows reconstruction via registry queries while keeping `ItemInstance` lightweight.

### Why Separate Material Hierarchy?
Separating Materials, Submaterials, and ComponentKinds allows flexible material compatibility rules while keeping the system extensible.

---

## Notes for LLMs

When modifying this subsystem:

1. **Maintain Type Safety**: Use the newtype IDs, don't pass raw strings
2. **Preserve Losslessness**: Provenance must capture all information needed to reconstruct crafting history
3. **Keep It Isolated**: This subsystem should not depend on stats, combat, or other game systems
4. **Respect Three-Tier Flow**: Simple → Component → Composite (no skipping tiers)
5. **Update Tests**: Add tests for new features
6. **Update This README**: Keep documentation current with changes
7. **Update Commit Hash Before Committing**: When asked to commit, update the "Previous Commit" hash at the top of this file to reference the commit that existed BEFORE the changes being committed

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
├── ids.rs          # All ID types
├── materials.rs    # Material, Submaterial, ComponentKind
├── quality.rs      # Quality enum
├── world_object.rs # WorldObjectKind, WorldObjectInstance
├── item_def.rs     # ItemDefinition, ItemKind, CompositeDef, CompositeSlot
├── recipe.rs       # SimpleRecipe, ComponentRecipe, CompositeRecipe
├── instance.rs     # ItemInstance, SimpleInstance, ComponentInstance, CompositeInstance
├── provenance.rs   # Provenance, ConsumedInput
├── registry.rs     # Registry for definitions and instances
├── content.rs      # Sample content registration
└── cli.rs          # REPL-based CLI
```
