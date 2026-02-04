//! Progression-focused crafting content for MVP
//! 
//! Organized by progression stages:
//! Stage 0: Starting resources (stick, rock, flint)
//! Stage 1: Makeshift tools (use items directly)
//! Stage 2: Flint tools (crude quality)
//! Stage 3: Bone tools (after hunting)
//! Stage 4: Metal age (copper, bronze, iron)

use super::{
    Material, MaterialId, Submaterial, SubmaterialId, ComponentKind, ComponentKindId,
    ItemDefinition, ItemId, ItemKind, CompositeDef, CompositeSlot, CompositeCategory, ToolType,
    SimpleRecipe, ComponentRecipe, CompositeRecipe, SimpleInput, RecipeId,
    ToolRequirement, WorldObjectRequirement, Quality, CraftingRegistry,
};
use super::item_def::StatBonuses;
use super::world_object::WorldObjectKind;
use super::ids::CraftingStationId;

/// Helper to create a MaterialId
fn mat(s: &str) -> MaterialId {
    MaterialId(s.to_string())
}

/// Helper to create a SubmaterialId
fn submat(s: &str) -> SubmaterialId {
    SubmaterialId(s.to_string())
}

/// Helper to create a ComponentKindId
fn comp_kind(s: &str) -> ComponentKindId {
    ComponentKindId(s.to_string())
}

/// Helper to create an ItemId
fn item(s: &str) -> ItemId {
    ItemId(s.to_string())
}

/// Helper to create a RecipeId
fn recipe(s: &str) -> RecipeId {
    RecipeId(s.to_string())
}

/// Populate registry with progression-focused content
pub fn register_sample_content(registry: &mut CraftingRegistry) {
    register_materials(registry);
    register_submaterials(registry);
    register_component_kinds(registry);
    register_items(registry);
    register_recipes(registry);
}

fn register_materials(registry: &mut CraftingRegistry) {
    registry.register_material(Material {
        id: mat("stone"),
        name: "Stone".to_string(),
        description: "Hard rock materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("wood"),
        name: "Wood".to_string(),
        description: "Timber and wooden materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("bone"),
        name: "Bone".to_string(),
        description: "Hard skeletal material from animals".to_string(),
    });

    registry.register_material(Material {
        id: mat("fiber"),
        name: "Fiber".to_string(),
        description: "Flexible binding materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("leather"),
        name: "Leather".to_string(),
        description: "Processed hide material".to_string(),
    });

    registry.register_material(Material {
        id: mat("metal"),
        name: "Metal".to_string(),
        description: "Metallic materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("clay"),
        name: "Clay".to_string(),
        description: "Moldable earth material".to_string(),
    });

    registry.register_material(Material {
        id: mat("meat"),
        name: "Meat".to_string(),
        description: "Animal flesh for food".to_string(),
    });
}

fn register_submaterials(registry: &mut CraftingRegistry) {
    // =========================================================================
    // STAGE 0-1: Starting Materials
    // =========================================================================
    
    registry.register_submaterial(Submaterial {
        id: submat("flint_stone"),
        material: mat("stone"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone for knapping".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("plant_fiber"),
        material: mat("fiber"),
        name: "Plant Fiber".to_string(),
        description: "Natural fibers from plants".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("clay_lump"),
        material: mat("clay"),
        name: "Clay".to_string(),
        description: "Wet clay for building".to_string(),
    });

    // =========================================================================
    // STAGE 2: Flint & Animal Materials
    // =========================================================================
    
    // Flint processed materials
    registry.register_submaterial(Submaterial {
        id: submat("flint_blade"),
        material: mat("stone"),
        name: "Flint Blade".to_string(),
        description: "Sharp knapped flint blade".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("flint_axe_head"),
        material: mat("stone"),
        name: "Flint Axe Head".to_string(),
        description: "Knapped axe head".to_string(),
    });

    // Wolf materials
    registry.register_submaterial(Submaterial {
        id: submat("wolf_bone"),
        material: mat("bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense bone from a wolf".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("wolf_sinew"),
        material: mat("fiber"),
        name: "Wolf Sinew".to_string(),
        description: "Strong animal tendon".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("wolf_hide"),
        material: mat("leather"),
        name: "Wolf Hide".to_string(),
        description: "Untanned wolf pelt".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("wolf_meat"),
        material: mat("meat"),
        name: "Wolf Meat".to_string(),
        description: "Raw wolf meat".to_string(),
    });

    // Deer materials
    registry.register_submaterial(Submaterial {
        id: submat("deer_bone"),
        material: mat("bone"),
        name: "Deer Bone".to_string(),
        description: "Light deer bone".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("deer_sinew"),
        material: mat("fiber"),
        name: "Deer Sinew".to_string(),
        description: "Flexible animal tendon".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("deer_hide"),
        material: mat("leather"),
        name: "Deer Hide".to_string(),
        description: "Soft untanned deer pelt".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("deer_meat"),
        material: mat("meat"),
        name: "Deer Meat".to_string(),
        description: "Raw deer meat".to_string(),
    });

    // =========================================================================
    // STAGE 3: Wood Processing
    // =========================================================================
    
    registry.register_submaterial(Submaterial {
        id: submat("wood_log"),
        material: mat("wood"),
        name: "Wood Log".to_string(),
        description: "Chopped wood from a tree".to_string(),
    });

    // =========================================================================
    // STAGE 4: Metal Age
    // =========================================================================
    
    // Ores
    registry.register_submaterial(Submaterial {
        id: submat("copper_ore"),
        material: mat("metal"),
        name: "Copper Ore".to_string(),
        description: "Raw copper ore for smelting".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("tin_ore"),
        material: mat("metal"),
        name: "Tin Ore".to_string(),
        description: "Raw tin ore, found in mountains".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("iron_ore"),
        material: mat("metal"),
        name: "Iron Ore".to_string(),
        description: "Raw iron ore".to_string(),
    });

    // Bars
    registry.register_submaterial(Submaterial {
        id: submat("copper_bar"),
        material: mat("metal"),
        name: "Copper Bar".to_string(),
        description: "Smelted copper bar".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("bronze_bar"),
        material: mat("metal"),
        name: "Bronze Bar".to_string(),
        description: "Alloyed bronze bar (copper + tin)".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("iron_bar"),
        material: mat("metal"),
        name: "Iron Bar".to_string(),
        description: "Smelted iron bar".to_string(),
    });
}

fn register_component_kinds(registry: &mut CraftingRegistry) {
    // Handles (different sizes for different tools)
    registry.register_component_kind(ComponentKind {
        id: comp_kind("handle"),
        name: "Handle".to_string(),
        description: "Tool grip, can be made from wood or bone".to_string(),
        accepted_materials: vec![mat("wood"), mat("bone")],
        makeshift_tags: vec![],
    });

    // Bindings (secure components together)
    registry.register_component_kind(ComponentKind {
        id: comp_kind("binding"),
        name: "Binding".to_string(),
        description: "Wrapping to secure tool components".to_string(),
        accepted_materials: vec![mat("fiber"), mat("leather")],
        makeshift_tags: vec![],
    });

    // Tool heads (different for each tool type)
    registry.register_component_kind(ComponentKind {
        id: comp_kind("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Cutting edge for a knife".to_string(),
        accepted_materials: vec![mat("stone"), mat("bone"), mat("metal")],
        makeshift_tags: vec!["knife".to_string()], // Blade alone can act as makeshift knife
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("axe_head"),
        name: "Axe Head".to_string(),
        description: "Chopping head for an axe".to_string(),
        accepted_materials: vec![mat("stone"), mat("bone"), mat("metal")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Mining head for a pickaxe".to_string(),
        accepted_materials: vec![mat("bone"), mat("metal")],
        makeshift_tags: vec![],
    });
}

fn register_items(registry: &mut CraftingRegistry) {
    // =========================================================================
    // WORLD OBJECTS - Starting resources available in the world
    // =========================================================================
    
    registry.register_item(ItemDefinition {
        id: item("stick"),
        name: "Stick".to_string(),
        description: "A fallen branch. Can be used as makeshift shovel or weapon.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses { health: 0, attack: 1, defense: 0, accuracy: 0, evasion: 0 },
    });

    registry.register_item(ItemDefinition {
        id: item("rock"),
        name: "Rock".to_string(),
        description: "A large stone. Can be used as makeshift hammer or knapping tool.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("flint"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone perfect for knapping into blades.".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("flint_stone")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("tree"),
        name: "Tree".to_string(),
        description: "A living tree that can be chopped for wood".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: false,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("plant_fiber"),
        name: "Plant Fiber".to_string(),
        description: "Natural plant fibers for binding".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("plant_fiber")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("clay"),
        name: "Clay".to_string(),
        description: "Wet clay, useful for building".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("clay_lump")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Carcasses
    registry.register_item(ItemDefinition {
        id: item("wolf_carcass"),
        name: "Wolf Carcass".to_string(),
        description: "Remains of a slain wolf. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("deer_carcass"),
        name: "Deer Carcass".to_string(),
        description: "Remains of a slain deer. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("rabbit_carcass"),
        name: "Rabbit Carcass".to_string(),
        description: "Remains of a slain rabbit. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("fox_carcass"),
        name: "Fox Carcass".to_string(),
        description: "Remains of a slain fox. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("spider_carcass"),
        name: "Spider Carcass".to_string(),
        description: "Remains of a slain spider. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("snake_carcass"),
        name: "Snake Carcass".to_string(),
        description: "Remains of a slain snake. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("lion_carcass"),
        name: "Lion Carcass".to_string(),
        description: "Remains of a slain lion. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("dragon_carcass"),
        name: "Dragon Carcass".to_string(),
        description: "Remains of a slain dragon. Can be processed for materials.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // =========================================================================
    // PROCESSED MATERIALS - From recipes
    // =========================================================================
    
    // Flint products
    registry.register_item(ItemDefinition {
        id: item("flint_blade"),
        name: "Flint Blade".to_string(),
        description: "Knapped flint blade, sharp but fragile.".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("flint_blade")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("flint_axe_head"),
        name: "Flint Axe Head".to_string(),
        description: "Knapped axe head for chopping".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("flint_axe_head")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Animal products (from processing carcasses)
    registry.register_item(ItemDefinition {
        id: item("wolf_bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense wolf bone, good for tools".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_bone")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("wolf_sinew"),
        name: "Wolf Sinew".to_string(),
        description: "Strong animal tendon for binding".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_sinew")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("wolf_hide"),
        name: "Wolf Hide".to_string(),
        description: "Untanned wolf pelt".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_hide")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("wolf_meat"),
        name: "Wolf Meat".to_string(),
        description: "Raw wolf meat, can be cooked".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_meat")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("deer_bone"),
        name: "Deer Bone".to_string(),
        description: "Light deer bone".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_bone")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("deer_sinew"),
        name: "Deer Sinew".to_string(),
        description: "Flexible animal tendon".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_sinew")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("deer_hide"),
        name: "Deer Hide".to_string(),
        description: "Soft untanned deer pelt".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_hide")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("deer_meat"),
        name: "Deer Meat".to_string(),
        description: "Raw deer meat, can be cooked".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_meat")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Wood products
    registry.register_item(ItemDefinition {
        id: item("wood_log"),
        name: "Wood Log".to_string(),
        description: "Chopped wood from a tree".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wood_log")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Metal ores
    registry.register_item(ItemDefinition {
        id: item("copper_ore"),
        name: "Copper Ore".to_string(),
        description: "Raw copper ore".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("copper_ore")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("tin_ore"),
        name: "Tin Ore".to_string(),
        description: "Raw tin ore, found in mountains".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("tin_ore")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("iron_ore"),
        name: "Iron Ore".to_string(),
        description: "Raw iron ore".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("iron_ore")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Metal bars
    registry.register_item(ItemDefinition {
        id: item("copper_bar"),
        name: "Copper Bar".to_string(),
        description: "Smelted copper bar".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("copper_bar")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("bronze_bar"),
        name: "Bronze Bar".to_string(),
        description: "Alloyed bronze bar (copper + tin)".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("bronze_bar")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("iron_bar"),
        name: "Iron Bar".to_string(),
        description: "Smelted iron bar".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("iron_bar")) },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // =========================================================================
    // CRAFTING STATIONS - Placeable structures for advanced crafting
    // =========================================================================
    
    registry.register_item(ItemDefinition {
        id: item("forge"),
        name: "Forge".to_string(),
        description: "A high-heat crafting station for smelting metal ores into bars.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("workbench"),
        name: "Workbench".to_string(),
        description: "A sturdy work surface for precise crafting.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: Some(WorldObjectKind::CraftingStation(CraftingStationId("workbench".to_string()))),
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("anvil"),
        name: "Anvil".to_string(),
        description: "A heavy iron anvil for forging metal tools and weapons.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
        placeable: Some(WorldObjectKind::CraftingStation(CraftingStationId("anvil".to_string()))),
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // =========================================================================
    // COMPONENTS - Parts for assembling tools
    // =========================================================================
    
    registry.register_item(ItemDefinition {
        id: item("handle"),
        name: "Handle".to_string(),
        description: "Tool handle, made from wood or bone".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("handle") },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("binding"),
        name: "Binding".to_string(),
        description: "Binding to secure tool parts".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("binding") },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Blade for a knife".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("knife_blade") },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("axe_head"),
        name: "Axe Head".to_string(),
        description: "Head for an axe".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("axe_head") },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    registry.register_item(ItemDefinition {
        id: item("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Head for a pickaxe".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("pickaxe_head") },
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // =========================================================================
    // COMPOSITE TOOLS - Assembled from components
    // =========================================================================
    
    // Knives
    registry.register_item(ItemDefinition {
        id: item("knife"),
        name: "Knife".to_string(),
        description: "Multi-purpose cutting tool".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "blade".to_string(),
                    component_kind: comp_kind("knife_blade"),
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind("handle"),
                },
                CompositeSlot {
                    name: "binding".to_string(),
                    component_kind: comp_kind("binding"),
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Knife),
        }),
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Axes
    registry.register_item(ItemDefinition {
        id: item("axe"),
        name: "Axe".to_string(),
        description: "Woodcutting tool".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "head".to_string(),
                    component_kind: comp_kind("axe_head"),
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind("handle"),
                },
                CompositeSlot {
                    name: "binding".to_string(),
                    component_kind: comp_kind("binding"),
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Axe),
        }),
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });

    // Pickaxes
    registry.register_item(ItemDefinition {
        id: item("pickaxe"),
        name: "Pickaxe".to_string(),
        description: "Mining tool for breaking rock and ore".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "head".to_string(),
                    component_kind: comp_kind("pickaxe_head"),
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind("handle"),
                },
                CompositeSlot {
                    name: "binding".to_string(),
                    component_kind: comp_kind("binding"),
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Pickaxe),
        }),
        placeable: None,
        pickupable: true,
        stat_bonuses: StatBonuses::default(),
    });
}

fn register_recipes(registry: &mut CraftingRegistry) {
    // =========================================================================
    // STAGE 1: CRAFTING STATIONS - Build infrastructure for advanced crafting
    // =========================================================================
    
    // Build a simple forge from rocks and clay
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("build_forge"),
        name: "Build Forge".to_string(),
        output: item("forge"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("rock"),
                quantity: 5,
            },
            SimpleInput {
                item_id: item("clay"),
                quantity: 3,
            },
        ],
        tool: None,
        world_object: None,
    });
    
    // Build a workbench from wood logs
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("build_workbench"),
        name: "Build Workbench".to_string(),
        output: item("workbench"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("wood_log"),
                quantity: 4,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Axe,
            min_quality: Quality::Crude,
        }),
        world_object: None,
    });
    
    // Build an anvil from iron bars (requires forge first)
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("build_anvil"),
        name: "Build Anvil".to_string(),
        output: item("anvil"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("iron_bar"),
                quantity: 3,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Crude,
        }),
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![],
        }),
    });

    // =========================================================================
    // STAGE 2: FLINT KNAPPING (Crude Quality)
    // =========================================================================
    
    // Knap flint into blade (using rock as makeshift hammer)
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("knap_flint_blade"),
        name: "Knap Flint Blade".to_string(),
        output: item("flint_blade"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("flint"),
                quantity: 1,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Makeshift, // Rock counts as makeshift hammer
        }),
        world_object: None,
    });

    // Knap flint into axe head
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("knap_flint_axe_head"),
        name: "Knap Flint Axe Head".to_string(),
        output: item("flint_axe_head"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("flint"),
                quantity: 1,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Makeshift,
        }),
        world_object: None,
    });

    // =========================================================================
    // COMPONENT RECIPES - Create components from submaterials
    // =========================================================================
    
    // Create handle from stick (wood)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_stick_handle"),
        name: "Craft Handle from Stick".to_string(),
        output: comp_kind("handle"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Makeshift, // Flint blade can act as makeshift knife
        }),
        world_object: None,
    });

    // Create handle from bone
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_bone_handle"),
        name: "Craft Handle from Bone".to_string(),
        output: comp_kind("handle"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Makeshift,
        }),
        world_object: None,
    });

    // Create binding from plant fiber
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_plant_fiber_binding"),
        name: "Craft Binding from Plant Fiber".to_string(),
        output: comp_kind("binding"),
        tool: None, // Can twist fibers by hand
        world_object: None,
    });

    // Create binding from sinew
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_sinew_binding"),
        name: "Craft Binding from Sinew".to_string(),
        output: comp_kind("binding"),
        tool: None, // Can process by hand
        world_object: None,
    });

    // Create knife blade from flint
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_flint_knife_blade"),
        name: "Craft Knife Blade from Flint".to_string(),
        output: comp_kind("knife_blade"),
        tool: None, // Uses the knapped flint blade directly
        world_object: None,
    });

    // Create axe head from flint
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_flint_axe_head"),
        name: "Craft Axe Head from Flint".to_string(),
        output: comp_kind("axe_head"),
        tool: None, // Uses the knapped flint axe head directly
        world_object: None,
    });

    // Create pickaxe head from bone
    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_bone_pickaxe_head"),
        name: "Craft Pickaxe Head from Bone".to_string(),
        output: comp_kind("pickaxe_head"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Crude, // Need proper knife
        }),
        world_object: None,
    });

    // =========================================================================
    // COMPOSITE RECIPES - Assemble tools from components
    // =========================================================================
    
    // Assemble knife
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_knife"),
        name: "Assemble Knife".to_string(),
        output: item("knife"),
        tool: None, // Just assembly
        world_object: None,
    });

    // Assemble axe
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_axe"),
        name: "Assemble Axe".to_string(),
        output: item("axe"),
        tool: None,
        world_object: None,
    });

    // Assemble pickaxe
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_pickaxe"),
        name: "Assemble Pickaxe".to_string(),
        output: item("pickaxe"),
        tool: None,
        world_object: None,
    });

    // =========================================================================
    // STAGE 3: PROCESSING & CRAFTING (with tools)
    // =========================================================================
    
    // Process wolf carcass (requires knife)
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("process_wolf_carcass"),
        name: "Process Wolf Carcass".to_string(),
        output: item("wolf_bone"),
        output_quantity: 2,
        inputs: vec![
            SimpleInput {
                item_id: item("wolf_carcass"),
                quantity: 1,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Makeshift,
        }),
        world_object: None,
    });

    // Process deer carcass (requires knife)
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("process_deer_carcass"),
        name: "Process Deer Carcass".to_string(),
        output: item("deer_bone"),
        output_quantity: 2,
        inputs: vec![
            SimpleInput {
                item_id: item("deer_carcass"),
                quantity: 1,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Makeshift,
        }),
        world_object: None,
    });

    // Chop tree (requires axe)
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("chop_tree"),
        name: "Chop Tree".to_string(),
        output: item("wood_log"),
        output_quantity: 4,
        inputs: vec![
            SimpleInput {
                item_id: item("tree"),
                quantity: 1,
            },
        ],
        tool: Some(ToolRequirement {
            tool_type: ToolType::Axe,
            min_quality: Quality::Crude,
        }),
        world_object: None,
    });

    // =========================================================================
    // STAGE 4: METAL SMELTING (requires forge)
    // =========================================================================
    
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("smelt_copper_bar"),
        name: "Smelt Copper Bar".to_string(),
        output: item("copper_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("copper_ore"),
                quantity: 2,
            },
        ],
        tool: None,
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![],
        }),
    });

    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("smelt_bronze_bar"),
        name: "Smelt Bronze Bar".to_string(),
        output: item("bronze_bar"),
        output_quantity: 2,
        inputs: vec![
            SimpleInput {
                item_id: item("copper_ore"),
                quantity: 3,
            },
            SimpleInput {
                item_id: item("tin_ore"),
                quantity: 1,
            },
        ],
        tool: None,
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![],
        }),
    });

    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("smelt_iron_bar"),
        name: "Smelt Iron Bar".to_string(),
        output: item("iron_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item("iron_ore"),
                quantity: 2,
            },
        ],
        tool: None,
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![],
        }),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_sample_content() {
        let mut registry = CraftingRegistry::new();
        register_sample_content(&mut registry);

        // Verify materials
        assert!(registry.get_material(&mat("stone")).is_some());
        assert!(registry.get_material(&mat("wood")).is_some());
        assert!(registry.get_material(&mat("bone")).is_some());
        assert!(registry.get_material(&mat("metal")).is_some());

        // Verify submaterials (progression order)
        assert!(registry.get_submaterial(&submat("flint_stone")).is_some());
        assert!(registry.get_submaterial(&submat("plant_fiber")).is_some());
        assert!(registry.get_submaterial(&submat("wolf_bone")).is_some());
        assert!(registry.get_submaterial(&submat("copper_ore")).is_some());

        // Verify component kinds
        assert!(registry.get_component_kind(&comp_kind("handle")).is_some());
        assert!(registry.get_component_kind(&comp_kind("binding")).is_some());
        assert!(registry.get_component_kind(&comp_kind("knife_blade")).is_some());

        // Verify items
        assert!(registry.get_item(&item("stick")).is_some());
        assert!(registry.get_item(&item("rock")).is_some());
        assert!(registry.get_item(&item("flint")).is_some());
        assert!(registry.get_item(&item("knife")).is_some());
        assert!(registry.get_item(&item("axe")).is_some());
        assert!(registry.get_item(&item("pickaxe")).is_some());

        // Verify recipes
        assert!(registry.get_simple_recipe(&recipe("knap_flint_blade")).is_some());
        assert!(registry.get_simple_recipe(&recipe("process_wolf_carcass")).is_some());
        assert!(registry.get_component_recipe(&recipe("craft_stick_handle")).is_some());
        assert!(registry.get_composite_recipe(&recipe("assemble_knife")).is_some());
    }

    #[test]
    fn test_knife_has_three_slots() {
        let mut registry = CraftingRegistry::new();
        register_sample_content(&mut registry);

        let knife = registry.get_item(&item("knife")).unwrap();
        if let ItemKind::Composite(def) = &knife.kind {
            assert_eq!(def.slots.len(), 3);
            assert_eq!(def.slots[0].name, "blade");
            assert_eq!(def.slots[1].name, "handle");
            assert_eq!(def.slots[2].name, "binding");
        } else {
            panic!("Knife should be a composite");
        }
    }

    #[test]
    fn test_handle_component_accepts_wood_and_bone() {
        let mut registry = CraftingRegistry::new();
        register_sample_content(&mut registry);

        let handle = registry.get_component_kind(&comp_kind("handle")).unwrap();
        assert_eq!(handle.accepted_materials.len(), 2);
        assert!(handle.accepted_materials.contains(&mat("wood")));
        assert!(handle.accepted_materials.contains(&mat("bone")));
    }
}
