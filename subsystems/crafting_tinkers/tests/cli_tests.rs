use crafting::{cli, Registry};
use serde_json::Value;

/// Helper to execute a command and parse JSON response
fn exec_command(cmd: &str, registry: &mut Registry) -> Value {
    let command = cli::parse_command(cmd).expect("Failed to parse command");
    cli::execute_command(command, registry)
}

/// Helper to check if response is successful
fn is_success(response: &Value) -> bool {
    response["status"] == "success"
}

/// Helper to get data from successful response
fn get_data(response: &Value) -> &Value {
    &response["data"]
}

// ============================================================================
// REGISTRATION TESTS - Materials, Submaterials, ComponentKinds
// ============================================================================

#[test]
fn test_material_registration() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Verify all 6 base materials exist
    let materials: Vec<_> = registry.all_materials().map(|m| m.id.0.clone()).collect();
    assert!(materials.contains(&"leather".to_string()));
    assert!(materials.contains(&"wood".to_string()));
    assert!(materials.contains(&"metal".to_string()));
    assert!(materials.contains(&"bone".to_string()));
    assert!(materials.contains(&"fiber".to_string()));
    assert!(materials.contains(&"stone".to_string()));
    assert_eq!(materials.len(), 6);
}

#[test]
fn test_submaterial_registration() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Verify submaterials with their parent materials
    let deer_leather = registry.get_submaterial(&crafting::SubmaterialId("deer_leather".to_string())).unwrap();
    assert_eq!(deer_leather.material.0, "leather");

    let oak_wood = registry.get_submaterial(&crafting::SubmaterialId("oak_wood".to_string())).unwrap();
    assert_eq!(oak_wood.material.0, "wood");

    let iron_metal = registry.get_submaterial(&crafting::SubmaterialId("iron_metal".to_string())).unwrap();
    assert_eq!(iron_metal.material.0, "metal");

    let flint_stone = registry.get_submaterial(&crafting::SubmaterialId("flint_stone".to_string())).unwrap();
    assert_eq!(flint_stone.material.0, "stone");

    // Count all submaterials
    // 2 leather + 2 wood + 3 metal + 2 bone + 2 fiber + 1 stone = 12
    let submaterial_count = registry.all_submaterials().count();
    assert_eq!(submaterial_count, 12);
}

#[test]
fn test_component_kind_registration() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Test handle accepts wood and bone
    let handle = registry.get_component_kind(&crafting::ComponentKindId("handle".to_string())).unwrap();
    assert_eq!(handle.accepted_materials.len(), 2);
    assert!(handle.accepted_materials.iter().any(|m| m.0 == "wood"));
    assert!(handle.accepted_materials.iter().any(|m| m.0 == "bone"));

    // Test binding accepts leather and fiber
    let binding = registry.get_component_kind(&crafting::ComponentKindId("binding".to_string())).unwrap();
    assert_eq!(binding.accepted_materials.len(), 2);
    assert!(binding.accepted_materials.iter().any(|m| m.0 == "leather"));
    assert!(binding.accepted_materials.iter().any(|m| m.0 == "fiber"));

    // Test scimitar_blade accepts only metal
    let blade = registry.get_component_kind(&crafting::ComponentKindId("scimitar_blade".to_string())).unwrap();
    assert_eq!(blade.accepted_materials.len(), 1);
    assert_eq!(blade.accepted_materials[0].0, "metal");

    // Test knife_blade has makeshift tags
    let knife_blade = registry.get_component_kind(&crafting::ComponentKindId("knife_blade".to_string())).unwrap();
    assert!(knife_blade.makeshift_tags.contains(&"knife".to_string()));

    // Count all component kinds
    let component_kind_count = registry.all_component_kinds().count();
    assert_eq!(component_kind_count, 8);
}

// ============================================================================
// ITEM DEFINITION TESTS - Three ItemKind Types
// ============================================================================

#[test]
fn test_simple_item_with_submaterial() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let deer_leather = registry.get_item(&crafting::ItemId("deer_leather".to_string())).unwrap();
    match &deer_leather.kind {
        crafting::ItemKind::Simple { submaterial } => {
            assert!(submaterial.is_some());
            assert_eq!(submaterial.as_ref().unwrap().0, "deer_leather");
        }
        _ => panic!("deer_leather should be a Simple item with submaterial"),
    }

    let iron_bar = registry.get_item(&crafting::ItemId("iron_bar".to_string())).unwrap();
    match &iron_bar.kind {
        crafting::ItemKind::Simple { submaterial } => {
            assert!(submaterial.is_some());
            assert_eq!(submaterial.as_ref().unwrap().0, "iron_metal");
        }
        _ => panic!("iron_bar should be a Simple item with submaterial"),
    }
}

#[test]
fn test_simple_item_without_submaterial() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let wolf = registry.get_item(&crafting::ItemId("wolf".to_string())).unwrap();
    match &wolf.kind {
        crafting::ItemKind::Simple { submaterial } => {
            assert!(submaterial.is_none());
        }
        _ => panic!("wolf should be a Simple item without submaterial"),
    }

    let iron_ore = registry.get_item(&crafting::ItemId("iron_ore".to_string())).unwrap();
    match &iron_ore.kind {
        crafting::ItemKind::Simple { submaterial } => {
            assert!(submaterial.is_none());
        }
        _ => panic!("iron_ore should be a Simple item without submaterial"),
    }
}

#[test]
fn test_component_item() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let handle = registry.get_item(&crafting::ItemId("handle".to_string())).unwrap();
    match &handle.kind {
        crafting::ItemKind::Component { component_kind } => {
            assert_eq!(component_kind.0, "handle");
        }
        _ => panic!("handle should be a Component item"),
    }

    let binding = registry.get_item(&crafting::ItemId("binding".to_string())).unwrap();
    match &binding.kind {
        crafting::ItemKind::Component { component_kind } => {
            assert_eq!(component_kind.0, "binding");
        }
        _ => panic!("binding should be a Component item"),
    }

    let scimitar_blade = registry.get_item(&crafting::ItemId("scimitar_blade".to_string())).unwrap();
    match &scimitar_blade.kind {
        crafting::ItemKind::Component { component_kind } => {
            assert_eq!(component_kind.0, "scimitar_blade");
        }
        _ => panic!("scimitar_blade should be a Component item"),
    }
}

#[test]
fn test_composite_item() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Test scimitar composite
    let scimitar = registry.get_item(&crafting::ItemId("scimitar".to_string())).unwrap();
    match &scimitar.kind {
        crafting::ItemKind::Composite(def) => {
            assert_eq!(def.slots.len(), 3);
            assert_eq!(def.slots[0].name, "blade");
            assert_eq!(def.slots[0].component_kind.0, "scimitar_blade");
            assert_eq!(def.slots[1].name, "handle");
            assert_eq!(def.slots[1].component_kind.0, "handle");
            assert_eq!(def.slots[2].name, "binding");
            assert_eq!(def.slots[2].component_kind.0, "binding");
            assert!(matches!(def.category, crafting::CompositeCategory::Weapon));
            assert!(def.tool_type.is_none());
        }
        _ => panic!("scimitar should be a Composite item"),
    }

    // Test pickaxe composite (has tool_type)
    let pickaxe = registry.get_item(&crafting::ItemId("pickaxe".to_string())).unwrap();
    match &pickaxe.kind {
        crafting::ItemKind::Composite(def) => {
            assert_eq!(def.slots.len(), 2);
            assert!(matches!(def.category, crafting::CompositeCategory::Tool));
            assert!(matches!(def.tool_type, Some(crafting::ToolType::Pickaxe)));
        }
        _ => panic!("pickaxe should be a Composite item"),
    }
}

#[test]
fn test_item_kinds_are_mutually_exclusive() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Verify an item can only be one kind
    for item in registry.all_items() {
        let is_simple = matches!(item.kind, crafting::ItemKind::Simple { .. });
        let is_component = matches!(item.kind, crafting::ItemKind::Component { .. });
        let is_composite = matches!(item.kind, crafting::ItemKind::Composite(_));

        // Exactly one should be true
        let count = [is_simple, is_component, is_composite].iter().filter(|&&x| x).count();
        assert_eq!(count, 1, "Item {} should be exactly one kind", item.id.0);
    }
}

// ============================================================================
// RECIPE TESTS - Three Recipe Types
// ============================================================================

#[test]
fn test_simple_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Test smelt_iron_bar recipe
    let smelt_iron = registry.get_simple_recipe(&crafting::RecipeId("smelt_iron_bar".to_string())).unwrap();
    assert_eq!(smelt_iron.output.0, "iron_bar");
    assert_eq!(smelt_iron.output_quantity, 1);
    assert_eq!(smelt_iron.inputs.len(), 1);
    assert_eq!(smelt_iron.inputs[0].item_id.0, "iron_ore");
    assert_eq!(smelt_iron.inputs[0].quantity, 2);

    // Test smelt_bronze_bar recipe (multiple inputs)
    let smelt_bronze = registry.get_simple_recipe(&crafting::RecipeId("smelt_bronze_bar".to_string())).unwrap();
    assert_eq!(smelt_bronze.output.0, "bronze_bar");
    assert_eq!(smelt_bronze.inputs.len(), 2);
    assert_eq!(smelt_bronze.inputs[0].item_id.0, "copper_ore");
    assert_eq!(smelt_bronze.inputs[0].quantity, 2);
    assert_eq!(smelt_bronze.inputs[1].item_id.0, "tin_ore");
    assert_eq!(smelt_bronze.inputs[1].quantity, 1);
}

#[test]
fn test_component_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Test craft_handle recipe (no tool requirement)
    let craft_handle = registry.get_component_recipe(&crafting::RecipeId("craft_handle".to_string())).unwrap();
    assert_eq!(craft_handle.output.0, "handle");
    assert!(craft_handle.tool.is_none());

    // Test craft_scimitar_blade recipe (has tool requirement)
    let craft_blade = registry.get_component_recipe(&crafting::RecipeId("craft_scimitar_blade".to_string())).unwrap();
    assert_eq!(craft_blade.output.0, "scimitar_blade");
    assert!(craft_blade.tool.is_some());
    let tool_req = craft_blade.tool.as_ref().unwrap();
    assert!(matches!(tool_req.tool_type, crafting::ToolType::Hammer));
    assert_eq!(tool_req.min_quality, crafting::Quality::Crude);
}

#[test]
fn test_composite_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Test assemble_scimitar recipe
    let assemble_scimitar = registry.get_composite_recipe(&crafting::RecipeId("assemble_scimitar".to_string())).unwrap();
    assert_eq!(assemble_scimitar.output.0, "scimitar");

    // Test assemble_pickaxe recipe
    let assemble_pickaxe = registry.get_composite_recipe(&crafting::RecipeId("assemble_pickaxe".to_string())).unwrap();
    assert_eq!(assemble_pickaxe.output.0, "pickaxe");
}

#[test]
fn test_all_three_recipe_types_registered() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let simple_count = registry.all_simple_recipes().count();
    let component_count = registry.all_component_recipes().count();
    let composite_count = registry.all_composite_recipes().count();

    assert_eq!(simple_count, 2, "Should have 2 simple recipes");
    assert_eq!(component_count, 8, "Should have 8 component recipes");
    assert_eq!(composite_count, 5, "Should have 5 composite recipes");
}

// ============================================================================
// CLI COMMAND TESTS
// ============================================================================

#[test]
fn test_list_items_command() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("list items", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert!(data["count"].as_u64().unwrap() > 0);
    assert!(data["items"].is_array());

    // Verify items have kind labels
    let items = data["items"].as_array().unwrap();
    let deer_leather = items.iter().find(|i| i["id"] == "deer_leather").unwrap();
    assert_eq!(deer_leather["kind"], "Simple (Submaterial)");

    let wolf = items.iter().find(|i| i["id"] == "wolf").unwrap();
    assert_eq!(wolf["kind"], "Simple");

    let handle = items.iter().find(|i| i["id"] == "handle").unwrap();
    assert_eq!(handle["kind"], "Component");

    let scimitar = items.iter().find(|i| i["id"] == "scimitar").unwrap();
    assert_eq!(scimitar["kind"], "Composite");
}

#[test]
fn test_list_recipes_command() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("list recipes", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["count"], 15); // 2 simple + 8 component + 5 composite
    assert!(data["recipes"].is_array());

    // Verify recipes have type labels
    let recipes = data["recipes"].as_array().unwrap();
    let smelt_iron = recipes.iter().find(|r| r["id"] == "smelt_iron_bar").unwrap();
    assert_eq!(smelt_iron["type"], "Simple");

    let craft_handle = recipes.iter().find(|r| r["id"] == "craft_handle").unwrap();
    assert_eq!(craft_handle["type"], "Component");

    let assemble_scimitar = recipes.iter().find(|r| r["id"] == "assemble_scimitar").unwrap();
    assert_eq!(assemble_scimitar["type"], "Composite");
}

#[test]
fn test_show_item_simple_with_submaterial() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show item deer_leather", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "deer_leather");
    assert_eq!(data["name"], "Deer Leather");
    assert_eq!(data["kind"]["type"], "Simple");
    assert_eq!(data["kind"]["submaterial"], "deer_leather");
}

#[test]
fn test_show_item_simple_without_submaterial() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show item wolf", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "wolf");
    assert_eq!(data["kind"]["type"], "Simple");
    assert!(data["kind"]["submaterial"].is_null());
}

#[test]
fn test_show_item_component() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show item handle", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "handle");
    assert_eq!(data["kind"]["type"], "Component");
    assert_eq!(data["kind"]["component_kind"], "handle");
}

#[test]
fn test_show_item_composite() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show item scimitar", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "scimitar");
    assert_eq!(data["kind"]["type"], "Composite");

    let slots = data["kind"]["slots"].as_array().unwrap();
    assert_eq!(slots.len(), 3);
    assert_eq!(slots[0]["name"], "blade");
    assert_eq!(slots[0]["component_kind"], "scimitar_blade");
}

#[test]
fn test_show_nonexistent_item() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show item nonexistent", &mut registry);
    assert_eq!(response["status"], "error");
    assert!(response["message"].as_str().unwrap().contains("not found"));
}

#[test]
fn test_show_simple_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show recipe smelt_bronze_bar", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "smelt_bronze_bar");
    assert_eq!(data["type"], "Simple");
    assert_eq!(data["output"], "bronze_bar");

    let inputs = data["inputs"].as_array().unwrap();
    assert_eq!(inputs.len(), 2);
}

#[test]
fn test_show_component_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show recipe craft_handle", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "craft_handle");
    assert_eq!(data["type"], "Component");
    assert_eq!(data["output"], "handle");
}

#[test]
fn test_show_composite_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show recipe assemble_scimitar", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], "assemble_scimitar");
    assert_eq!(data["type"], "Composite");
    assert_eq!(data["output"], "scimitar");
}

// ============================================================================
// INSTANCE TESTS - Creating and Listing Instances
// ============================================================================

#[test]
fn test_create_simple_instance() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("new deer_leather", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["instance_id"], 0);
    assert_eq!(data["item"], "deer_leather");
}

#[test]
fn test_create_multiple_simple_instances() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response1 = exec_command("new deer_leather", &mut registry);
    assert_eq!(get_data(&response1)["instance_id"], 0);

    let response2 = exec_command("new iron_bar", &mut registry);
    assert_eq!(get_data(&response2)["instance_id"], 1);

    let response3 = exec_command("new wolf", &mut registry);
    assert_eq!(get_data(&response3)["instance_id"], 2);
}

#[test]
fn test_list_instances() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Create some instances
    exec_command("new deer_leather", &mut registry);
    exec_command("new iron_bar", &mut registry);

    let response = exec_command("list instances", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["count"], 2);

    let instances = data["instances"].as_array().unwrap();
    assert_eq!(instances.len(), 2);
    assert_eq!(instances[0]["kind"], "Simple");
    assert_eq!(instances[1]["kind"], "Simple");
}

#[test]
fn test_show_simple_instance() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    exec_command("new deer_leather", &mut registry);

    let response = exec_command("show instance 0", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    assert_eq!(data["id"], 0);
    assert_eq!(data["kind"], "Simple");
    assert_eq!(data["item"], "deer_leather");
}

#[test]
fn test_show_nonexistent_instance() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    let response = exec_command("show instance 999", &mut registry);
    assert_eq!(response["status"], "error");
    assert!(response["message"].as_str().unwrap().contains("not found"));
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[test]
fn test_submaterial_has_correct_parent_material() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // Verify all leather submaterials have leather parent
    let deer_leather = registry.get_submaterial(&crafting::SubmaterialId("deer_leather".to_string())).unwrap();
    assert_eq!(deer_leather.material.0, "leather");

    let wolf_leather = registry.get_submaterial(&crafting::SubmaterialId("wolf_leather".to_string())).unwrap();
    assert_eq!(wolf_leather.material.0, "leather");

    // Verify all wood submaterials have wood parent
    let oak_wood = registry.get_submaterial(&crafting::SubmaterialId("oak_wood".to_string())).unwrap();
    assert_eq!(oak_wood.material.0, "wood");

    let yew_wood = registry.get_submaterial(&crafting::SubmaterialId("yew_wood".to_string())).unwrap();
    assert_eq!(yew_wood.material.0, "wood");

    // Verify all metal submaterials have metal parent
    let iron_metal = registry.get_submaterial(&crafting::SubmaterialId("iron_metal".to_string())).unwrap();
    assert_eq!(iron_metal.material.0, "metal");

    let bronze_metal = registry.get_submaterial(&crafting::SubmaterialId("bronze_metal".to_string())).unwrap();
    assert_eq!(bronze_metal.material.0, "metal");

    let steel_metal = registry.get_submaterial(&crafting::SubmaterialId("steel_metal".to_string())).unwrap();
    assert_eq!(steel_metal.material.0, "metal");
}

#[test]
fn test_composite_slots_match_component_kinds() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // For each composite, verify its slots reference valid component kinds
    for item in registry.all_items() {
        if let crafting::ItemKind::Composite(def) = &item.kind {
            for slot in &def.slots {
                let component_kind = registry.get_component_kind(&slot.component_kind);
                assert!(
                    component_kind.is_some(),
                    "Composite {} has slot {} that references non-existent component_kind {}",
                    item.id.0,
                    slot.name,
                    slot.component_kind.0
                );
            }
        }
    }
}

#[test]
fn test_component_recipes_output_valid_component_kinds() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // For each component recipe, verify its output references a valid component kind
    for recipe in registry.all_component_recipes() {
        let component_kind = registry.get_component_kind(&recipe.output);
        assert!(
            component_kind.is_some(),
            "Component recipe {} outputs non-existent component_kind {}",
            recipe.id.0,
            recipe.output.0
        );
    }
}

#[test]
fn test_simple_recipes_use_valid_items() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // For each simple recipe, verify inputs and outputs are valid items
    for recipe in registry.all_simple_recipes() {
        // Check output
        let output_item = registry.get_item(&recipe.output);
        assert!(
            output_item.is_some(),
            "Simple recipe {} outputs non-existent item {}",
            recipe.id.0,
            recipe.output.0
        );

        // Check inputs
        for input in &recipe.inputs {
            let input_item = registry.get_item(&input.item_id);
            assert!(
                input_item.is_some(),
                "Simple recipe {} requires non-existent item {}",
                recipe.id.0,
                input.item_id.0
            );
        }
    }
}

#[test]
fn test_composite_recipes_output_valid_composites() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    // For each composite recipe, verify output is a composite item
    for recipe in registry.all_composite_recipes() {
        let output_item = registry.get_item(&recipe.output);
        assert!(
            output_item.is_some(),
            "Composite recipe {} outputs non-existent item {}",
            recipe.id.0,
            recipe.output.0
        );

        let item = output_item.unwrap();
        assert!(
            matches!(item.kind, crafting::ItemKind::Composite(_)),
            "Composite recipe {} should output a Composite item, got {:?}",
            recipe.id.0,
            item.kind
        );
    }
}

// ============================================================================
// COMMAND PARSING TESTS
// ============================================================================

#[test]
fn test_parse_list_items() {
    let cmd = cli::parse_command("list items").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ListItems));
}

#[test]
fn test_parse_list_recipes() {
    let cmd = cli::parse_command("list recipes").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ListRecipes));
}

#[test]
fn test_parse_list_instances() {
    let cmd = cli::parse_command("list instances").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ListInstances));
}

#[test]
fn test_parse_show_item() {
    let cmd = cli::parse_command("show item deer_leather").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ShowItem(id) if id == "deer_leather"));
}

#[test]
fn test_parse_show_recipe() {
    let cmd = cli::parse_command("show recipe smelt_iron_bar").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ShowRecipe(id) if id == "smelt_iron_bar"));
}

#[test]
fn test_parse_show_instance() {
    let cmd = cli::parse_command("show instance 42").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::ShowInstance(42)));
}

#[test]
fn test_parse_new() {
    let cmd = cli::parse_command("new deer_leather").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::New { item_id } if item_id == "deer_leather"));
}

#[test]
fn test_parse_help() {
    let cmd = cli::parse_command("help").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::Help));
}

#[test]
fn test_parse_exit() {
    let cmd = cli::parse_command("exit").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::Exit));

    let cmd = cli::parse_command("quit").unwrap();
    assert!(matches!(cmd, crafting::cli::Command::Exit));
}

#[test]
fn test_parse_invalid_commands() {
    // Empty command
    let result = cli::parse_command("");
    assert!(result.is_err());

    // Unknown command
    let result = cli::parse_command("foobar");
    assert!(result.is_err());

    // Incomplete commands
    let result = cli::parse_command("list");
    assert!(result.is_err());

    let result = cli::parse_command("show");
    assert!(result.is_err());

    let result = cli::parse_command("new");
    assert!(result.is_err());
}

// ============================================================================
// HELP COMMAND TEST
// ============================================================================

#[test]
fn test_help_command() {
    let mut registry = Registry::new();

    let response = exec_command("help", &mut registry);
    assert!(is_success(&response));

    let data = get_data(&response);
    let commands = data["commands"].as_array().unwrap();
    assert!(commands.len() > 0);
}

// ============================================================================
// COMPREHENSIVE SYSTEM TEST
// ============================================================================

#[test]
fn test_full_crafting_hierarchy() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);

    println!("\n=== TESTING FULL CRAFTING HIERARCHY ===\n");

    // 1. Verify materials exist
    println!("1. Verifying materials...");
    assert!(registry.get_material(&crafting::MaterialId("leather".to_string())).is_some());
    assert!(registry.get_material(&crafting::MaterialId("wood".to_string())).is_some());
    assert!(registry.get_material(&crafting::MaterialId("metal".to_string())).is_some());
    println!("   ✓ Materials registered");

    // 2. Verify submaterials with parent references
    println!("2. Verifying submaterials...");
    let deer_leather = registry.get_submaterial(&crafting::SubmaterialId("deer_leather".to_string())).unwrap();
    assert_eq!(deer_leather.material.0, "leather");
    let oak_wood = registry.get_submaterial(&crafting::SubmaterialId("oak_wood".to_string())).unwrap();
    assert_eq!(oak_wood.material.0, "wood");
    let iron_metal = registry.get_submaterial(&crafting::SubmaterialId("iron_metal".to_string())).unwrap();
    assert_eq!(iron_metal.material.0, "metal");
    println!("   ✓ Submaterials registered with correct parent materials");

    // 3. Verify component kinds with accepted materials
    println!("3. Verifying component kinds...");
    let handle = registry.get_component_kind(&crafting::ComponentKindId("handle".to_string())).unwrap();
    assert!(handle.accepted_materials.iter().any(|m| m.0 == "wood"));
    assert!(handle.accepted_materials.iter().any(|m| m.0 == "bone"));
    let binding = registry.get_component_kind(&crafting::ComponentKindId("binding".to_string())).unwrap();
    assert!(binding.accepted_materials.iter().any(|m| m.0 == "leather"));
    let blade = registry.get_component_kind(&crafting::ComponentKindId("scimitar_blade".to_string())).unwrap();
    assert!(blade.accepted_materials.iter().any(|m| m.0 == "metal"));
    println!("   ✓ Component kinds registered with correct accepted materials");

    // 4. Verify Simple items (submaterials)
    println!("4. Verifying Simple items with submaterials...");
    let deer_leather_item = registry.get_item(&crafting::ItemId("deer_leather".to_string())).unwrap();
    assert!(matches!(deer_leather_item.kind, crafting::ItemKind::Simple { submaterial: Some(_) }));
    println!("   ✓ Simple items with submaterials registered");

    // 5. Verify Component items
    println!("5. Verifying Component items...");
    let handle_item = registry.get_item(&crafting::ItemId("handle".to_string())).unwrap();
    assert!(matches!(handle_item.kind, crafting::ItemKind::Component { .. }));
    println!("   ✓ Component items registered");

    // 6. Verify Composite items
    println!("6. Verifying Composite items...");
    let scimitar = registry.get_item(&crafting::ItemId("scimitar".to_string())).unwrap();
    if let crafting::ItemKind::Composite(def) = &scimitar.kind {
        assert_eq!(def.slots.len(), 3);
        assert_eq!(def.slots[0].component_kind.0, "scimitar_blade");
        assert_eq!(def.slots[1].component_kind.0, "handle");
        assert_eq!(def.slots[2].component_kind.0, "binding");
    } else {
        panic!("Scimitar should be composite");
    }
    println!("   ✓ Composite items registered with slots");

    // 7. Verify three recipe types
    println!("7. Verifying three recipe types...");
    assert!(registry.get_simple_recipe(&crafting::RecipeId("smelt_iron_bar".to_string())).is_some());
    assert!(registry.get_component_recipe(&crafting::RecipeId("craft_handle".to_string())).is_some());
    assert!(registry.get_composite_recipe(&crafting::RecipeId("assemble_scimitar".to_string())).is_some());
    println!("   ✓ All three recipe types registered");

    // 8. Test instance creation
    println!("8. Testing instance creation...");
    let response = exec_command("new deer_leather", &mut registry);
    assert!(is_success(&response));
    println!("   ✓ Simple instances can be created");

    println!("\n=== ALL HIERARCHY TESTS PASSED ===\n");
}
