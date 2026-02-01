//! Sample content: one item from each category in items_list.md
//! 
//! Categories implemented:
//! 1. Resource Node: copper_boulder
//! 2. Creature: wolf
//! 3. Raw Material: copper_ore, tin_ore, wolf_hide, wolf_meat, wolf_heart
//! 4. Processed Material: bronze_bar, leather
//! 5. Crafting Station: forge
//! 6. Tool: pickaxe
//! 7. Weapon: sword
//! 8. Armor: cap
//! 9. Consumable: cooked_meat

use crate::{
    ComponentSlot, Construction, ItemCategories, ItemDefinition, ItemId, MaterialInput,
    MaterialTag, Property, QualityFormula, Quality, Recipe, RecipeId, RecipeOutput, Registry,
    ToolRequirement, ToolType, WorldObjectKind, WorldObjectRequirement,
    CraftingStationId, ResourceNodeId,
};

/// Helper to create a MaterialTag
fn tag(s: &str) -> MaterialTag {
    MaterialTag(s.to_string())
}

/// Helper to create an ItemId
fn item_id(s: &str) -> ItemId {
    ItemId(s.to_string())
}

/// Helper to create a RecipeId
fn recipe_id(s: &str) -> RecipeId {
    RecipeId(s.to_string())
}

/// Populate registry with sample content
pub fn register_sample_content(registry: &mut Registry) {
    // =========================================================================
    // RESOURCE NODES
    // =========================================================================
    
    // copper_boulder - Rich copper ore deposit (Mining Node)
    registry.register_item(ItemDefinition {
        id: item_id("copper_boulder"),
        name: "Copper Boulder".to_string(),
        description: "A rich deposit of copper ore with a distinctive reddish-brown color.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_placeable: true, // Resource nodes are world objects
            ..Default::default()
        },
        material_tags: vec![tag("resource_node"), tag("mining_node"), tag("ore_source")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // CREATURES
    // =========================================================================
    
    // wolf - Pack-hunting predator
    registry.register_item(ItemDefinition {
        id: item_id("wolf"),
        name: "Wolf".to_string(),
        description: "A gray-furred pack-hunting predator. Dangerous alone, deadly in groups.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_creature: true,
            ..Default::default()
        },
        material_tags: vec![tag("creature"), tag("beast"), tag("predator")],
        tool_type: None,
        inherent_properties: vec![
            Property {
                id: "pack_hunter".to_string(),
                name: "Pack Hunter".to_string(),
                description: "This creature is more dangerous when accompanied by others of its kind.".to_string(),
            }
        ],
    });

    // wolf_carcass - Used for provenance tracking (what the wolf drops when killed)
    registry.register_item(ItemDefinition {
        id: item_id("wolf_carcass"),
        name: "Wolf Carcass".to_string(),
        description: "The remains of a slain wolf. Can be harvested for hide, meat, and other materials.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            is_placeable: true, // Carcasses are placed in the world
            ..Default::default()
        },
        material_tags: vec![tag("carcass"), tag("beast_carcass"), tag("wolf_carcass")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // RAW MATERIALS
    // =========================================================================
    
    // copper_ore - Reddish-brown metal ore
    registry.register_item(ItemDefinition {
        id: item_id("copper_ore"),
        name: "Copper Ore".to_string(),
        description: "Chunks of reddish-brown copper ore ready for smelting.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("ore"), tag("metal_ore"), tag("copper")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // tin_ore - Soft, silvery-white ore (needed for bronze)
    registry.register_item(ItemDefinition {
        id: item_id("tin_ore"),
        name: "Tin Ore".to_string(),
        description: "Soft, silvery-white ore that alloys well with copper.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("ore"), tag("metal_ore"), tag("tin")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // wolf_hide - Tough gray pelt
    registry.register_item(ItemDefinition {
        id: item_id("wolf_hide"),
        name: "Wolf Hide".to_string(),
        description: "A tough gray pelt from a wolf. Provides good protection when tanned.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("hide"), tag("animal_hide"), tag("leather_source")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // wolf_meat - Gamey red meat
    registry.register_item(ItemDefinition {
        id: item_id("wolf_meat"),
        name: "Wolf Meat".to_string(),
        description: "Gamey red meat from a wolf. Should be cooked before eating.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("meat"), tag("raw_meat"), tag("game_meat")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // wolf_heart - Powerful organ (provenance tracking showcase)
    registry.register_item(ItemDefinition {
        id: item_id("wolf_heart"),
        name: "Wolf Heart".to_string(),
        description: "The still-warm heart of a wolf. Valued for its symbolic power in crafting.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("organ"), tag("heart"), tag("wolf_part"), tag("pommel_material")],
        tool_type: None,
        inherent_properties: vec![
            Property {
                id: "predator_essence".to_string(),
                name: "Predator's Essence".to_string(),
                description: "Contains the fierce spirit of a predator.".to_string(),
            }
        ],
    });

    // oak_logs - For tool handles
    registry.register_item(ItemDefinition {
        id: item_id("oak_logs"),
        name: "Oak Logs".to_string(),
        description: "Sturdy hardwood timber from an oak tree.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("wood"), tag("hardwood"), tag("handle_material")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // PROCESSED MATERIALS
    // =========================================================================
    
    // bronze_bar - Copper + tin alloy
    registry.register_item(ItemDefinition {
        id: item_id("bronze_bar"),
        name: "Bronze Bar".to_string(),
        description: "A sturdy alloy of copper and tin. Good for early metal tools and weapons.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![
            tag("metal"), tag("bar"), tag("bronze"), 
            tag("blade_material"), tag("head_material"), tag("pommel_material"),
        ],
        tool_type: None,
        inherent_properties: vec![],
    });

    // leather - Tanned hide
    registry.register_item(ItemDefinition {
        id: item_id("leather"),
        name: "Leather".to_string(),
        description: "Tanned animal hide, flexible and durable.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![
            tag("leather"), tag("armor_material"), 
            tag("handle_material"), tag("shell_material"),
        ],
        tool_type: None,
        inherent_properties: vec![],
    });

    // linen_cloth - For armor lining
    registry.register_item(ItemDefinition {
        id: item_id("linen_cloth"),
        name: "Linen Cloth".to_string(),
        description: "Woven linen fabric, soft and breathable.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_material: true,
            ..Default::default()
        },
        material_tags: vec![tag("cloth"), tag("fabric"), tag("lining_material")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // CRAFTING STATIONS
    // =========================================================================
    
    // forge - High-heat furnace for smelting
    registry.register_item(ItemDefinition {
        id: item_id("forge"),
        name: "Forge".to_string(),
        description: "A high-heat furnace capable of smelting ores into metal bars.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_placeable: true,
            ..Default::default()
        },
        material_tags: vec![tag("crafting_station"), tag("high_heat"), tag("smelting")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // TOOLS
    // =========================================================================
    
    // pickaxe - Mining tool with head and handle components
    registry.register_item(ItemDefinition {
        id: item_id("pickaxe"),
        name: "Pickaxe".to_string(),
        description: "A mining tool for breaking rock and extracting ore.".to_string(),
        component_slots: vec![
            ComponentSlot {
                name: "head".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("head_material")], // stone, metal bars, etc.
                optional_tags: vec![tag("metal")],
            },
            ComponentSlot {
                name: "handle".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("wood"), tag("bone")], // wood OR bone
                optional_tags: vec![tag("hardwood")],
            },
        ],
        categories: ItemCategories {
            is_tool: true,
            ..Default::default()
        },
        material_tags: vec![tag("tool"), tag("mining_tool")],
        tool_type: Some(ToolType::Pickaxe),
        inherent_properties: vec![],
    });

    // =========================================================================
    // WEAPONS
    // =========================================================================
    
    // sword - Versatile blade with blade, handle, and pommel components
    registry.register_item(ItemDefinition {
        id: item_id("sword"),
        name: "Sword".to_string(),
        description: "A versatile one-handed blade suitable for both slashing and thrusting.".to_string(),
        component_slots: vec![
            ComponentSlot {
                name: "blade".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("blade_material")], // metal bars
                optional_tags: vec![tag("metal")],
            },
            ComponentSlot {
                name: "handle".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("wood"), tag("leather"), tag("bone")], // wood OR leather OR bone
                optional_tags: vec![tag("hardwood")],
            },
            ComponentSlot {
                name: "pommel".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("pommel_material"), tag("gem")], // metal OR gem OR special items
                optional_tags: vec![],
            },
        ],
        categories: ItemCategories {
            is_weapon: true,
            ..Default::default()
        },
        material_tags: vec![tag("weapon"), tag("blade_weapon"), tag("one_handed")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // ARMOR
    // =========================================================================
    
    // cap - Head protection with shell and lining components
    registry.register_item(ItemDefinition {
        id: item_id("cap"),
        name: "Cap".to_string(),
        description: "Basic head protection made from leather with a cloth lining.".to_string(),
        component_slots: vec![
            ComponentSlot {
                name: "shell".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("leather"), tag("hide")], // leather OR raw hide
                optional_tags: vec![],
            },
            ComponentSlot {
                name: "lining".to_string(),
                required_tags: vec![],
                accepted_tags: vec![tag("cloth"), tag("fabric")], // any cloth/fabric
                optional_tags: vec![],
            },
        ],
        categories: ItemCategories {
            is_armor: true,
            ..Default::default()
        },
        material_tags: vec![tag("armor"), tag("head_armor"), tag("light_armor")],
        tool_type: None,
        inherent_properties: vec![],
    });

    // =========================================================================
    // CONSUMABLES
    // =========================================================================
    
    // cooked_meat - Prepared food
    registry.register_item(ItemDefinition {
        id: item_id("cooked_meat"),
        name: "Cooked Meat".to_string(),
        description: "Well-prepared meat that restores health when consumed.".to_string(),
        component_slots: vec![],
        categories: ItemCategories {
            is_consumable: true,
            ..Default::default()
        },
        material_tags: vec![tag("food"), tag("cooked"), tag("meat")],
        tool_type: None,
        inherent_properties: vec![
            Property {
                id: "restores_health".to_string(),
                name: "Restores Health".to_string(),
                description: "Consuming this item restores a moderate amount of health.".to_string(),
            }
        ],
    });

    // =========================================================================
    // RECIPES
    // =========================================================================

    // Recipe: Mine copper ore from copper boulder
    registry.register_recipe(Recipe {
        id: recipe_id("mine_copper_ore"),
        name: "Mine Copper Ore".to_string(),
        construction: Construction {
            tool: Some(ToolRequirement {
                tool_type: ToolType::Pickaxe,
                min_quality: Quality::Crude,
            }),
            world_object: Some(WorldObjectRequirement {
                kind: Some(WorldObjectKind::ResourceNode(ResourceNodeId("copper_boulder".to_string()))),
                required_tags: vec![],
            }),
            material_inputs: vec![],
        },
        output: RecipeOutput {
            item_id: item_id("copper_ore"),
            quantity: 1,
            quality_formula: QualityFormula::Custom("tool_quality_based".to_string()),
        },
    });

    // Recipe: Smelt bronze bar at forge
    registry.register_recipe(Recipe {
        id: recipe_id("smelt_bronze_bar"),
        name: "Smelt Bronze Bar".to_string(),
        construction: Construction {
            tool: None,
            world_object: Some(WorldObjectRequirement {
                kind: Some(WorldObjectKind::CraftingStation(CraftingStationId("forge".to_string()))),
                required_tags: vec![],
            }),
            material_inputs: vec![
                MaterialInput {
                    item_id: Some(item_id("copper_ore")),
                    required_tags: vec![],
                    quantity: 2,
                    min_quality: None,
                    fills_slot: None, // Simple output, no component slots
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
                MaterialInput {
                    item_id: Some(item_id("tin_ore")),
                    required_tags: vec![],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: None,
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("bronze_bar"),
            quantity: 1,
            quality_formula: QualityFormula::MinOfInputs,
        },
    });

    // Recipe: Harvest wolf carcass (skinning)
    registry.register_recipe(Recipe {
        id: recipe_id("harvest_wolf_hide"),
        name: "Harvest Wolf Hide".to_string(),
        construction: Construction {
            tool: Some(ToolRequirement {
                tool_type: ToolType::Knife,
                min_quality: Quality::Crude,
            }),
            world_object: None,
            material_inputs: vec![
                MaterialInput {
                    item_id: Some(item_id("wolf_carcass")),
                    required_tags: vec![],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: None,
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("wolf_hide"),
            quantity: 1,
            quality_formula: QualityFormula::MinOfInputs,
        },
    });

    // Recipe: Craft pickaxe
    registry.register_recipe(Recipe {
        id: recipe_id("craft_pickaxe"),
        name: "Craft Pickaxe".to_string(),
        construction: Construction {
            tool: Some(ToolRequirement {
                tool_type: ToolType::Hammer,
                min_quality: Quality::Crude,
            }),
            world_object: None,
            material_inputs: vec![
                MaterialInput {
                    item_id: None, // Any item with head_material tag
                    required_tags: vec![tag("head_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("head".to_string()), // Explicitly fills head slot
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
                MaterialInput {
                    item_id: None, // Any item with handle_material tag
                    required_tags: vec![tag("handle_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("handle".to_string()), // Explicitly fills handle slot
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("pickaxe"),
            quantity: 1,
            quality_formula: QualityFormula::Weighted(vec![
                ("head".to_string(), 0.7),
                ("handle".to_string(), 0.3),
            ]),
        },
    });

    // Recipe: Craft sword
    registry.register_recipe(Recipe {
        id: recipe_id("craft_sword"),
        name: "Craft Sword".to_string(),
        construction: Construction {
            tool: Some(ToolRequirement {
                tool_type: ToolType::Hammer,
                min_quality: Quality::Common,
            }),
            world_object: None,
            material_inputs: vec![
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("blade_material")],
                    quantity: 2, // Swords need more metal
                    min_quality: None,
                    fills_slot: Some("blade".to_string()),
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("handle_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("handle".to_string()),
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("pommel_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("pommel".to_string()),
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("sword"),
            quantity: 1,
            quality_formula: QualityFormula::Weighted(vec![
                ("blade".to_string(), 0.6),
                ("handle".to_string(), 0.25),
                ("pommel".to_string(), 0.15),
            ]),
        },
    });

    // Recipe: Craft cap
    registry.register_recipe(Recipe {
        id: recipe_id("craft_cap"),
        name: "Craft Cap".to_string(),
        construction: Construction {
            tool: Some(ToolRequirement {
                tool_type: ToolType::Needle,
                min_quality: Quality::Crude,
            }),
            world_object: None,
            material_inputs: vec![
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("shell_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("shell".to_string()),
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("lining_material")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: Some("lining".to_string()),
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("cap"),
            quantity: 1,
            quality_formula: QualityFormula::AverageOfInputs,
        },
    });

    // Recipe: Cook meat (uses high_heat tag instead of specific station)
    registry.register_recipe(Recipe {
        id: recipe_id("cook_meat"),
        name: "Cook Meat".to_string(),
        construction: Construction {
            tool: None,
            world_object: Some(WorldObjectRequirement {
                kind: None, // Any world object with high_heat tag
                required_tags: vec![crate::WorldObjectTag("high_heat".to_string())],
            }),
            material_inputs: vec![
                MaterialInput {
                    item_id: None,
                    required_tags: vec![tag("raw_meat")],
                    quantity: 1,
                    min_quality: None,
                    fills_slot: None,
                    component_reqs: vec![],
                    provenance_reqs: None,
                },
            ],
        },
        output: RecipeOutput {
            item_id: item_id("cooked_meat"),
            quantity: 1,
            quality_formula: QualityFormula::MinOfInputs,
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_sample_content() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        // Verify items were registered
        assert!(registry.get_item(&item_id("copper_boulder")).is_some());
        assert!(registry.get_item(&item_id("wolf")).is_some());
        assert!(registry.get_item(&item_id("copper_ore")).is_some());
        assert!(registry.get_item(&item_id("bronze_bar")).is_some());
        assert!(registry.get_item(&item_id("forge")).is_some());
        assert!(registry.get_item(&item_id("pickaxe")).is_some());
        assert!(registry.get_item(&item_id("sword")).is_some());
        assert!(registry.get_item(&item_id("cap")).is_some());
        assert!(registry.get_item(&item_id("cooked_meat")).is_some());

        // Verify recipes were registered
        assert!(registry.get_recipe(&recipe_id("mine_copper_ore")).is_some());
        assert!(registry.get_recipe(&recipe_id("smelt_bronze_bar")).is_some());
        assert!(registry.get_recipe(&recipe_id("craft_pickaxe")).is_some());
        assert!(registry.get_recipe(&recipe_id("craft_sword")).is_some());
        assert!(registry.get_recipe(&recipe_id("craft_cap")).is_some());
        assert!(registry.get_recipe(&recipe_id("cook_meat")).is_some());
    }

    #[test]
    fn test_pickaxe_has_components() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let pickaxe = registry.get_item(&item_id("pickaxe")).unwrap();
        assert_eq!(pickaxe.component_slots.len(), 2);
        assert_eq!(pickaxe.component_slots[0].name, "head");
        assert_eq!(pickaxe.component_slots[1].name, "handle");
    }

    #[test]
    fn test_sword_has_three_components() {
        let mut registry = Registry::new();
        register_sample_content(&mut registry);

        let sword = registry.get_item(&item_id("sword")).unwrap();
        assert_eq!(sword.component_slots.len(), 3);
        
        let slot_names: Vec<_> = sword.component_slots.iter().map(|s| &s.name).collect();
        assert!(slot_names.contains(&&"blade".to_string()));
        assert!(slot_names.contains(&&"handle".to_string()));
        assert!(slot_names.contains(&&"pommel".to_string()));
    }
}
