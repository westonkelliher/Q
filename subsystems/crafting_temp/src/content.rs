//! Sample content for the Tinker's Construct style crafting system
//!
//! This module demonstrates the new three-tier hierarchy:
//! 1. Materials → Submaterials → Simple Items
//! 2. Submaterials → Components (via ComponentRecipe)
//! 3. Components → Composites (via CompositeRecipe)

use crate::{
    ItemId, RecipeId, MaterialId, SubmaterialId, ComponentKindId,
    Material, Submaterial, ComponentKind,
    ItemDefinition, ItemKind, CompositeDef, CompositeSlot, CompositeCategory, ToolType,
    SimpleRecipe, SimpleInput, ComponentRecipe, CompositeRecipe,
    ToolRequirement, Registry, Quality,
};

/// Helper to create material IDs
fn mat_id(s: &str) -> MaterialId {
    MaterialId(s.to_string())
}

/// Helper to create submaterial IDs
fn submat_id(s: &str) -> SubmaterialId {
    SubmaterialId(s.to_string())
}

/// Helper to create component kind IDs
fn comp_kind_id(s: &str) -> ComponentKindId {
    ComponentKindId(s.to_string())
}

/// Helper to create item IDs
fn item_id(s: &str) -> ItemId {
    ItemId(s.to_string())
}

/// Helper to create recipe IDs
fn recipe_id(s: &str) -> RecipeId {
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

//==============================================================================
// MATERIALS (Broad categories)
//==============================================================================

fn register_materials(registry: &mut Registry) {
    registry.register_material(Material {
        id: mat_id("leather"),
        name: "Leather".to_string(),
        description: "Flexible and durable material from animal hides".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("wood"),
        name: "Wood".to_string(),
        description: "Sturdy material from trees".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("metal"),
        name: "Metal".to_string(),
        description: "Strong metallic material".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("gem"),
        name: "Gem".to_string(),
        description: "Precious crystalline material".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("bone"),
        name: "Bone".to_string(),
        description: "Rigid organic material".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("fiber"),
        name: "Fiber".to_string(),
        description: "Plant or animal fibers for binding".to_string(),
    });

    registry.register_material(Material {
        id: mat_id("stone"),
        name: "Stone".to_string(),
        description: "Hard mineral material".to_string(),
    });
}

//==============================================================================
// SUBMATERIALS (Specific variants that belong to materials)
//==============================================================================

fn register_submaterials(registry: &mut Registry) {
    // Leather submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("deer_leather"),
        material: mat_id("leather"),
        name: "Deer Leather".to_string(),
        description: "Soft, flexible leather from deer hides".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("wolf_leather"),
        material: mat_id("leather"),
        name: "Wolf Leather".to_string(),
        description: "Tough, durable leather from wolf pelts".to_string(),
    });

    // Wood submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("oak_wood"),
        material: mat_id("wood"),
        name: "Oak Wood".to_string(),
        description: "Strong, reliable hardwood".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("yew_wood"),
        material: mat_id("wood"),
        name: "Yew Wood".to_string(),
        description: "Flexible wood ideal for tool handles".to_string(),
    });

    // Metal submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("iron_metal"),
        material: mat_id("metal"),
        name: "Iron".to_string(),
        description: "Common, reliable metal".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("bronze_metal"),
        material: mat_id("metal"),
        name: "Bronze".to_string(),
        description: "Copper-tin alloy, good for early tools".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("steel_metal"),
        material: mat_id("metal"),
        name: "Steel".to_string(),
        description: "Refined iron, strong and sharp".to_string(),
    });

    // Bone submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("wolf_bone"),
        material: mat_id("bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense bone from wolf skeletons".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("deer_bone"),
        material: mat_id("bone"),
        name: "Deer Bone".to_string(),
        description: "Light but sturdy bone".to_string(),
    });

    // Fiber submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("plant_fiber"),
        material: mat_id("fiber"),
        name: "Plant Fiber".to_string(),
        description: "Processed plant fibers for cordage".to_string(),
    });

    registry.register_submaterial(Submaterial {
        id: submat_id("sinew"),
        material: mat_id("fiber"),
        name: "Sinew".to_string(),
        description: "Animal tendon, strong binding material".to_string(),
    });

    // Stone submaterials
    registry.register_submaterial(Submaterial {
        id: submat_id("flint_stone"),
        material: mat_id("stone"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone for primitive tools".to_string(),
    });
}

//==============================================================================
// COMPONENT KINDS (Types of components that can be crafted)
//==============================================================================

fn register_component_kinds(registry: &mut Registry) {
    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("handle"),
        name: "Handle".to_string(),
        description: "Grip for tools and weapons".to_string(),
        accepted_materials: vec![mat_id("wood"), mat_id("bone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("binding"),
        name: "Binding".to_string(),
        description: "Material to bind components together".to_string(),
        accepted_materials: vec![mat_id("leather"), mat_id("fiber")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("scimitar_blade"),
        name: "Scimitar Blade".to_string(),
        description: "Curved blade for a scimitar".to_string(),
        accepted_materials: vec![mat_id("metal")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("sword_blade"),
        name: "Sword Blade".to_string(),
        description: "Straight blade for a sword".to_string(),
        accepted_materials: vec![mat_id("metal")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Small blade for a knife".to_string(),
        accepted_materials: vec![mat_id("metal"), mat_id("stone")],
        makeshift_tags: vec!["knife".to_string()],  // Can act as makeshift knife
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Mining head for a pickaxe".to_string(),
        accepted_materials: vec![mat_id("metal"), mat_id("stone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("hatchet_head"),
        name: "Hatchet Head".to_string(),
        description: "Chopping head for a hatchet".to_string(),
        accepted_materials: vec![mat_id("metal"), mat_id("stone")],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: comp_kind_id("pommel"),
        name: "Pommel".to_string(),
        description: "Counterweight at the end of a weapon handle".to_string(),
        accepted_materials: vec![mat_id("metal"), mat_id("stone"), mat_id("gem")],
        makeshift_tags: vec![],
    });
}

//==============================================================================
// ITEMS
//==============================================================================

fn register_items(registry: &mut Registry) {
    // Simple items WITH submaterials (these are the submaterial items themselves)
    register_submaterial_items(registry);

    // Simple items WITHOUT submaterials (non-material items)
    register_simple_items(registry);

    // Component items (correspond to component kinds)
    register_component_items(registry);

    // Composite items (assembled from components)
    register_composite_items(registry);
}

fn register_submaterial_items(registry: &mut Registry) {
    // Leather items
    registry.register_item(ItemDefinition {
        id: item_id("deer_leather"),
        name: "Deer Leather".to_string(),
        description: "Tanned deer hide, soft and flexible".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("deer_leather"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("wolf_leather"),
        name: "Wolf Leather".to_string(),
        description: "Tanned wolf pelt, tough and durable".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("wolf_leather"))
        },
    });

    // Wood items
    registry.register_item(ItemDefinition {
        id: item_id("oak_wood"),
        name: "Oak Wood".to_string(),
        description: "Sturdy hardwood timber".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("oak_wood"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("yew_wood"),
        name: "Yew Wood".to_string(),
        description: "Flexible wood ideal for handles".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("yew_wood"))
        },
    });

    // Metal items (bars)
    registry.register_item(ItemDefinition {
        id: item_id("iron_bar"),
        name: "Iron Bar".to_string(),
        description: "Solid bar of smelted iron".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("iron_metal"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("bronze_bar"),
        name: "Bronze Bar".to_string(),
        description: "Copper-tin alloy bar".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("bronze_metal"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("steel_bar"),
        name: "Steel Bar".to_string(),
        description: "Refined iron bar, strong and sharp".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("steel_metal"))
        },
    });

    // Bone items
    registry.register_item(ItemDefinition {
        id: item_id("wolf_bone"),
        name: "Wolf Bone".to_string(),
        description: "Dense bone from a wolf".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("wolf_bone"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("deer_bone"),
        name: "Deer Bone".to_string(),
        description: "Light but sturdy bone from a deer".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("deer_bone"))
        },
    });

    // Fiber items
    registry.register_item(ItemDefinition {
        id: item_id("plant_fiber"),
        name: "Plant Fiber".to_string(),
        description: "Processed plant fibers".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("plant_fiber"))
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("sinew"),
        name: "Sinew".to_string(),
        description: "Animal tendon for strong bindings".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("sinew"))
        },
    });

    // Stone items
    registry.register_item(ItemDefinition {
        id: item_id("flint"),
        name: "Flint".to_string(),
        description: "Sharp-edged stone".to_string(),
        kind: ItemKind::Simple {
            submaterial: Some(submat_id("flint_stone"))
        },
    });
}

fn register_simple_items(registry: &mut Registry) {
    // Creatures
    registry.register_item(ItemDefinition {
        id: item_id("wolf"),
        name: "Wolf".to_string(),
        description: "A gray-furred pack-hunting predator".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item_id("wolf_carcass"),
        name: "Wolf Carcass".to_string(),
        description: "Remains of a slain wolf, can be harvested".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    // Raw materials (ores)
    registry.register_item(ItemDefinition {
        id: item_id("copper_ore"),
        name: "Copper Ore".to_string(),
        description: "Reddish-brown copper ore ready for smelting".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item_id("tin_ore"),
        name: "Tin Ore".to_string(),
        description: "Soft, silvery-white ore that alloys well with copper".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    registry.register_item(ItemDefinition {
        id: item_id("iron_ore"),
        name: "Iron Ore".to_string(),
        description: "Gray metallic ore that can be smelted into iron".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });

    // Consumables
    registry.register_item(ItemDefinition {
        id: item_id("cooked_meat"),
        name: "Cooked Meat".to_string(),
        description: "Well-prepared meat that restores health".to_string(),
        kind: ItemKind::Simple { submaterial: None },
    });
}

fn register_component_items(registry: &mut Registry) {
    // These item definitions correspond 1:1 with component kinds
    registry.register_item(ItemDefinition {
        id: item_id("handle"),
        name: "Handle".to_string(),
        description: "Grip for tools and weapons".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("handle")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("binding"),
        name: "Binding".to_string(),
        description: "Material to bind components together".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("binding")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("scimitar_blade"),
        name: "Scimitar Blade".to_string(),
        description: "Curved blade for a scimitar".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("scimitar_blade")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("sword_blade"),
        name: "Sword Blade".to_string(),
        description: "Straight blade for a sword".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("sword_blade")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("knife_blade"),
        name: "Knife Blade".to_string(),
        description: "Small blade for a knife".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("knife_blade")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("pickaxe_head"),
        name: "Pickaxe Head".to_string(),
        description: "Mining head for a pickaxe".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("pickaxe_head")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("hatchet_head"),
        name: "Hatchet Head".to_string(),
        description: "Chopping head for a hatchet".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("hatchet_head")
        },
    });

    registry.register_item(ItemDefinition {
        id: item_id("pommel"),
        name: "Pommel".to_string(),
        description: "Counterweight at the end of a weapon handle".to_string(),
        kind: ItemKind::Component {
            component_kind: comp_kind_id("pommel")
        },
    });
}

fn register_composite_items(registry: &mut Registry) {
    // Scimitar: blade + handle + binding
    registry.register_item(ItemDefinition {
        id: item_id("scimitar"),
        name: "Scimitar".to_string(),
        description: "Curved one-handed blade".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "blade".to_string(),
                    component_kind: comp_kind_id("scimitar_blade")
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind_id("handle")
                },
                CompositeSlot {
                    name: "binding".to_string(),
                    component_kind: comp_kind_id("binding")
                },
            ],
            category: CompositeCategory::Weapon,
            tool_type: None,
        }),
    });

    // Sword: blade + handle + pommel
    registry.register_item(ItemDefinition {
        id: item_id("sword"),
        name: "Sword".to_string(),
        description: "Straight one-handed blade".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "blade".to_string(),
                    component_kind: comp_kind_id("sword_blade")
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind_id("handle")
                },
                CompositeSlot {
                    name: "pommel".to_string(),
                    component_kind: comp_kind_id("pommel")
                },
            ],
            category: CompositeCategory::Weapon,
            tool_type: None,
        }),
    });

    // Knife: blade + handle + binding
    registry.register_item(ItemDefinition {
        id: item_id("knife"),
        name: "Knife".to_string(),
        description: "Small utility blade".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "blade".to_string(),
                    component_kind: comp_kind_id("knife_blade")
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind_id("handle")
                },
                CompositeSlot {
                    name: "binding".to_string(),
                    component_kind: comp_kind_id("binding")
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Knife),
        }),
    });

    // Pickaxe: head + handle
    registry.register_item(ItemDefinition {
        id: item_id("pickaxe"),
        name: "Pickaxe".to_string(),
        description: "Mining tool for breaking rock".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "head".to_string(),
                    component_kind: comp_kind_id("pickaxe_head")
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind_id("handle")
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Pickaxe),
        }),
    });

    // Hatchet: head + handle
    registry.register_item(ItemDefinition {
        id: item_id("hatchet"),
        name: "Hatchet".to_string(),
        description: "Woodcutting tool for felling trees".to_string(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "head".to_string(),
                    component_kind: comp_kind_id("hatchet_head")
                },
                CompositeSlot {
                    name: "handle".to_string(),
                    component_kind: comp_kind_id("handle")
                },
            ],
            category: CompositeCategory::Tool,
            tool_type: Some(ToolType::Hatchet),
        }),
    });
}

//==============================================================================
// RECIPES
//==============================================================================

fn register_recipes(registry: &mut Registry) {
    register_simple_recipes(registry);
    register_component_recipes(registry);
    register_composite_recipes(registry);
}

fn register_simple_recipes(registry: &mut Registry) {
    // Smelt iron bar: iron_ore → iron_bar
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe_id("smelt_iron_bar"),
        name: "Smelt Iron Bar".to_string(),
        output: item_id("iron_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item_id("iron_ore"),
                quantity: 2,
            },
        ],
        tool: None,
        world_object: None,  // TODO: add forge requirement
    });

    // Smelt bronze bar: copper_ore + tin_ore → bronze_bar
    registry.register_simple_recipe(SimpleRecipe {
        id: recipe_id("smelt_bronze_bar"),
        name: "Smelt Bronze Bar".to_string(),
        output: item_id("bronze_bar"),
        output_quantity: 1,
        inputs: vec![
            SimpleInput {
                item_id: item_id("copper_ore"),
                quantity: 2,
            },
            SimpleInput {
                item_id: item_id("tin_ore"),
                quantity: 1,
            },
        ],
        tool: None,
        world_object: None,  // TODO: add forge requirement
    });
}

fn register_component_recipes(registry: &mut Registry) {
    // Craft handle (from wood or bone submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_handle"),
        name: "Craft Handle".to_string(),
        output: comp_kind_id("handle"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });

    // Craft binding (from leather or fiber submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_binding"),
        name: "Craft Binding".to_string(),
        output: comp_kind_id("binding"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Knife,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });

    // Craft scimitar blade (from metal submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_scimitar_blade"),
        name: "Craft Scimitar Blade".to_string(),
        output: comp_kind_id("scimitar_blade"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,  // TODO: add forge requirement
    });

    // Craft sword blade (from metal submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_sword_blade"),
        name: "Craft Sword Blade".to_string(),
        output: comp_kind_id("sword_blade"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,  // TODO: add forge requirement
    });

    // Craft knife blade (from metal or stone submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_knife_blade"),
        name: "Craft Knife Blade".to_string(),
        output: comp_kind_id("knife_blade"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });

    // Craft pickaxe head (from metal or stone submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_pickaxe_head"),
        name: "Craft Pickaxe Head".to_string(),
        output: comp_kind_id("pickaxe_head"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });

    // Craft hatchet head (from metal or stone submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_hatchet_head"),
        name: "Craft Hatchet Head".to_string(),
        output: comp_kind_id("hatchet_head"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });

    // Craft pommel (from metal, stone, or gem submaterial)
    registry.register_component_recipe(ComponentRecipe {
        id: recipe_id("craft_pommel"),
        name: "Craft Pommel".to_string(),
        output: comp_kind_id("pommel"),
        tool: Some(ToolRequirement {
            tool_type: ToolType::Hammer,
            min_quality: Quality::Common,
        }),
        world_object: None,
    });
}

fn register_composite_recipes(registry: &mut Registry) {
    // Assemble scimitar (blade + handle + binding)
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe_id("assemble_scimitar"),
        name: "Assemble Scimitar".to_string(),
        output: item_id("scimitar"),
        tool: None,  // Hand assembly
        world_object: None,
    });

    // Assemble sword (blade + handle + pommel)
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe_id("assemble_sword"),
        name: "Assemble Sword".to_string(),
        output: item_id("sword"),
        tool: None,  // Hand assembly
        world_object: None,
    });

    // Assemble knife (blade + handle + binding)
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe_id("assemble_knife"),
        name: "Assemble Knife".to_string(),
        output: item_id("knife"),
        tool: None,  // Hand assembly
        world_object: None,
    });

    // Assemble pickaxe (head + handle)
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe_id("assemble_pickaxe"),
        name: "Assemble Pickaxe".to_string(),
        output: item_id("pickaxe"),
        tool: None,  // Hand assembly
        world_object: None,
    });

    // Assemble hatchet (head + handle)
    registry.register_composite_recipe(CompositeRecipe {
        id: recipe_id("assemble_hatchet"),
        name: "Assemble Hatchet".to_string(),
        output: item_id("hatchet"),
        tool: None,  // Hand assembly
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
        assert!(registry.get_material(&mat_id("leather")).is_some());
        assert!(registry.get_material(&mat_id("wood")).is_some());
        assert!(registry.get_material(&mat_id("metal")).is_some());

        // Verify submaterials
        assert!(registry.get_submaterial(&submat_id("deer_leather")).is_some());
        assert!(registry.get_submaterial(&submat_id("oak_wood")).is_some());
        assert!(registry.get_submaterial(&submat_id("iron_metal")).is_some());

        // Verify component kinds
        assert!(registry.get_component_kind(&comp_kind_id("handle")).is_some());
        assert!(registry.get_component_kind(&comp_kind_id("binding")).is_some());
        assert!(registry.get_component_kind(&comp_kind_id("scimitar_blade")).is_some());

        // Verify items
        assert!(registry.get_item(&item_id("deer_leather")).is_some());
        assert!(registry.get_item(&item_id("handle")).is_some());
        assert!(registry.get_item(&item_id("scimitar")).is_some());
        assert!(registry.get_item(&item_id("pickaxe")).is_some());

        // Verify recipes
        assert!(registry.get_simple_recipe(&recipe_id("smelt_iron_bar")).is_some());
        assert!(registry.get_component_recipe(&recipe_id("craft_handle")).is_some());
        assert!(registry.get_composite_recipe(&recipe_id("assemble_scimitar")).is_some());
    }

    #[test]
    fn test_component_kinds_have_correct_accepted_materials() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let handle = registry.get_component_kind(&comp_kind_id("handle")).unwrap();
        assert_eq!(handle.accepted_materials.len(), 2);
        assert!(handle.accepted_materials.contains(&mat_id("wood")));
        assert!(handle.accepted_materials.contains(&mat_id("bone")));

        let binding = registry.get_component_kind(&comp_kind_id("binding")).unwrap();
        assert!(binding.accepted_materials.contains(&mat_id("leather")));
        assert!(binding.accepted_materials.contains(&mat_id("fiber")));
    }

    #[test]
    fn test_composite_items_have_correct_slots() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let scimitar = registry.get_item(&item_id("scimitar")).unwrap();
        match &scimitar.kind {
            ItemKind::Composite(def) => {
                assert_eq!(def.slots.len(), 3);
                assert_eq!(def.slots[0].name, "blade");
                assert_eq!(def.slots[1].name, "handle");
                assert_eq!(def.slots[2].name, "binding");
            }
            _ => panic!("Expected Composite item"),
        }
    }
}
