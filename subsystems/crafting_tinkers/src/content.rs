//! Sample content following the Material/Submaterial hierarchy
//! 
//! This implements the Tinker's Construct style crafting system where:
//! - Materials are broad categories (leather, wood, metal, etc.)
//! - Submaterials are specific variants (deer_leather, oak_wood, iron_metal, etc.)
//! - Components are crafted from submaterials (handle, binding, blade, etc.)
//! - Composites are assembled from components (sword, pickaxe, etc.)

use crate::{
    Material, MaterialId, Submaterial, SubmaterialId, ComponentKind, ComponentKindId,
    ItemDefinition, ItemId, ItemKind, CompositeDef, CompositeSlot, CompositeCategory, ToolType,
    SimpleRecipe, ComponentRecipe, CompositeRecipe, SimpleInput, RecipeId,
    ToolRequirement, WorldObjectRequirement, Quality, Registry,
};
use crate::ids::WorldObjectTag;
use crate::world_object::WorldObjectKind;
use crate::ids::CraftingStationId;

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

/// Populate registry with sample content
pub fn register_sample_content(registry: &mut Registry) {
    register_materials(registry);
    register_submaterials(registry);
    register_component_kinds(registry);
    register_items(registry);
    register_recipes(registry);
}

fn register_materials(registry: &mut Registry) {
    registry.register_material(Material {
        id: mat("leather"),
        name: "Leather".to_string(),
        description: "Flexible hide material from animals".to_string(),
    });

    registry.register_material(Material {
        id: mat("wood"),
        name: "Wood".to_string(),
        description: "Sturdy timber from trees".to_string(),
    });

    registry.register_material(Material {
        id: mat("metal"),
        name: "Metal".to_string(),
        description: "Hard metallic materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("bone"),
        name: "Bone".to_string(),
        description: "Hard skeletal material".to_string(),
    });

    registry.register_material(Material {
        id: mat("fiber"),
        name: "Fiber".to_string(),
        description: "Flexible cordage and binding materials".to_string(),
    });

    registry.register_material(Material {
        id: mat("stone"),
        name: "Stone".to_string(),
        description: "Hard rock materials".to_string(),
    });
}

fn register_submaterials(registry: &mut Registry) {
    // Leather submaterials
    registry.register_submaterial(Submaterial {
        id: submat("deer_leather"),
        material: mat("leather"),
        name: "Deer Leather".to_string(),
        description: "Soft, supple leather from deer hide".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("wolf_leather"),
        material: mat("leather"),
        name: "Wolf Leather".to_string(),
        description: "Tough, gray leather from wolf hide".to_string(),
    });

    // Wood submaterials
    registry.register_submaterial(Submaterial {
        id: submat("oak_wood"),
        material: mat("wood"),
        name: "Oak Wood".to_string(),
        description: "Hard, durable oak timber".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("yew_wood"),
        material: mat("wood"),
        name: "Yew Wood".to_string(),
        description: "Flexible, strong yew timber".to_string(),
    });

    // Metal submaterials
    registry.register_submaterial(Submaterial {
        id: submat("iron_metal"),
        material: mat("metal"),
        name: "Iron".to_string(),
        description: "Strong, common metal".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("bronze_metal"),
        material: mat("metal"),
        name: "Bronze".to_string(),
        description: "Copper-tin alloy, easier to work than iron".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("steel_metal"),
        material: mat("metal"),
        name: "Steel".to_string(),
        description: "Superior refined iron alloy".to_string(),
    });

    // Bone submaterials
    registry.register_submaterial(Submaterial {
        id: submat("wolf_bone"),
        material: mat("bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense bone from a wolf".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("deer_bone"),
        material: mat("bone"),
        name: "Deer Bone".to_string(),
        description: "Light but sturdy deer bone".to_string(),
    });

    // Fiber submaterials
    registry.register_submaterial(Submaterial {
        id: submat("plant_fiber"),
        material: mat("fiber"),
        name: "Plant Fiber".to_string(),
        description: "Twisted plant fibers".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat("sinew"),
        material: mat("fiber"),
        name: "Sinew".to_string(),
        description: "Animal tendon, very strong".to_string(),
    });

    // Stone submaterials
    registry.register_submaterial(Submaterial {
        id: submat("flint_stone"),
        material: mat("stone"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone".to_string(),
    });
}

fn register_component_kinds(registry: &mut Registry) {
    registry.register_component_kind(ComponentKind {
        id: comp_kind("handle"),
        name: "Handle".to_string(),
        description: "Grip for tools and weapons".to_string(),
        accepted_materials: vec![mat("wood"), mat("bone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("binding"),
        name: "Binding".to_string(),
        description: "Wrapping to secure components".to_string(),
        accepted_materials: vec![mat("leather"), mat("fiber")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("scimitar_blade"),
        name: "Scimitar Blade".to_string(),
        description: "Curved blade for a scimitar".to_string(),
        accepted_materials: vec![mat("metal")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("sword_blade"),
        name: "Sword Blade".to_string(),
        description: "Straight blade for a sword".to_string(),
        accepted_materials: vec![mat("metal")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Small blade for a knife".to_string(),
        accepted_materials: vec![mat("metal"), mat("stone")],
        makeshift_tags: vec!["knife".to_string()],  // Can act as makeshift knife
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Heavy head for mining".to_string(),
        accepted_materials: vec![mat("metal"), mat("stone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("hatchet_head"),
        name: "Hatchet Head".to_string(),
        description: "Sharp head for chopping wood".to_string(),
        accepted_materials: vec![mat("metal"), mat("stone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind("pommel"),
        name: "Pommel".to_string(),
        description: "Counterweight at the end of a weapon".to_string(),
        accepted_materials: vec![mat("metal"), mat("stone")],
        makeshift_tags: vec![],
    });
}

fn register_items(registry: &mut Registry) {
    // =========================================================================
    // SIMPLE ITEMS - Submaterial items
    // =========================================================================
    
    // Leather items
    registry.register_item(ItemDefinition {
        id: item("deer_leather"),
        name: "Deer Leather".to_string(),
        description: "Soft leather from a deer hide".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_leather")) },
    });

    registry.register_item(ItemDefinition {
        id: item("wolf_leather"),
        name: "Wolf Leather".to_string(),
        description: "Tough leather from a wolf hide".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_leather")) },
    });

    // Wood items
    registry.register_item(ItemDefinition {
        id: item("oak_wood"),
        name: "Oak Wood".to_string(),
        description: "Sturdy oak timber".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("oak_wood")) },
    });

    registry.register_item(ItemDefinition {
        id: item("yew_wood"),
        name: "Yew Wood".to_string(),
        description: "Flexible yew timber".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("yew_wood")) },
    });

    // Metal items (note: item ID vs submaterial ID can differ)
    registry.register_item(ItemDefinition {
        id: item("iron_bar"),
        name: "Iron Bar".to_string(),
        description: "Solid bar of smelted iron".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("iron_metal")) },
    });

    registry.register_item(ItemDefinition {
        id: item("bronze_bar"),
        name: "Bronze Bar".to_string(),
        description: "Bronze alloy bar".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("bronze_metal")) },
    });

    registry.register_item(ItemDefinition {
        id: item("steel_bar"),
        name: "Steel Bar".to_string(),
        description: "Refined steel bar".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("steel_metal")) },
    });

    // Fiber items
    registry.register_item(ItemDefinition {
        id: item("plant_fiber"),
        name: "Plant Fiber".to_string(),
        description: "Twisted plant fibers".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("plant_fiber")) },
    });

    registry.register_item(ItemDefinition {
        id: item("sinew"),
        name: "Sinew".to_string(),
        description: "Strong animal tendon".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("sinew")) },
    });

    // Bone items
    registry.register_item(ItemDefinition {
        id: item("wolf_bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense wolf bone".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("wolf_bone")) },
    });

    registry.register_item(ItemDefinition {
        id: item("deer_bone"),
        name: "Deer Bone".to_string(),
        description: "Light deer bone".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("deer_bone")) },
    });

    // Stone items
    registry.register_item(ItemDefinition {
        id: item("flint"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone for knapping".to_string(),
        kind: ItemKind::Simple { submaterial: Some(submat("flint_stone")) },
    });

    // =========================================================================
    // SIMPLE ITEMS - Non-submaterial (consumables, creatures, etc.)
    // =========================================================================

    registry.register_item(ItemDefinition {
        id: item("wolf"),
        name: "Wolf".to_string(),
        description: "A gray-furred predator".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item("wolf_carcass"),
        name: "Wolf Carcass".to_string(),
        description: "Remains of a slain wolf".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item("cooked_meat"),
        name: "Cooked Meat".to_string(),
        description: "Prepared meat that restores health".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item("iron_ore"),
        name: "Iron Ore".to_string(),
        description: "Raw iron ore for smelting".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item("copper_ore"),
        name: "Copper Ore".to_string(),
        description: "Raw copper ore".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item("tin_ore"),
        name: "Tin Ore".to_string(),
        description: "Raw tin ore".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    // Crafting stations
    registry.register_item(ItemDefinition {
        id: item("forge"),
        name: "Forge".to_string(),
        description: "A forge for smelting metals. Provides high heat for crafting.".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    // =========================================================================
    // COMPONENT ITEMS
    // =========================================================================

    registry.register_item(ItemDefinition {
        id: item("handle"),
        name: "Handle".to_string(),
        description: "Grip for tools and weapons".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("handle") },
    });

    registry.register_item(ItemDefinition {
        id: item("binding"),
        name: "Binding".to_string(),
        description: "Wrapping to secure components".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("binding") },
    });

    registry.register_item(ItemDefinition {
        id: item("scimitar_blade"),
        name: "Scimitar Blade".to_string(),
        description: "Curved blade".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("scimitar_blade") },
    });

    registry.register_item(ItemDefinition {
        id: item("sword_blade"),
        name: "Sword Blade".to_string(),
        description: "Straight blade".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("sword_blade") },
    });

    registry.register_item(ItemDefinition {
        id: item("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Small blade".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("knife_blade") },
    });

    registry.register_item(ItemDefinition {
        id: item("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Heavy mining head".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("pickaxe_head") },
    });

    registry.register_item(ItemDefinition {
        id: item("hatchet_head"),
        name: "Hatchet Head".to_string(),
        description: "Sharp chopping head".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("hatchet_head") },
    });

    registry.register_item(ItemDefinition {
        id: item("pommel"),
        name: "Pommel".to_string(),
        description: "Weapon counterweight".to_string(),
        kind: ItemKind::Component { component_kind: comp_kind("pommel") },
    });

    // =========================================================================
    // COMPOSITE ITEMS
    // =========================================================================

    registry.register_item(ItemDefinition {
        id: item("scimitar"),
        name: "Scimitar".to_string(),
        description: "Curved sword with blade, handle, and binding".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot { name: "blade".to_string(), component_kind: comp_kind("scimitar_blade") },
                CompositeSlot { name: "handle".to_string(), component_kind: comp_kind("handle") },
                CompositeSlot { name: "binding".to_string(), component_kind: comp_kind("binding") },
            ],
            category: CompositeCategory::Weapon,
            tool_type: None,
        }),
    });

    registry.register_item(ItemDefinition {
        id: item("sword"),
        name: "Sword".to_string(),
        description: "One-handed blade with pommel".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot { name: "blade".to_string(), component_kind: comp_kind("sword_blade") },
                CompositeSlot { name: "handle".to_string(), component_kind: comp_kind("handle") },
                CompositeSlot { name: "pommel".to_string(), component_kind: comp_kind("pommel") },
            ],
            category: CompositeCategory::Weapon,
            tool_type: None,
        }),
    });

    registry.register_item(ItemDefinition {
        id: item("knife"),
        name: "Knife".to_string(),
        description: "Small cutting tool".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot { name: "blade".to_string(), component_kind: comp_kind("knife_blade") },
                CompositeSlot { name: "handle".to_string(), component_kind: comp_kind("handle") },
                CompositeSlot { name: "binding".to_string(), component_kind: comp_kind("binding") },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Knife),
        }),
    });

    registry.register_item(ItemDefinition {
        id: item("pickaxe"),
        name: "Pickaxe".to_string(),
        description: "Mining tool for breaking rock".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot { name: "head".to_string(), component_kind: comp_kind("pickaxe_head") },
                CompositeSlot { name: "handle".to_string(), component_kind: comp_kind("handle") },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Pickaxe),
        }),
    });

    registry.register_item(ItemDefinition {
        id: item("hatchet"),
        name: "Hatchet".to_string(),
        description: "Woodcutting tool".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot { name: "head".to_string(), component_kind: comp_kind("hatchet_head") },
                CompositeSlot { name: "handle".to_string(), component_kind: comp_kind("handle") },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Hatchet),
        }),
    });
}

fn register_recipes(registry: &mut Registry) {
    // =========================================================================
    // SIMPLE RECIPES - Creating simple items
    // =========================================================================

    // Smelting recipes
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("smelt_iron_bar"),
        name: "Smelt Iron Bar".to_string(),
        output: item("iron_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput { item_id: item("iron_ore"), quantity: 2 },
        ],
        tool: None,
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![WorldObjectTag("high_heat".to_string())],
        }),
    });

    registry.register_simple_recipe(SimpleRecipe {
        id: recipe("smelt_bronze_bar"),
        name: "Smelt Bronze Bar".to_string(),
        output: item("bronze_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput { item_id: item("copper_ore"), quantity: 2 },
            SimpleInput { item_id: item("tin_ore"), quantity: 1 },
        ],
        tool: None,
        world_object: Some(WorldObjectRequirement {
            kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
            required_tags: vec![WorldObjectTag("high_heat".to_string())],
        }),
    });

    // =========================================================================
    // COMPONENT RECIPES - Crafting components from submaterials
    // =========================================================================

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_handle"),
        name: "Craft Handle".to_string(),
        output: comp_kind("handle"),
        tool: None,
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_binding"),
        name: "Craft Binding".to_string(),
        output: comp_kind("binding"),
        tool: None,
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_scimitar_blade"),
        name: "Craft Scimitar Blade".to_string(),
        output: comp_kind("scimitar_blade"),
        tool: Some(ToolRequirement { tool_type: ToolType::Hammer, min_quality: Quality::Crude }),
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_sword_blade"),
        name: "Craft Sword Blade".to_string(),
        output: comp_kind("sword_blade"),
        tool: Some(ToolRequirement { tool_type: ToolType::Hammer, min_quality: Quality::Crude }),
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_knife_blade"),
        name: "Craft Knife Blade".to_string(),
        output: comp_kind("knife_blade"),
        tool: Some(ToolRequirement { tool_type: ToolType::Hammer, min_quality: Quality::Crude }),
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_pickaxe_head"),
        name: "Craft Pickaxe Head".to_string(),
        output: comp_kind("pickaxe_head"),
        tool: Some(ToolRequirement { tool_type: ToolType::Hammer, min_quality: Quality::Crude }),
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_hatchet_head"),
        name: "Craft Hatchet Head".to_string(),
        output: comp_kind("hatchet_head"),
        tool: Some(ToolRequirement { tool_type: ToolType::Hammer, min_quality: Quality::Crude }),
        world_object: None,
    });

    registry.register_component_recipe(ComponentRecipe {
        id: recipe("craft_pommel"),
        name: "Craft Pommel".to_string(),
        output: comp_kind("pommel"),
        tool: None,
        world_object: None,
    });

    // =========================================================================
    // COMPOSITE RECIPES - Assembling composites from components
    // =========================================================================

    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_scimitar"),
        name: "Assemble Scimitar".to_string(),
        output: item("scimitar"),
        tool: None,
        world_object: None,
    });

    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_sword"),
        name: "Assemble Sword".to_string(),
        output: item("sword"),
        tool: None,
        world_object: None,
    });

    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_knife"),
        name: "Assemble Knife".to_string(),
        output: item("knife"),
        tool: None,
        world_object: None,
    });

    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_pickaxe"),
        name: "Assemble Pickaxe".to_string(),
        output: item("pickaxe"),
        tool: None,
        world_object: None,
    });

    registry.register_composite_recipe(CompositeRecipe {
        id: recipe("assemble_hatchet"),
        name: "Assemble Hatchet".to_string(),
        output: item("hatchet"),
        tool: None,
        world_object: None,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_sample_content() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        // Verify materials
        assert!(registry.get_material(&mat("leather")).is_some());
        assert!(registry.get_material(&mat("wood")).is_some());
        assert!(registry.get_material(&mat("metal")).is_some());

        // Verify submaterials
        assert!(registry.get_submaterial(&submat("deer_leather")).is_some());
        assert!(registry.get_submaterial(&submat("oak_wood")).is_some());
        assert!(registry.get_submaterial(&submat("iron_metal")).is_some());

        // Verify component kinds
        assert!(registry.get_component_kind(&comp_kind("handle")).is_some());
        assert!(registry.get_component_kind(&comp_kind("binding")).is_some());
        assert!(registry.get_component_kind(&comp_kind("scimitar_blade")).is_some());

        // Verify items
        assert!(registry.get_item(&item("deer_leather")).is_some());
        assert!(registry.get_item(&item("iron_bar")).is_some());
        assert!(registry.get_item(&item("handle")).is_some());
        assert!(registry.get_item(&item("scimitar")).is_some());

        // Verify recipes
        assert!(registry.get_simple_recipe(&recipe("smelt_iron_bar")).is_some());
        assert!(registry.get_component_recipe(&recipe("craft_handle")).is_some());
        assert!(registry.get_composite_recipe(&recipe("assemble_scimitar")).is_some());
    }

    #[test]
    fn test_scimitar_has_three_slots() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let scimitar = registry.get_item(&item("scimitar")).unwrap();
        if let ItemKind::Composite(def) = &scimitar.kind {
            assert_eq!(def.slots.len(), 3);
            assert_eq!(def.slots[0].name, "blade");
            assert_eq!(def.slots[1].name, "handle");
            assert_eq!(def.slots[2].name, "binding");
        } else {
            panic!("Scimitar should be a composite");
        }
    }

    #[test]
    fn test_handle_component_accepts_wood_and_bone() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let handle = registry.get_component_kind(&comp_kind("handle")).unwrap();
        assert_eq!(handle.accepted_materials.len(), 2);
        assert!(handle.accepted_materials.contains(&mat("wood")));
        assert!(handle.accepted_materials.contains(&mat("bone")));
    }
}
