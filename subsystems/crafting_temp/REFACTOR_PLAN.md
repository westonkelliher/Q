# Tinker's Construct Style Crafting Refactor

## Executive Summary

Refactor the crafting system to use a clean Material/Submaterial hierarchy and strict 3-tier item flow. This eliminates the current tag-soup complexity where items can be both materials and tools simultaneously.

**Current state**: Complex tag-based system with `ItemCategories` flags, `MaterialTag` matching, and items that can be multiple things at once.

**Target state**: Clean hierarchy where items are exactly ONE of: Simple, Component, or Composite.

---

## Core Concepts

### Material vs Submaterial

**Materials** are broad categories:
- `leather`, `wood`, `metal`, `gem`, `bone`, `fiber`

**Submaterials** are specific variants that belong to a material:
| Material | Submaterials |
|----------|--------------|
| leather | deer_leather, bear_leather, wolf_leather |
| wood | oak_wood, yew_wood, elder_wood |
| metal | iron_metal, bronze_metal, steel_metal, silver_metal |
| gem | sapphire_gem, diamond_gem, ruby_gem |
| bone | wolf_bone, deer_bone |
| fiber | plant_fiber, sinew |

**Key insight**: "Material" is NOT a type of item. Items are *of* a submaterial, and submaterials belong to a material category.

### Three Item Kinds (Mutually Exclusive)

An item is exactly ONE of:

1. **Simple** - standalone items with no assembly structure:
   - Submaterial items: `deer_leather`, `oak_wood`, `iron_metal`
   - Consumables: `cooked_meat`, `health_potion`
   - Creatures: `wolf`, `deer`
   - Resource nodes: `copper_boulder`, `oak_tree`
   - Carcasses: `wolf_carcass`

2. **Component** - parts made from a submaterial, used exclusively as inputs to composites:
   - `binding` (made from leather submaterials)
   - `handle` (made from wood/bone submaterials)
   - `scimitar_blade` (made from metal submaterials)
   - Each component instance tracks which specific submaterial was used

3. **Composite** - assembled from components, the final craftable items:
   - `scimitar` (requires: scimitar_blade + handle + binding)
   - `pickaxe` (requires: pickaxe_head + handle)
   - Tools, weapons, armor
   - **Composites do NOT have a material** - they have components, each with their own submaterial

### Crafting Flow

```
Submaterial Item  →  Component  →  Composite
(deer_leather)       (binding)     (scimitar)
```

- Simple items (submaterials) are inputs to Component recipes
- Components are inputs to Composite recipes
- No shortcuts: you can't use a submaterial directly in a composite

### Component Kinds

Component definitions specify what materials (not submaterials) they accept:

```rust
ComponentKind {
    id: "handle",
    accepted_materials: vec![wood, bone],  // OR logic
}

ComponentKind {
    id: "binding",
    accepted_materials: vec![leather, fiber],
}

ComponentKind {
    id: "scimitar_blade",
    accepted_materials: vec![metal],
}
```

When crafting a component, you provide a submaterial item. The system validates that the submaterial's parent material is in the accepted list.

### Makeshift Tags

Some components can substitute for tools in non-crafting scenarios:
- `knife_blade` has `makeshift_tags: ["knife"]` - can act as a makeshift knife
- This does NOT affect crafting recipes, only gameplay substitution

---

## Data Structures

### New ID Types (ids.rs)

```rust
pub struct MaterialId(pub String);      // "leather", "wood", "metal"
pub struct SubmaterialId(pub String);   // "deer_leather", "oak_wood"
pub struct ComponentKindId(pub String); // "handle", "binding", "scimitar_blade"
```

Keep existing:
- `ItemId`, `RecipeId`, `ItemInstanceId`, `WorldObjectInstanceId`
- `ResourceNodeId`, `CraftingStationId`

Remove:
- `MaterialTag` (replaced by MaterialId/SubmaterialId)
- `WorldObjectTag` (can keep if needed for world object requirements)

### Material System (new file: materials.rs)

```rust
/// Broad material category
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub description: String,
}

/// Specific variant - these correspond to actual Simple items
pub struct Submaterial {
    pub id: SubmaterialId,
    pub material: MaterialId,  // parent category
    pub name: String,
    pub description: String,
    // Future: properties, stat modifiers, etc.
}

/// Defines a type of component
pub struct ComponentKind {
    pub id: ComponentKindId,
    pub name: String,
    pub description: String,
    pub accepted_materials: Vec<MaterialId>,  // OR logic
    pub makeshift_tags: Vec<String>,          // what this can substitute for
}
```

### Item Definition (item_def.rs)

```rust
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
}

pub enum ItemKind {
    /// Raw materials, consumables, creatures, resource nodes
    /// If this is a submaterial item, include the submaterial ID
    Simple { submaterial: Option<SubmaterialId> },
    
    /// Parts made from submaterials, used to build composites
    Component { component_kind: ComponentKindId },
    
    /// Final assembled items (tools, weapons, armor)
    Composite(CompositeDef),
}

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

### Instances (instance.rs)

```rust
/// Instance of a Simple item (including submaterials)
pub struct SimpleInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub provenance: Provenance,
}

/// Instance of a Component - tracks which submaterial was used
pub struct ComponentInstance {
    pub id: ItemInstanceId,
    pub component_kind: ComponentKindId,
    pub submaterial: SubmaterialId,  // e.g., deer_leather, oak_wood
    pub provenance: Provenance,
}

/// Instance of a Composite - tracks which components were used
pub struct CompositeInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,
    pub quality: Quality,  // TODO: quality calculation, default to Common
    pub components: HashMap<String, ComponentInstance>,  // slot_name -> component
    pub provenance: Provenance,
}
```

### Recipes (recipe.rs)

Simplified recipe types:

```rust
/// Recipe to create a Simple item (mining, harvesting, smelting)
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

/// Recipe to craft a Component from a submaterial
pub struct ComponentRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ComponentKindId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Input is implicit: one submaterial item whose material is in ComponentKind.accepted_materials
}

/// Recipe to assemble a Composite from components
pub struct CompositeRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Inputs are implicit: whatever ComponentKinds the composite's slots require
}
```

### Provenance (provenance.rs)

Keep the existing structure - provenance tracking is still important:

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

---

## What to Remove

From the current codebase, remove:

1. **ItemCategories struct** - replaced by ItemKind enum
2. **MaterialTag** - replaced by MaterialId/SubmaterialId
3. **ComponentSlot on items** - moved to CompositeDef
4. **ComponentSlot.accepted_tags, required_tags, optional_tags** - replaced by ComponentKind.accepted_materials
5. **MaterialInput.fills_slot** - slots inferred from component kinds
6. **MaterialInput.component_reqs** - not needed, component kinds handle this
7. **MaterialInput.provenance_reqs** - removed from crafting (other systems can query provenance)
8. **ProvenanceRequirements struct** - removed from crafting
9. **quality_for_slot on items** - removed
10. **Complex QualityFormula variants** - just use Fixed(Common) with TODO comment

---

## Quality System

For this refactor, simplify quality:
- Only Composite items have quality
- Default all to `Quality::Common`
- Add TODO comment for future quality calculation

```rust
// TODO: Implement quality calculation based on component submaterials
pub quality: Quality,  // defaults to Common
```

---

## Provenance Tracking

**Important**: We keep full provenance tracking. The crafting system's job is to:

1. **Track provenance faithfully** - every instance records recipe, tool, world object, consumed inputs
2. **Make data queryable** - other systems (rituals, quests) can traverse the chain

Example provenance chain:
```
scimitar instance
  → provenance.consumed_inputs → [blade_instance, handle_instance, binding_instance]
  
binding_instance  
  → submaterial: deer_leather
  → provenance.consumed_inputs → [deer_leather_instance]
  
deer_leather_instance
  → provenance.world_object → wolf_carcass_world_object_instance
  → provenance.tool → knife_instance (used for harvesting)

wolf_carcass_world_object_instance
  → provenance.tool → sword_instance (weapon that killed the wolf)
```

Other systems can query: "Show me all items that trace back to a wolf killed with a manasteel blade."

---

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `src/ids.rs` | Modify | Add MaterialId, SubmaterialId, ComponentKindId; remove MaterialTag |
| `src/materials.rs` | Create | Material, Submaterial, ComponentKind definitions |
| `src/item_def.rs` | Rewrite | ItemKind enum, CompositeDef, CompositeSlot; remove ItemCategories, old ComponentSlot |
| `src/recipe.rs` | Rewrite | SimpleRecipe, ComponentRecipe, CompositeRecipe; remove MaterialInput complexity |
| `src/instance.rs` | Rewrite | SimpleInstance, ComponentInstance, CompositeInstance |
| `src/provenance.rs` | Keep | Minimal changes, remove ProvenanceRequirements if present |
| `src/registry.rs` | Modify | Register materials, submaterials, component kinds; update item/recipe handling |
| `src/content.rs` | Rewrite | All sample content using new structure |
| `src/lib.rs` | Modify | Update exports |
| `src/quality.rs` | Keep | No changes needed |
| `src/world_object.rs` | Keep | No changes needed |
| `tests/cli_tests.rs` | Update | Fix tests for new structure |
| `README.md` | Rewrite | Document new architecture |

---

## Sample Content

### Materials
```rust
Material { id: "leather", name: "Leather", ... }
Material { id: "wood", name: "Wood", ... }
Material { id: "metal", name: "Metal", ... }
Material { id: "gem", name: "Gem", ... }
Material { id: "bone", name: "Bone", ... }
Material { id: "fiber", name: "Fiber", ... }
```

### Submaterials
```rust
Submaterial { id: "deer_leather", material: "leather", ... }
Submaterial { id: "wolf_leather", material: "leather", ... }
Submaterial { id: "oak_wood", material: "wood", ... }
Submaterial { id: "yew_wood", material: "wood", ... }
Submaterial { id: "iron_metal", material: "metal", ... }
Submaterial { id: "bronze_metal", material: "metal", ... }
Submaterial { id: "steel_metal", material: "metal", ... }
```

### Component Kinds
```rust
ComponentKind { 
    id: "handle", 
    accepted_materials: vec!["wood", "bone"],
    makeshift_tags: vec![],
}
ComponentKind { 
    id: "binding", 
    accepted_materials: vec!["leather", "fiber"],
    makeshift_tags: vec![],
}
ComponentKind { 
    id: "scimitar_blade", 
    accepted_materials: vec!["metal"],
    makeshift_tags: vec![],
}
ComponentKind { 
    id: "knife_blade", 
    accepted_materials: vec!["metal"],
    makeshift_tags: vec!["knife"],  // can substitute for knife tool
}
ComponentKind { 
    id: "pickaxe_head", 
    accepted_materials: vec!["metal", "stone"],
    makeshift_tags: vec![],
}
```

### Simple Items (submaterial items)
```rust
ItemDefinition {
    id: "deer_leather",
    name: "Deer Leather",
    kind: ItemKind::Simple { submaterial: Some("deer_leather") },
}
ItemDefinition {
    id: "oak_wood",
    name: "Oak Wood",
    kind: ItemKind::Simple { submaterial: Some("oak_wood") },
}
ItemDefinition {
    id: "iron_bar",  // Note: item ID vs submaterial ID can differ
    name: "Iron Bar",
    kind: ItemKind::Simple { submaterial: Some("iron_metal") },
}
```

### Simple Items (non-submaterial)
```rust
ItemDefinition {
    id: "wolf",
    name: "Wolf",
    kind: ItemKind::Simple { submaterial: None },
}
ItemDefinition {
    id: "cooked_meat",
    name: "Cooked Meat",
    kind: ItemKind::Simple { submaterial: None },
}
```

### Component Items
```rust
ItemDefinition {
    id: "handle",
    name: "Handle",
    kind: ItemKind::Component { component_kind: "handle" },
}
ItemDefinition {
    id: "binding",
    name: "Binding",
    kind: ItemKind::Component { component_kind: "binding" },
}
ItemDefinition {
    id: "scimitar_blade",
    name: "Scimitar Blade",
    kind: ItemKind::Component { component_kind: "scimitar_blade" },
}
```

### Composite Items
```rust
ItemDefinition {
    id: "scimitar",
    name: "Scimitar",
    kind: ItemKind::Composite(CompositeDef {
        slots: vec![
            CompositeSlot { name: "blade", component_kind: "scimitar_blade" },
            CompositeSlot { name: "handle", component_kind: "handle" },
            CompositeSlot { name: "binding", component_kind: "binding" },
        ],
        category: CompositeCategory::Weapon,
        tool_type: None,
    }),
}
ItemDefinition {
    id: "pickaxe",
    name: "Pickaxe",
    kind: ItemKind::Composite(CompositeDef {
        slots: vec![
            CompositeSlot { name: "head", component_kind: "pickaxe_head" },
            CompositeSlot { name: "handle", component_kind: "handle" },
        ],
        category: CompositeCategory::Tool,
        tool_type: Some(ToolType::Pickaxe),
    }),
}
```

---

## Key Constraints (Validation Rules)

1. **Item kind is exclusive** - an item cannot be multiple kinds
2. **Components only in composites** - Component items are ONLY used as inputs to CompositeRecipes
3. **Composites only from components** - CompositeRecipes ONLY accept Component inputs
4. **Submaterials only in components** - ComponentRecipes ONLY accept Simple items with submaterials
5. **Material validation** - When crafting a component, the submaterial's parent material must be in the ComponentKind's accepted_materials list
6. **Slot matching** - When crafting a composite, each slot must be filled with a component of the matching ComponentKind

---

## Implementation Order

1. `ids.rs` - Add new ID types
2. `materials.rs` - Create new file with Material, Submaterial, ComponentKind
3. `item_def.rs` - Rewrite with ItemKind enum
4. `recipe.rs` - Simplify recipe types
5. `instance.rs` - New instance types
6. `registry.rs` - Update registration and lookup
7. `content.rs` - Rewrite sample content
8. `lib.rs` - Update exports
9. `tests/cli_tests.rs` - Fix tests
10. `README.md` - Update documentation

---

## Testing Checklist

- [ ] Can register materials and submaterials
- [ ] Can register component kinds with accepted materials
- [ ] Can register Simple items (with and without submaterial)
- [ ] Can register Component items
- [ ] Can register Composite items with slots
- [ ] ComponentRecipe validates submaterial's material is accepted
- [ ] CompositeRecipe validates component kinds match slots
- [ ] ComponentInstance tracks submaterial correctly
- [ ] CompositeInstance tracks components correctly
- [ ] Provenance chain is fully queryable
- [ ] makeshift_tags are stored on component kinds
