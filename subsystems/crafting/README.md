# Crafting Subsystem

A crafting system with tag-based material compatibility, multi-component items, and full provenance tracking.

## Features

- **Tag-based material compatibility**: Materials have tags (e.g., `metal`, `wood`), and component slots accept materials with matching tags
- **Multi-component items**: Tinker's Construct-style items with named slots (blade, handle, pommel)
- **Quality tiers**: Makeshift → Crude → Common → Uncommon → Rare → Epic → Legendary
- **Provenance tracking**: Full traceability of crafting chains for quests and lore
- **LLM-friendly**: String-based IDs designed for content generation

## Architecture

```
ItemDefinition  ─┬─> ComponentSlot ─> MaterialTag
                 └─> ItemCategories (material, tool, placeable, consumable)

Recipe ─> Construction ─┬─> ToolRequirement
                        ├─> WorldObjectKind (ResourceNode | CraftingStation)
                        └─> MaterialInput

ItemInstance ─┬─> ComponentInstance (slot → material)
              └─> Provenance (immediate inputs, tool used, world object)
```

## Modules

- `ids` - Identifier types (ItemId, RecipeId, MaterialTag, etc.)
- `quality` - Quality tier enum
- `world_object` - ResourceNode and CraftingStation types
- `item_def` - Item definitions with component slots and categories
- `recipe` - Recipes, constructions, and material inputs
- `instance` - Runtime item instances
- `provenance` - Crafting history tracking
- `registry` - Central storage for definitions and instances

## Usage

```rust
use crafting::{ItemDefinition, Recipe, Registry, Quality, MaterialTag};

let mut registry = Registry::new();

// Register item definitions, recipes, then craft instances
// Provenance is tracked automatically through the crafting chain
```
