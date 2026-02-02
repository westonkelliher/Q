# Crafting Subsystem - Tinker's Construct Style

A crafting system with a clean Material/Submaterial hierarchy and strict three-tier crafting flow, inspired by Minecraft's Tinker's Construct mod.

> **Last Updated**: 2026-02-01
> **Architecture**: Tinker's Construct style with Material/Submaterial hierarchy

## Overview

This crafting system follows a **Tinker's Construct style architecture** where items are composed from components, and each component is crafted from specific materials. This creates a clean, hierarchical crafting flow with full provenance tracking.

### Key Principles

1. **Material Hierarchy**: Materials are broad categories (metal, wood, leather); Submaterials are specific variants (iron_metal, oak_wood, deer_leather)
2. **Mutually Exclusive Item Kinds**: Every item is exactly ONE of: Simple, Component, or Composite
3. **Strict Three-Tier Flow**: Submaterials → Components → Composites (no shortcuts)
4. **Component Acceptance Rules**: Components accept Materials (not specific submaterials), validated at craft time
5. **Full Provenance Tracking**: Complete crafting history is preserved and queryable

### Why This Design?

**Before (Tag-Based System):**
- Items could be multiple things at once (material + tool + weapon)
- Complex tag matching with required/accepted/optional tags
- Recursive provenance requirements baked into recipes
- Single Recipe type with complex MaterialInput structure
- "Tag soup" made system hard to reason about

**After (Material/Submaterial Hierarchy):**
- Items are exactly ONE thing - no ambiguity
- Clean material categories with specific variants
- Components define what materials they accept upfront
- Three distinct recipe types with clear purposes
- Provenance preserved but decoupled from recipe definitions

---

## Core Concepts

### 1. Material vs Submaterial

**Materials** are broad categories that group related resources:

| Material | Description |
|----------|-------------|
| `leather` | Animal hides and leathers |
| `wood` | Timber and wooden materials |
| `metal` | Metallic materials and alloys |
| `gem` | Precious stones |
| `bone` | Skeletal materials |
| `fiber` | Plant fibers and sinew |
| `stone` | Stone and mineral materials |

**Submaterials** are specific variants that belong to a material:

| Material | Example Submaterials |
|----------|---------------------|
| leather | `deer_leather`, `wolf_leather`, `bear_leather` |
| wood | `oak_wood`, `yew_wood`, `elder_wood` |
| metal | `iron_metal`, `bronze_metal`, `steel_metal`, `silver_metal` |
| gem | `sapphire_gem`, `diamond_gem`, `ruby_gem` |
| bone | `wolf_bone`, `deer_bone` |
| fiber | `plant_fiber`, `sinew` |
| stone | `flint_stone`, `granite_stone` |

**Key Insight**: "Material" is NOT a type of item. Items are *of* a submaterial, and submaterials *belong to* a material category.

### 2. Three Item Kinds (Mutually Exclusive)

An item definition is **exactly ONE** of these three kinds:

#### Simple Items
Standalone items with no assembly structure. Includes:
- **Submaterial items**: Items that represent a specific submaterial (e.g., `deer_leather`, `oak_wood`, `iron_bar`)
- **Consumables**: Items that can be consumed (e.g., `cooked_meat`, `health_potion`)
- **Creatures**: Living entities (e.g., `wolf`, `deer`)
- **Resource nodes**: Harvestable world objects (e.g., `copper_boulder`, `oak_tree`)
- **Carcasses**: Remains of slain creatures (e.g., `wolf_carcass`)

```rust
ItemKind::Simple {
    submaterial: Some(SubmaterialId("deer_leather".into()))
}

ItemKind::Simple {
    submaterial: None  // Not a submaterial (e.g., consumable)
}
```

#### Component Items
Parts crafted from a submaterial, used **exclusively** as inputs to Composite items. Each component instance tracks which specific submaterial was used.

Examples:
- `binding` (made from leather or fiber submaterials)
- `handle` (made from wood or bone submaterials)
- `scimitar_blade` (made from metal submaterials)
- `pickaxe_head` (made from metal or stone submaterials)

```rust
ItemKind::Component {
    component_kind: ComponentKindId("handle".into())
}
```

**Important**: Components are NOT standalone usable items - they exist solely to be assembled into composites.

#### Composite Items
Final assembled items built from components. The classic Tinker's Construct pattern.

Examples:
- `scimitar` (requires: scimitar_blade + handle + binding)
- `pickaxe` (requires: pickaxe_head + handle)
- `sword` (requires: sword_blade + handle + pommel)

```rust
ItemKind::Composite(CompositeDef {
    slots: vec![
        CompositeSlot {
            name: "blade".into(),
            component_kind: ComponentKindId("scimitar_blade".into())
        },
        CompositeSlot {
            name: "handle".into(),
            component_kind: ComponentKindId("handle".into())
        },
        CompositeSlot {
            name: "binding".into(),
            component_kind: ComponentKindId("binding".into())
        },
    ],
    category: CompositeCategory::Weapon,
    tool_type: None,
})
```

**Important**: Composites do NOT have a material - they have components, each with their own submaterial.

### 3. Component Kinds and Material Acceptance

**Component Kinds** define types of components and specify which materials they accept:

```rust
ComponentKind {
    id: ComponentKindId("handle".into()),
    name: "Handle".into(),
    accepted_materials: vec![
        MaterialId("wood".into()),
        MaterialId("bone".into())
    ],  // OR logic
    makeshift_tags: vec![],
}

ComponentKind {
    id: ComponentKindId("scimitar_blade".into()),
    name: "Scimitar Blade".into(),
    accepted_materials: vec![
        MaterialId("metal".into())
    ],
    makeshift_tags: vec![],
}
```

**Acceptance Rules:**
- Component kinds specify **materials** they accept, not specific submaterials
- Uses OR logic: a handle accepts (wood OR bone)
- At craft time, the system validates that the submaterial's parent material is in the accepted list
- Example: `oak_wood` (submaterial) → `wood` (material) → accepted by `handle`

**Makeshift Tags:**
Some components can substitute for tools in non-crafting scenarios:
```rust
ComponentKind {
    id: ComponentKindId("knife_blade".into()),
    accepted_materials: vec![MaterialId("metal".into()), MaterialId("stone".into())],
    makeshift_tags: vec!["knife".into()],  // Can act as makeshift knife
}
```

This does NOT affect crafting recipes - only gameplay substitution.

### 4. Three-Tier Crafting Flow

The crafting flow is strictly hierarchical:

```
┌─────────────┐
│  Submaterial│  Simple Item (e.g., iron_bar)
│    Item     │  submaterial: Some(iron_metal)
└──────┬──────┘
       │ ComponentRecipe
       │ (validates material acceptance)
       ▼
┌─────────────┐
│  Component  │  Component Item (e.g., blade)
│   Instance  │  component_kind: scimitar_blade
│             │  submaterial: iron_metal
└──────┬──────┘
       │ CompositeRecipe
       │ (assembles components)
       ▼
┌─────────────┐
│  Composite  │  Composite Item (e.g., scimitar)
│   Instance  │  components: {blade: ComponentInstance, ...}
└─────────────┘
```

**No Shortcuts:**
- You cannot use a submaterial directly in a composite
- You cannot skip the component crafting step
- Each tier has distinct recipes and validation rules

---

## Architecture

### Item System

#### Simple Items
```rust
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Simple {
        submaterial: Option<SubmaterialId>  // Some if this represents a submaterial
    },
    Component {
        component_kind: ComponentKindId
    },
    Composite(CompositeDef),
}
```

**Simple Item Examples:**
```rust
// Submaterial item - represents deer leather
ItemDefinition {
    id: ItemId("deer_leather".into()),
    kind: ItemKind::Simple { submaterial: Some(SubmaterialId("deer_leather".into())) },
}

// Non-submaterial simple item
ItemDefinition {
    id: ItemId("health_potion".into()),
    kind: ItemKind::Simple { submaterial: None },
}
```

#### Component Items
```rust
// Component item definition
ItemDefinition {
    id: ItemId("handle".into()),
    name: "Handle".into(),
    kind: ItemKind::Component {
        component_kind: ComponentKindId("handle".into())
    },
}

// The ComponentKind definition (separate from item)
ComponentKind {
    id: ComponentKindId("handle".into()),
    name: "Handle".into(),
    accepted_materials: vec![MaterialId("wood".into()), MaterialId("bone".into())],
    makeshift_tags: vec![],
}
```

#### Composite Items
```rust
pub struct CompositeDef {
    pub slots: Vec<CompositeSlot>,
    pub category: CompositeCategory,
    pub tool_type: Option<ToolType>,
}

pub struct CompositeSlot {
    pub name: String,                     // "blade", "handle", "binding"
    pub component_kind: ComponentKindId,  // which component type fits here
}

pub enum CompositeCategory {
    Tool,
    Weapon,
    Armor,
}
```

**Composite Example:**
```rust
ItemDefinition {
    id: ItemId("scimitar".into()),
    name: "Scimitar".into(),
    kind: ItemKind::Composite(CompositeDef {
        slots: vec![
            CompositeSlot {
                name: "blade".into(),
                component_kind: ComponentKindId("scimitar_blade".into())
            },
            CompositeSlot {
                name: "handle".into(),
                component_kind: ComponentKindId("handle".into())
            },
            CompositeSlot {
                name: "binding".into(),
                component_kind: ComponentKindId("binding".into())
            },
        ],
        category: CompositeCategory::Weapon,
        tool_type: None,
    }),
}
```

### Recipe System

Three distinct recipe types for three-tier crafting:

#### SimpleRecipe
Creates Simple items from other Simple items. Used for mining, harvesting, smelting, etc.

```rust
pub struct SimpleRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,              // Must be a Simple item
    pub output_quantity: u32,
    pub inputs: Vec<SimpleInput>,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

pub struct SimpleInput {
    pub item_id: ItemId,
    pub quantity: u32,
}
```

**Example: Smelting Iron**
```rust
SimpleRecipe {
    id: RecipeId("smelt_iron_bar".into()),
    name: "Smelt Iron Bar".into(),
    output: ItemId("iron_bar".into()),
    output_quantity: 1,
    inputs: vec![
        SimpleInput { item_id: ItemId("iron_ore".into()), quantity: 1 }
    ],
    tool: Some(ToolRequirement {
        tool_type: ToolType::Hammer,
        min_quality: Quality::Common
    }),
    world_object: Some(WorldObjectRequirement {
        kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".into()))),
        required_tags: vec![],
    }),
}
```

#### ComponentRecipe
Creates Component items from submaterial items. The input is **implicit** - you provide a submaterial item whose material must be in the ComponentKind's accepted_materials list.

```rust
pub struct ComponentRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ComponentKindId,     // Creates a component
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Input is implicit: one submaterial item with accepted material
}
```

**Example: Crafting a Handle**
```rust
ComponentRecipe {
    id: RecipeId("craft_handle".into()),
    name: "Craft Handle".into(),
    output: ComponentKindId("handle".into()),
    tool: Some(ToolRequirement {
        tool_type: ToolType::Knife,
        min_quality: Quality::Common
    }),
    world_object: None,
}

// At craft time, you provide (for example) oak_wood
// System validates: oak_wood → submaterial → wood (material) → accepted by handle
// Creates: ComponentInstance { component_kind: handle, submaterial: oak_wood }
```

#### CompositeRecipe
Assembles Composite items from Component items. The inputs are **implicit** - you must provide components matching each slot's component_kind from the CompositeDef.

```rust
pub struct CompositeRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,              // Must be a Composite item
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Inputs are implicit: whatever ComponentKinds the composite's slots require
}
```

**Example: Assembling a Scimitar**
```rust
CompositeRecipe {
    id: RecipeId("assemble_scimitar".into()),
    name: "Assemble Scimitar".into(),
    output: ItemId("scimitar".into()),
    tool: Some(ToolRequirement {
        tool_type: ToolType::Hammer,
        min_quality: Quality::Common
    }),
    world_object: Some(WorldObjectRequirement {
        kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("workbench".into()))),
        required_tags: vec![],
    }),
}

// At craft time, you provide:
// - blade component (iron_blade instance)
// - handle component (oak_handle instance)
// - binding component (leather_binding instance)
// System validates component kinds match slots
// Creates: CompositeInstance with all component details
```

### Instance System

Three instance types mirror the three item kinds:

#### SimpleInstance
```rust
pub struct SimpleInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub provenance: Provenance,
}
```

#### ComponentInstance
```rust
pub struct ComponentInstance {
    pub id: ItemInstanceId,
    pub component_kind: ComponentKindId,
    pub submaterial: SubmaterialId,  // Tracks which submaterial was used
    pub provenance: Provenance,
}
```

**Key Point**: ComponentInstance stores the submaterial ID, allowing you to query "show me all handles made from oak_wood" or "all blades made from steel_metal".

#### CompositeInstance
```rust
pub struct CompositeInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub quality: Quality,  // TODO: quality calculation, defaults to Common
    pub components: HashMap<String, ComponentInstance>,  // slot_name -> component
    pub provenance: Provenance,
}
```

**Key Point**: CompositeInstance stores the full component instances, preserving the complete material chain. A scimitar instance knows it has an iron blade, oak handle, and deer leather binding.

#### Unified ItemInstance Enum
```rust
pub enum ItemInstance {
    Simple(SimpleInstance),
    Component(ComponentInstance),
    Composite(CompositeInstance),
}

impl ItemInstance {
    pub fn id(&self) -> ItemInstanceId { /* ... */ }
    pub fn provenance(&self) -> &Provenance { /* ... */ }
}
```

### Provenance Tracking

Provenance is preserved across all three tiers:

```rust
pub struct Provenance {
    pub recipe_id: RecipeId,
    pub consumed_inputs: Vec<ConsumedInput>,
    pub tool_used: Option<ItemInstanceId>,
    pub world_object_used: Option<WorldObjectInstanceId>,
    pub crafted_at: i64,  // timestamp
}

pub struct ConsumedInput {
    pub instance_id: ItemInstanceId,
    pub quantity: u32,
}
```

**Provenance Chain Example:**
```
scimitar_instance (CompositeInstance)
  ├─ provenance.recipe_id: "assemble_scimitar"
  ├─ provenance.consumed_inputs: [blade_instance_id, handle_instance_id, binding_instance_id]
  └─ provenance.tool: hammer_instance_id

blade_instance (ComponentInstance)
  ├─ component_kind: scimitar_blade
  ├─ submaterial: iron_metal
  ├─ provenance.recipe_id: "craft_blade"
  └─ provenance.consumed_inputs: [iron_bar_instance_id]

iron_bar_instance (SimpleInstance)
  ├─ definition: "iron_bar"
  ├─ provenance.recipe_id: "smelt_iron_bar"
  └─ provenance.consumed_inputs: [iron_ore_instance_id]
```

You can traverse this chain to answer queries like:
- "Show me all swords with steel blades"
- "Find all items crafted at the enchanted forge"
- "List all items that trace back to deer killed with bronze weapons"

### Material Validation Rules

The system enforces strict validation:

1. **Component Material Validation**
   - When crafting a component, you provide a Simple item with a submaterial
   - System validates: submaterial's parent material must be in ComponentKind.accepted_materials
   - Example: `oak_wood` (submaterial) → `wood` (material) → accepted by `handle`

2. **Composite Slot Validation**
   - When crafting a composite, you provide components for each slot
   - System validates: each component's component_kind must match slot's component_kind
   - Example: scimitar's "blade" slot requires component_kind "scimitar_blade"

3. **Item Kind Enforcement**
   - SimpleRecipes only produce Simple items
   - ComponentRecipes only produce Component items
   - CompositeRecipes only produce Composite items
   - No mixing allowed

4. **Tool and World Object Requirements**
   - Optional but enforced when specified
   - Tool must match ToolType and meet minimum quality
   - World object must match kind or have required tags

---

## Complete Crafting Flow Example

Let's craft a scimitar from scratch:

### Step 1: Gather Raw Materials (SimpleRecipes)

**Mine Iron Ore:**
```rust
// Recipe: mine_iron_ore
// Input: iron_boulder (world object instance)
// Output: iron_ore (Simple item, no submaterial)

let iron_ore_instance = SimpleInstance {
    id: ItemInstanceId(1001),
    definition: ItemId("iron_ore".into()),
    provenance: Provenance {
        recipe_id: RecipeId("mine_iron_ore".into()),
        world_object_used: Some(WorldObjectInstanceId(500)), // the boulder
        tool_used: Some(ItemInstanceId(50)), // pickaxe used for mining
        consumed_inputs: vec![],
        crafted_at: 1738450000,
    },
};
```

**Smelt Iron Bar:**
```rust
// Recipe: smelt_iron_bar
// Input: iron_ore
// Output: iron_bar (Simple item with submaterial iron_metal)

let iron_bar_instance = SimpleInstance {
    id: ItemInstanceId(1002),
    definition: ItemId("iron_bar".into()),
    provenance: Provenance {
        recipe_id: RecipeId("smelt_iron_bar".into()),
        world_object_used: Some(WorldObjectInstanceId(501)), // forge
        tool_used: Some(ItemInstanceId(51)), // hammer
        consumed_inputs: vec![
            ConsumedInput { instance_id: ItemInstanceId(1001), quantity: 1 }
        ],
        crafted_at: 1738450100,
    },
};
```

Similarly, gather:
- `oak_wood` (Simple item with submaterial oak_wood)
- `deer_leather` (Simple item with submaterial deer_leather)

### Step 2: Craft Components (ComponentRecipes)

**Craft Blade:**
```rust
// Recipe: craft_blade
// Input: iron_bar (submaterial: iron_metal)
// Validation: iron_metal (submaterial) → metal (material) → accepted by scimitar_blade
// Output: blade component

let blade_instance = ComponentInstance {
    id: ItemInstanceId(2001),
    component_kind: ComponentKindId("scimitar_blade".into()),
    submaterial: SubmaterialId("iron_metal".into()),  // Tracked!
    provenance: Provenance {
        recipe_id: RecipeId("craft_blade".into()),
        world_object_used: Some(WorldObjectInstanceId(502)), // anvil
        tool_used: Some(ItemInstanceId(52)), // hammer
        consumed_inputs: vec![
            ConsumedInput { instance_id: ItemInstanceId(1002), quantity: 1 }
        ],
        crafted_at: 1738450200,
    },
};
```

**Craft Handle:**
```rust
// Recipe: craft_handle
// Input: oak_wood (submaterial: oak_wood)
// Validation: oak_wood → wood → accepted by handle
// Output: handle component

let handle_instance = ComponentInstance {
    id: ItemInstanceId(2002),
    component_kind: ComponentKindId("handle".into()),
    submaterial: SubmaterialId("oak_wood".into()),  // Tracked!
    provenance: Provenance {
        recipe_id: RecipeId("craft_handle".into()),
        tool_used: Some(ItemInstanceId(53)), // knife
        consumed_inputs: vec![
            ConsumedInput { instance_id: ItemInstanceId(1003), quantity: 1 }
        ],
        crafted_at: 1738450300,
    },
};
```

**Craft Binding:**
```rust
// Recipe: craft_binding
// Input: deer_leather (submaterial: deer_leather)
// Validation: deer_leather → leather → accepted by binding
// Output: binding component

let binding_instance = ComponentInstance {
    id: ItemInstanceId(2003),
    component_kind: ComponentKindId("binding".into()),
    submaterial: SubmaterialId("deer_leather".into()),  // Tracked!
    provenance: Provenance {
        recipe_id: RecipeId("craft_binding".into()),
        tool_used: Some(ItemInstanceId(54)), // needle
        consumed_inputs: vec![
            ConsumedInput { instance_id: ItemInstanceId(1004), quantity: 1 }
        ],
        crafted_at: 1738450400,
    },
};
```

### Step 3: Assemble Composite (CompositeRecipe)

**Assemble Scimitar:**
```rust
// Recipe: assemble_scimitar
// Inputs: blade, handle, binding components
// Validation: component_kinds match composite slots
// Output: scimitar (Composite item)

let scimitar_instance = CompositeInstance {
    id: ItemInstanceId(3001),
    definition: ItemId("scimitar".into()),
    quality: Quality::Common,  // TODO: calculate from components
    components: {
        let mut map = HashMap::new();
        map.insert("blade".into(), blade_instance);
        map.insert("handle".into(), handle_instance);
        map.insert("binding".into(), binding_instance);
        map
    },
    provenance: Provenance {
        recipe_id: RecipeId("assemble_scimitar".into()),
        world_object_used: Some(WorldObjectInstanceId(503)), // workbench
        tool_used: Some(ItemInstanceId(55)), // hammer
        consumed_inputs: vec![
            ConsumedInput { instance_id: ItemInstanceId(2001), quantity: 1 },
            ConsumedInput { instance_id: ItemInstanceId(2002), quantity: 1 },
            ConsumedInput { instance_id: ItemInstanceId(2003), quantity: 1 },
        ],
        crafted_at: 1738450500,
    },
};
```

### Result

You now have a **scimitar instance** with complete provenance:
- **Blade**: iron_metal (from iron_bar from iron_ore from iron_boulder)
- **Handle**: oak_wood (from oak tree)
- **Binding**: deer_leather (from deer hide from deer carcass)

Every step is tracked and queryable!

---

## Materials and Submaterials Reference

### Complete Material Definitions

```rust
// Materials
Material { id: MaterialId("leather".into()), name: "Leather".into(), ... }
Material { id: MaterialId("wood".into()), name: "Wood".into(), ... }
Material { id: MaterialId("metal".into()), name: "Metal".into(), ... }
Material { id: MaterialId("bone".into()), name: "Bone".into(), ... }
Material { id: MaterialId("fiber".into()), name: "Fiber".into(), ... }
Material { id: MaterialId("stone".into()), name: "Stone".into(), ... }
```

### Complete Submaterial Definitions

```rust
// Leather submaterials
Submaterial {
    id: SubmaterialId("deer_leather".into()),
    material: MaterialId("leather".into()),
    name: "Deer Leather".into(),
    ...
}
Submaterial {
    id: SubmaterialId("wolf_leather".into()),
    material: MaterialId("leather".into()),
    ...
}

// Wood submaterials
Submaterial {
    id: SubmaterialId("oak_wood".into()),
    material: MaterialId("wood".into()),
    ...
}
Submaterial {
    id: SubmaterialId("yew_wood".into()),
    material: MaterialId("wood".into()),
    ...
}

// Metal submaterials
Submaterial {
    id: SubmaterialId("iron_metal".into()),
    material: MaterialId("metal".into()),
    ...
}
Submaterial {
    id: SubmaterialId("bronze_metal".into()),
    material: MaterialId("metal".into()),
    ...
}
Submaterial {
    id: SubmaterialId("steel_metal".into()),
    material: MaterialId("metal".into()),
    ...
}

// Bone submaterials
Submaterial {
    id: SubmaterialId("wolf_bone".into()),
    material: MaterialId("bone".into()),
    ...
}
Submaterial {
    id: SubmaterialId("deer_bone".into()),
    material: MaterialId("bone".into()),
    ...
}

// Fiber submaterials
Submaterial {
    id: SubmaterialId("plant_fiber".into()),
    material: MaterialId("fiber".into()),
    ...
}
Submaterial {
    id: SubmaterialId("sinew".into()),
    material: MaterialId("fiber".into()),
    ...
}

// Stone submaterials
Submaterial {
    id: SubmaterialId("flint_stone".into()),
    material: MaterialId("stone".into()),
    ...
}
```

### Component Kind Definitions

```rust
// Handle - accepts wood or bone
ComponentKind {
    id: ComponentKindId("handle".into()),
    name: "Handle".into(),
    accepted_materials: vec![MaterialId("wood".into()), MaterialId("bone".into())],
    makeshift_tags: vec![],
}

// Binding - accepts leather or fiber
ComponentKind {
    id: ComponentKindId("binding".into()),
    name: "Binding".into(),
    accepted_materials: vec![MaterialId("leather".into()), MaterialId("fiber".into())],
    makeshift_tags: vec![],
}

// Scimitar blade - accepts metal only
ComponentKind {
    id: ComponentKindId("scimitar_blade".into()),
    name: "Scimitar Blade".into(),
    accepted_materials: vec![MaterialId("metal".into())],
    makeshift_tags: vec![],
}

// Knife blade - accepts metal or stone, can substitute for knife tool
ComponentKind {
    id: ComponentKindId("knife_blade".into()),
    name: "Knife Blade".into(),
    accepted_materials: vec![MaterialId("metal".into()), MaterialId("stone".into())],
    makeshift_tags: vec!["knife".into()],  // Can act as makeshift knife
}

// Pickaxe head - accepts metal or stone
ComponentKind {
    id: ComponentKindId("pickaxe_head".into()),
    name: "Pickaxe Head".into(),
    accepted_materials: vec![MaterialId("metal".into()), MaterialId("stone".into())],
    makeshift_tags: vec![],
}
```

---

## CLI Usage

The crafting system includes an interactive CLI for testing and exploration:

### Starting the CLI

```bash
cargo run
```

### Available Commands

#### List Items
```
list_items
```
Shows all registered items with their kind (Simple/Component/Composite).

**Example Output:**
```
Items:
  iron_bar [Simple] - Iron Bar (submaterial: iron_metal)
  handle [Component] - Handle (kind: handle)
  scimitar [Composite] - Scimitar (3 slots)
```

#### List Recipes
```
list_recipes
```
Shows all registered recipes with their type.

**Example Output:**
```
Recipes:
  [Simple] smelt_iron_bar - Smelt Iron Bar → iron_bar (x1)
  [Component] craft_handle - Craft Handle → handle
  [Composite] assemble_scimitar - Assemble Scimitar → scimitar
```

#### Show Item Details
```
show_item <item_id>
```
Displays detailed information about an item based on its kind.

**Example:**
```
show_item scimitar

Item: scimitar
Name: Scimitar
Kind: Composite
Category: Weapon
Slots:
  - blade: scimitar_blade
  - handle: handle
  - binding: binding
```

#### Show Recipe Details
```
show_recipe <recipe_id>
```
Displays recipe details including inputs, tools, and world objects.

**Example:**
```
show_recipe assemble_scimitar

Recipe: assemble_scimitar
Type: Composite
Output: scimitar
Tool: Hammer (min quality: Common)
World Object: workbench
Slots Required:
  - blade: scimitar_blade
  - handle: handle
  - binding: binding
```

#### Create Simple Instance
```
new <item_id>
```
Creates a new Simple item instance (no crafting, for testing).

**Example:**
```
new iron_bar

Created instance:
{
  "id": 1001,
  "definition": "iron_bar",
  "provenance": { ... }
}
```

#### Show Instance
```
show_instance <instance_id>
```
Displays instance details including components and provenance.

---

## API Reference

### Core Types

#### IDs
```rust
pub struct ItemId(pub String);
pub struct RecipeId(pub String);
pub struct MaterialId(pub String);
pub struct SubmaterialId(pub String);
pub struct ComponentKindId(pub String);
pub struct ItemInstanceId(pub u64);
pub struct WorldObjectInstanceId(pub u64);
```

#### Materials
```rust
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub description: String,
}

pub struct Submaterial {
    pub id: SubmaterialId,
    pub material: MaterialId,  // Parent material
    pub name: String,
    pub description: String,
}

pub struct ComponentKind {
    pub id: ComponentKindId,
    pub name: String,
    pub description: String,
    pub accepted_materials: Vec<MaterialId>,  // OR logic
    pub makeshift_tags: Vec<String>,          // Gameplay substitution
}
```

#### Item Definitions
```rust
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Simple { submaterial: Option<SubmaterialId> },
    Component { component_kind: ComponentKindId },
    Composite(CompositeDef),
}

pub struct CompositeDef {
    pub slots: Vec<CompositeSlot>,
    pub category: CompositeCategory,
    pub tool_type: Option<ToolType>,
}

pub struct CompositeSlot {
    pub name: String,
    pub component_kind: ComponentKindId,
}

pub enum CompositeCategory {
    Tool,
    Weapon,
    Armor,
}

pub enum ToolType {
    Pickaxe,
    Axe,
    Hatchet,
    Hammer,
    Knife,
    Saw,
    Needle,
    Custom(String),
}
```

#### Recipes
```rust
pub struct SimpleRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub output_quantity: u32,
    pub inputs: Vec<SimpleInput>,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

pub struct SimpleInput {
    pub item_id: ItemId,
    pub quantity: u32,
}

pub struct ComponentRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ComponentKindId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

pub struct CompositeRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

pub struct ToolRequirement {
    pub tool_type: ToolType,
    pub min_quality: Quality,
}

pub struct WorldObjectRequirement {
    pub kind: Option<WorldObjectKind>,
    pub required_tags: Vec<WorldObjectTag>,
}
```

#### Instances
```rust
pub enum ItemInstance {
    Simple(SimpleInstance),
    Component(ComponentInstance),
    Composite(CompositeInstance),
}

pub struct SimpleInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub provenance: Provenance,
}

pub struct ComponentInstance {
    pub id: ItemInstanceId,
    pub component_kind: ComponentKindId,
    pub submaterial: SubmaterialId,
    pub provenance: Provenance,
}

pub struct CompositeInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub quality: Quality,
    pub components: HashMap<String, ComponentInstance>,
    pub provenance: Provenance,
}
```

#### Provenance
```rust
pub struct Provenance {
    pub recipe_id: RecipeId,
    pub consumed_inputs: Vec<ConsumedInput>,
    pub tool_used: Option<ItemInstanceId>,
    pub world_object_used: Option<WorldObjectInstanceId>,
    pub crafted_at: i64,
}

pub struct ConsumedInput {
    pub instance_id: ItemInstanceId,
    pub quantity: u32,
}
```

#### Quality
```rust
pub enum Quality {
    Makeshift,
    Crude,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
```

### Registry Methods

```rust
impl Registry {
    pub fn new() -> Self;

    // Material registration
    pub fn register_material(&mut self, material: Material);
    pub fn register_submaterial(&mut self, submaterial: Submaterial);
    pub fn register_component_kind(&mut self, component_kind: ComponentKind);

    // Item registration
    pub fn register_item(&mut self, item: ItemDefinition);
    pub fn get_item(&self, id: &ItemId) -> Option<&ItemDefinition>;
    pub fn items(&self) -> impl Iterator<Item = &ItemDefinition>;

    // Recipe registration
    pub fn register_simple_recipe(&mut self, recipe: SimpleRecipe);
    pub fn register_component_recipe(&mut self, recipe: ComponentRecipe);
    pub fn register_composite_recipe(&mut self, recipe: CompositeRecipe);

    pub fn get_simple_recipe(&self, id: &RecipeId) -> Option<&SimpleRecipe>;
    pub fn get_component_recipe(&self, id: &RecipeId) -> Option<&ComponentRecipe>;
    pub fn get_composite_recipe(&self, id: &RecipeId) -> Option<&CompositeRecipe>;

    // Instance management
    pub fn add_instance(&mut self, instance: ItemInstance) -> ItemInstanceId;
    pub fn get_instance(&self, id: &ItemInstanceId) -> Option<&ItemInstance>;
}
```

---

## Design Patterns and Best Practices

### 1. Naming Conventions

**Materials**: Lowercase, singular, generic category
- ✅ `leather`, `wood`, `metal`
- ❌ `leathers`, `Metal`, `iron`

**Submaterials**: Lowercase with material suffix
- ✅ `deer_leather`, `oak_wood`, `iron_metal`
- ❌ `deer`, `oak`, `iron`

**Component Kinds**: Lowercase, descriptive function
- ✅ `handle`, `binding`, `scimitar_blade`
- ❌ `HandleComponent`, `blade`

**Item IDs**: Match submaterial ID for submaterial items, descriptive for others
- ✅ `iron_bar` (submaterial: iron_metal), `health_potion`
- ❌ `iron` (ambiguous), `potion1`

### 2. Material Acceptance Rules

Be thoughtful about which materials a component accepts:
- **Narrow acceptance**: `scimitar_blade` accepts only `metal` (requires hardness)
- **Broad acceptance**: `handle` accepts `wood` OR `bone` (flexibility OK)
- **Creative acceptance**: `pickaxe_head` accepts `metal` OR `stone` (makeshift support)

### 3. Quality System (TODO)

Currently, all composites default to `Quality::Common`. Future implementation should:
- Calculate quality from component submaterials
- Consider tool quality used in crafting
- Factor in world object quality (blessed forge, etc.)

**Placeholder:**
```rust
// TODO: Implement quality calculation
pub quality: Quality,  // defaults to Common
```

### 4. Provenance Queries

Design provenance queries to traverse the chain:

**Example: Find all swords with steel blades**
```rust
fn find_steel_blade_swords(registry: &Registry) -> Vec<ItemInstanceId> {
    registry.instances()
        .filter_map(|inst| match inst {
            ItemInstance::Composite(composite) => {
                let blade = composite.components.get("blade")?;
                if blade.submaterial.0 == "steel_metal" {
                    Some(composite.id)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}
```

**Example: Find items crafted at specific forge**
```rust
fn find_items_from_forge(
    registry: &Registry,
    forge_id: WorldObjectInstanceId
) -> Vec<ItemInstanceId> {
    registry.instances()
        .filter(|inst| {
            inst.provenance().world_object_used == Some(forge_id)
        })
        .map(|inst| inst.id())
        .collect()
}
```

### 5. Extensibility

The system is designed for LLM content generation:
- String-based IDs allow natural language → ID conversion
- `ToolType::Custom(String)` supports arbitrary tool types
- Material and submaterial systems are open-ended
- Component kinds can be added dynamically

---

## Migration Guide (From Old Tag-Based System)

If you're migrating from the old tag-based system:

### Old System → New System Mapping

| Old Concept | New Concept |
|-------------|-------------|
| `MaterialTag` | `MaterialId` (categories) and `SubmaterialId` (variants) |
| `ItemCategories` (is_material, is_tool, etc.) | `ItemKind` enum (Simple, Component, Composite) |
| `ComponentSlot.accepted_tags` | `ComponentKind.accepted_materials` |
| `MaterialInput.fills_slot` | Implicit from ComponentRecipe (one input) |
| `MaterialInput.provenance_reqs` | Removed from recipes (query at runtime) |
| Single `Recipe` type | Three types: SimpleRecipe, ComponentRecipe, CompositeRecipe |
| Single `ItemInstance` type | Three types: SimpleInstance, ComponentInstance, CompositeInstance |

### Key Changes

1. **Items are no longer multi-category**: Old system allowed `is_material && is_tool`. New system: item is either Simple, Component, or Composite.

2. **Material hierarchy is explicit**: Old system used tags. New system: Material → Submaterial → Item.

3. **Recipes are specialized**: Old system had one Recipe type with complex MaterialInput. New system: three recipe types with clear purposes.

4. **Provenance requirements decoupled**: Old system embedded recursive provenance requirements in recipes. New system: track provenance, query at runtime.

5. **Component validation simplified**: Old system matched tags at craft time. New system: ComponentKind defines accepted materials upfront.

---

## Future Enhancements

### Quality Calculation (TODO)
Implement quality calculation for composites based on:
- Component submaterial qualities
- Tool quality used
- World object quality (enchanted workbench, etc.)
- Craftsman skill level

### Stat Modifiers (TODO)
Submaterials should grant stat modifiers:
- `steel_metal`: +5 durability, +2 damage
- `yew_wood`: +10% attack speed for handles
- `diamond_gem`: +20% quality for pommels

### Makeshift Crafting (TODO)
Expand makeshift_tags system:
- Allow crafting with makeshift substitutions
- Apply quality penalties (Makeshift quality)
- Support early-game progression

### Recursive Provenance Queries (TODO)
Add helper methods for complex provenance queries:
- "Find all items that trace back to X"
- "Show all items using tool Y anywhere in their chain"
- "List all items from materials gathered at location Z"

---

## File Structure

```
src/
├── lib.rs              # Module exports
├── ids.rs              # ID types (ItemId, MaterialId, SubmaterialId, etc.)
├── materials.rs        # Material, Submaterial, ComponentKind
├── item_def.rs         # ItemDefinition, ItemKind, CompositeDef
├── recipe.rs           # SimpleRecipe, ComponentRecipe, CompositeRecipe
├── instance.rs         # SimpleInstance, ComponentInstance, CompositeInstance
├── provenance.rs       # Provenance, ConsumedInput
├── quality.rs          # Quality enum
├── world_object.rs     # WorldObjectKind
├── registry.rs         # Registry for definitions and instances
├── content.rs          # Sample content registration
├── cli.rs              # Interactive CLI
└── main.rs             # CLI entry point

tests/
└── cli_tests.rs        # Integration tests

REFACTOR_PLAN.md        # Detailed refactor specification
progress.md             # Refactor progress tracking
README.md               # This file
```

---

## Contributing

When modifying this system:

1. **Maintain strict tier separation**: Simple → Component → Composite, no shortcuts
2. **Preserve provenance tracking**: Every instance must track complete crafting history
3. **Validate material acceptance**: ComponentKind validation must be enforced
4. **Keep it isolated**: This subsystem should not depend on stats, combat, or other game systems
5. **Update tests**: Add tests for new features
6. **Update this README**: Keep documentation current

---

## License

[Your License Here]

---

## Questions?

For questions about this architecture:
- See `REFACTOR_PLAN.md` for detailed design rationale
- See `progress.md` for implementation status
- See example content in `src/content.rs`
- Run the CLI with `cargo run` to explore interactively
