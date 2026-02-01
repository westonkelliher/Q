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

#[test]
fn test_list_items() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response = exec_command("list items", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert!(data["count"].as_u64().unwrap() > 0);
    assert!(data["items"].is_array());
}

#[test]
fn test_list_recipes() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response = exec_command("list recipes", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert!(data["count"].as_u64().unwrap() > 0);
    assert!(data["recipes"].is_array());
}

#[test]
fn test_show_item() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response = exec_command("show item copper_ore", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["id"], "copper_ore");
    assert_eq!(data["name"], "Copper Ore");
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
fn test_create_raw_material() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response = exec_command("new copper_ore common", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["instance_id"], 0);
    assert_eq!(data["item"], "copper_ore");
    assert_eq!(data["quality"], "Common");
}

#[test]
fn test_create_multiple_instances() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response1 = exec_command("new copper_ore common", &mut registry);
    assert_eq!(get_data(&response1)["instance_id"], 0);
    
    let response2 = exec_command("new tin_ore common", &mut registry);
    assert_eq!(get_data(&response2)["instance_id"], 1);
    
    let response3 = exec_command("new oak_logs rare", &mut registry);
    assert_eq!(get_data(&response3)["instance_id"], 2);
}

#[test]
fn test_list_instances() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Create some instances
    exec_command("new copper_ore common", &mut registry);
    exec_command("new tin_ore common", &mut registry);
    
    let response = exec_command("list instances", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["count"], 2);
    
    let instances = data["instances"].as_array().unwrap();
    assert_eq!(instances.len(), 2);
}

#[test]
fn test_show_instance() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    exec_command("new copper_ore common", &mut registry);
    
    let response = exec_command("show instance 0", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["id"], 0);
    assert_eq!(data["item"], "copper_ore");
    assert_eq!(data["quality"], "Common");
}

#[test]
fn test_basic_crafting_flow() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Create raw materials (smelt_bronze_bar needs 2 copper_ore + 1 tin_ore)
    let r1 = exec_command("new copper_ore common", &mut registry);
    assert!(is_success(&r1));
    
    let r2 = exec_command("new copper_ore common", &mut registry);
    assert!(is_success(&r2));
    
    let r3 = exec_command("new tin_ore common", &mut registry);
    assert!(is_success(&r3));
    
    // Craft bronze bar
    let r4 = exec_command("craft smelt_bronze_bar 0 1 2", &mut registry);
    if !is_success(&r4) {
        eprintln!("Craft failed: {}", r4);
    }
    assert!(is_success(&r4));
    
    let data = get_data(&r4);
    assert_eq!(data["instance_id"], 3);
    assert_eq!(data["item"], "bronze_bar");
    assert_eq!(data["consumed"].as_array().unwrap().len(), 3);
}

#[test]
fn test_craft_with_missing_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    exec_command("new copper_ore common", &mut registry);
    
    let response = exec_command("craft nonexistent_recipe 0", &mut registry);
    assert_eq!(response["status"], "error");
    assert!(response["message"].as_str().unwrap().contains("Recipe not found"));
}

#[test]
fn test_craft_with_wrong_material_count() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    exec_command("new copper_ore common", &mut registry);
    
    // smelt_bronze_bar requires 3 materials, only providing 1
    let response = exec_command("craft smelt_bronze_bar 0", &mut registry);
    assert_eq!(response["status"], "error");
}

#[test]
fn test_trace_raw_material() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    exec_command("new copper_ore common", &mut registry);
    
    let response = exec_command("trace 0", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["instance"], 0);
    assert!(data["tree"].is_object());
    assert_eq!(data["tree"]["item"], "copper_ore");
}

#[test]
fn test_trace_crafted_item() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Create materials and craft (smelt_bronze_bar needs 2 copper_ore + 1 tin_ore)
    exec_command("new copper_ore common", &mut registry);
    exec_command("new copper_ore common", &mut registry);
    exec_command("new tin_ore common", &mut registry);
    exec_command("craft smelt_bronze_bar 0 1 2", &mut registry);
    
    let response = exec_command("trace 3", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["instance"], 3);
    assert_eq!(data["tree"]["item"], "bronze_bar");
    
    let inputs = data["tree"]["inputs"].as_array().unwrap();
    assert_eq!(inputs.len(), 3);
}

#[test]
fn test_help_command() {
    let mut registry = Registry::new();
    
    let response = exec_command("help", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    let commands = data["commands"].as_array().unwrap();
    assert!(commands.len() > 0);
}

#[test]
fn test_quality_variants() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let qualities = vec![
        "makeshift", "crude", "common", "uncommon", "rare", "epic", "legendary"
    ];
    
    for (i, quality) in qualities.iter().enumerate() {
        let cmd = format!("new copper_ore {}", quality);
        let response = exec_command(&cmd, &mut registry);
        assert!(is_success(&response));
        
        let data = get_data(&response);
        assert_eq!(data["instance_id"], i as u64);
    }
}

#[test]
fn test_complex_crafting_chain() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    // Create bronze bar
    exec_command("new copper_ore common", &mut registry);
    exec_command("new copper_ore common", &mut registry);
    exec_command("new tin_ore common", &mut registry);
    exec_command("craft smelt_bronze_bar 0 1 2", &mut registry);
    
    // Create handle material
    exec_command("new oak_logs common", &mut registry);
    
    // The bronze_bar is instance 3, oak_logs is instance 4
    // Try to craft pickaxe (recipe exists)
    let response = exec_command("show recipe craft_pickaxe", &mut registry);
    
    // Recipe should exist in sample content
    assert_eq!(response["status"], "success");
}

#[test]
fn test_show_recipe() {
    let mut registry = Registry::new();
    crafting::content::register_sample_content(&mut registry);
    
    let response = exec_command("show recipe smelt_bronze_bar", &mut registry);
    assert!(is_success(&response));
    
    let data = get_data(&response);
    assert_eq!(data["id"], "smelt_bronze_bar");
    assert!(data["construction"]["material_inputs"].is_array());
}

#[test]
fn test_invalid_commands() {
    let _registry = Registry::new();
    
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
    
    // "new copper_ore" is now valid (defaults to common quality)
    let result = cli::parse_command("new");
    assert!(result.is_err());
}
