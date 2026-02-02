use crate::{
    ItemId, ItemInstanceId, RecipeId, Registry, Provenance,
    ItemInstance, SimpleInstance, ItemKind,
};
use serde_json::{json, Value};
use std::io::{self, Write as IoWrite};
use colored::*;

/// CLI Commands
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// List all item definitions
    ListItems,
    /// List all recipes
    ListRecipes,
    /// List all item instances
    ListInstances,
    /// Show detailed item definition
    ShowItem(String),
    /// Show recipe details
    ShowRecipe(String),
    /// Show instance details
    ShowInstance(u64),
    /// Create a raw material instance (Simple items only)
    New { item_id: String },
    /// Craft an item using a recipe and inventory indices
    Craft { recipe_id: String, input_indices: Vec<usize> },
    /// Show help
    Help,
    /// Exit REPL
    Exit,
}

/// Parse a command from user input
pub fn parse_command(input: &str) -> Result<Command, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Empty command".to_string());
    }
    
    // Skip comments
    if input.starts_with('#') {
        return Err("Comment".to_string());
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts[0] {
        // Shorthand commands
        "lr" => Ok(Command::ListRecipes),
        "li" => Ok(Command::ListItems),
        "ls" => Ok(Command::ListInstances),
        "c" => {
            if parts.len() < 2 {
                return Err("craft requires: craft <recipe_id> [index1] [index2] ...".to_string());
            }
            let recipe_id = parts[1].to_string();
            let input_indices: Result<Vec<usize>, String> = parts[2..]
                .iter()
                .map(|s| s.parse::<usize>()
                    .map_err(|_| format!("Invalid index: {}", s)))
                .collect();
            let input_indices = input_indices?;
            Ok(Command::Craft { recipe_id, input_indices })
        }
        "i" | "inventory" | "inv" => Ok(Command::ListInstances),
        "si" => {
            if parts.len() < 2 {
                return Err("show item requires: show item <id>".to_string());
            }
            Ok(Command::ShowItem(parts[1].to_string()))
        }
        "sr" => {
            if parts.len() < 2 {
                return Err("show recipe requires: show recipe <id>".to_string());
            }
            Ok(Command::ShowRecipe(parts[1].to_string()))
        }
        "sin" => {
            if parts.len() < 2 {
                return Err("show instance requires: show instance <id>".to_string());
            }
            let id = parts[1].parse::<u64>()
                .map_err(|_| format!("Invalid instance ID: {}", parts[1]))?;
            Ok(Command::ShowInstance(id))
        }
        "n" => {
            if parts.len() < 2 {
                return Err("new requires: new <item_id>".to_string());
            }
            Ok(Command::New { item_id: parts[1].to_string() })
        }
        "h" | "?" => Ok(Command::Help),
        "q" => Ok(Command::Exit),
        "list" => {
            if parts.len() < 2 {
                return Err("list requires a target: items, recipes, or instances".to_string());
            }
            match parts[1] {
                "items" => Ok(Command::ListItems),
                "recipes" => Ok(Command::ListRecipes),
                "instances" => Ok(Command::ListInstances),
                _ => Err(format!("Unknown list target: {}", parts[1])),
            }
        }
        "show" => {
            if parts.len() < 3 {
                return Err("show requires: show <type> <id>".to_string());
            }
            match parts[1] {
                "item" => Ok(Command::ShowItem(parts[2].to_string())),
                "recipe" => Ok(Command::ShowRecipe(parts[2].to_string())),
                "instance" => {
                    let id = parts[2].parse::<u64>()
                        .map_err(|_| format!("Invalid instance ID: {}", parts[2]))?;
                    Ok(Command::ShowInstance(id))
                }
                _ => Err(format!("Unknown show type: {}", parts[1])),
            }
        }
        "new" => {
            if parts.len() < 2 {
                return Err("new requires: new <item_id>".to_string());
            }
            let item_id = parts[1].to_string();
            Ok(Command::New { item_id })
        }
        "craft" => {
            if parts.len() < 2 {
                return Err("craft requires: craft <recipe_id> [index1] [index2] ...".to_string());
            }
            let recipe_id = parts[1].to_string();
            let input_indices: Result<Vec<usize>, String> = parts[2..]
                .iter()
                .map(|s| s.parse::<usize>()
                    .map_err(|_| format!("Invalid index: {}", s)))
                .collect();
            let input_indices = input_indices?;
            Ok(Command::Craft { recipe_id, input_indices })
        }
        "help" => Ok(Command::Help),
        "exit" | "quit" => Ok(Command::Exit),
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}

/// Execute a command against the registry
pub fn execute_command(command: Command, registry: &mut Registry) -> Value {
    match command {
        Command::ListItems => {
            let items: Vec<Value> = registry.all_items()
                .map(|item| {
                    let kind_str = match &item.kind {
                        ItemKind::Simple { submaterial } => {
                            if submaterial.is_some() {
                                "Simple (Submaterial)"
                            } else {
                                "Simple"
                            }
                        }
                        ItemKind::Component { .. } => "Component",
                        ItemKind::Composite(_) => "Composite",
                    };
                    json!({
                        "id": item.id.0,
                        "name": item.name,
                        "kind": kind_str,
                    })
                })
                .collect();
            json!({
                "status": "success",
                "data": {
                    "items": items,
                    "count": items.len()
                }
            })
        }
        Command::ListRecipes => {
            let mut recipes = Vec::new();
            
            for recipe in registry.all_simple_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Simple",
                    "output": recipe.output.0,
                    "quantity": recipe.output_quantity,
                }));
            }
            
            for recipe in registry.all_component_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Component",
                    "output": recipe.output.0,
                }));
            }
            
            for recipe in registry.all_composite_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Composite",
                    "output": recipe.output.0,
                }));
            }
            
            // Add indices to recipes
            let recipes_with_indices: Vec<Value> = recipes.iter()
                .enumerate()
                .map(|(index, recipe)| {
                    let mut recipe_obj = recipe.as_object().unwrap().clone();
                    recipe_obj.insert("index".to_string(), json!(index));
                    json!(recipe_obj)
                })
                .collect();
            
            json!({
                "status": "success",
                "data": {
                    "recipes": recipes_with_indices,
                    "count": recipes_with_indices.len()
                }
            })
        }
        Command::ListInstances => {
            let instances: Vec<(usize, Value)> = registry.all_instances()
                .enumerate()
                .map(|(index, instance)| {
                    let value = match instance {
                        ItemInstance::Simple(i) => json!({
                            "index": index,
                            "id": i.id.0,
                            "kind": "Simple",
                            "item": i.definition.0,
                        }),
                        ItemInstance::Component(i) => json!({
                            "index": index,
                            "id": i.id.0,
                            "kind": "Component",
                            "component_kind": i.component_kind.0,
                            "submaterial": i.submaterial.0,
                        }),
                        ItemInstance::Composite(i) => json!({
                            "index": index,
                            "id": i.id.0,
                            "kind": "Composite",
                            "item": i.definition.0,
                            "quality": format!("{:?}", i.quality),
                        }),
                    };
                    (index, value)
                })
                .collect();
            let instances_values: Vec<Value> = instances.iter().map(|(_, v)| v.clone()).collect();
            json!({
                "status": "success",
                "data": {
                    "instances": instances_values,
                    "count": instances.len()
                }
            })
        }
        Command::ShowItem(id_str) => {
            let item_id = ItemId(id_str.clone());
            match registry.get_item(&item_id) {
                Some(item) => {
                    let kind_data = match &item.kind {
                        ItemKind::Simple { submaterial } => json!({
                            "type": "Simple",
                            "submaterial": submaterial.as_ref().map(|s| &s.0),
                        }),
                        ItemKind::Component { component_kind } => json!({
                            "type": "Component",
                            "component_kind": component_kind.0,
                        }),
                        ItemKind::Composite(def) => json!({
                            "type": "Composite",
                            "category": format!("{:?}", def.category),
                            "tool_type": def.tool_type.as_ref().map(|t| format!("{:?}", t)),
                            "slots": def.slots.iter().map(|slot| json!({
                                "name": slot.name,
                                "component_kind": slot.component_kind.0,
                            })).collect::<Vec<_>>(),
                        }),
                    };
                    json!({
                        "status": "success",
                        "data": {
                            "id": item.id.0,
                            "name": item.name,
                            "description": item.description,
                            "kind": kind_data,
                        }
                    })
                }
                None => json!({
                    "status": "error",
                    "message": format!("Item not found: {}", id_str)
                }),
            }
        }
        Command::ShowRecipe(id_str) => {
            let recipe_id = RecipeId(id_str.clone());
            
            // Try each recipe type
            if let Some(recipe) = registry.get_simple_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Simple",
                        "output": recipe.output.0,
                        "output_quantity": recipe.output_quantity,
                        "inputs": recipe.inputs.iter().map(|i| json!({
                            "item_id": i.item_id.0,
                            "quantity": i.quantity,
                        })).collect::<Vec<_>>(),
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                    }
                });
            }
            
            if let Some(recipe) = registry.get_component_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Component",
                        "output": recipe.output.0,
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                        "note": "Requires one submaterial item whose material is accepted by this component kind"
                    }
                });
            }
            
            if let Some(recipe) = registry.get_composite_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Composite",
                        "output": recipe.output.0,
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                        "note": "Requires component instances matching the composite's slots"
                    }
                });
            }
            
            json!({
                "status": "error",
                "message": format!("Recipe not found: {}", id_str)
            })
        }
        Command::ShowInstance(id) => {
            let instance_id = ItemInstanceId(id);
            match registry.get_instance(instance_id) {
                Some(instance) => json!({
                    "status": "success",
                    "data": serialize_instance(instance)
                }),
                None => json!({
                    "status": "error",
                    "message": format!("Instance not found: {}", id)
                }),
            }
        }
        Command::New { item_id } => {
            let item_id_obj = ItemId(item_id.clone());
            
            // Verify item exists and is Simple
            match registry.get_item(&item_id_obj) {
                Some(item) => {
                    match &item.kind {
                        ItemKind::Simple { .. } => {},
                        _ => return json!({
                            "status": "error",
                            "message": "new command only works with Simple items"
                        }),
                    }
                }
                None => return json!({
                    "status": "error",
                    "message": format!("Item not found: {}", item_id)
                }),
            }
            
            let instance_id = registry.next_instance_id();
            let instance = ItemInstance::Simple(SimpleInstance {
                id: instance_id,
                definition: item_id_obj,
                provenance: Provenance {
                    recipe_id: RecipeId("raw_material".to_string()),
                    consumed_inputs: vec![],
                    tool_used: None,
                    world_object_used: None,
                    crafted_at: 0,
                },
            });
            
            registry.register_instance(instance);
            
            json!({
                "status": "success",
                "data": {
                    "instance_id": instance_id.0,
                    "item": item_id,
                }
            })
        }
        Command::Craft { recipe_id, input_indices } => {
            // Collect all instances into a vector for indexing
            let instances_vec: Vec<ItemInstanceId> = registry.all_instances()
                .map(|inst| inst.id())
                .collect();
            
            // Validate indices
            for &index in &input_indices {
                if index >= instances_vec.len() {
                    return json!({
                        "status": "error",
                        "message": format!("Invalid inventory index: {}. Inventory has {} items (indices 0-{})", 
                            index, instances_vec.len(), 
                            if instances_vec.is_empty() { 0 } else { instances_vec.len() - 1 })
                    });
                }
            }
            
            // Map indices to instance IDs
            let input_instance_ids: Vec<ItemInstanceId> = input_indices.iter()
                .map(|&idx| instances_vec[idx])
                .collect();
            
            // Find the recipe
            let recipe_id_obj = RecipeId(recipe_id.clone());
            
            // Try Simple recipe first
            if let Some(recipe) = registry.get_simple_recipe(&recipe_id_obj) {
                let recipe_clone = recipe.clone();
                match registry.execute_simple_recipe(&recipe_clone, input_instance_ids.clone(), None, None) {
                    Ok(new_instance) => {
                        // Remove consumed instances
                        for &id in &input_instance_ids {
                            registry.remove_instance(id);
                        }
                        
                        // Register the new instance
                        let new_id = new_instance.id();
                        registry.register_instance(new_instance);
                        
                        json!({
                            "status": "success",
                            "data": {
                                "instance_id": new_id.0,
                                "recipe": recipe_id,
                                "type": "Simple"
                            }
                        })
                    }
                    Err(e) => json!({
                        "status": "error",
                        "message": e
                    })
                }
            }
            // Try Component recipe
            else if let Some(recipe) = registry.get_component_recipe(&recipe_id_obj) {
                if input_instance_ids.len() != 1 {
                    return json!({
                        "status": "error",
                        "message": format!("Component recipe requires exactly 1 input, got {}", input_instance_ids.len())
                    });
                }
                
                let recipe_clone = recipe.clone();
                match registry.execute_component_recipe(&recipe_clone, input_instance_ids[0], None, None) {
                    Ok(new_instance) => {
                        // Remove consumed instance
                        registry.remove_instance(input_instance_ids[0]);
                        
                        // Register the new instance
                        let new_id = new_instance.id();
                        registry.register_instance(new_instance);
                        
                        json!({
                            "status": "success",
                            "data": {
                                "instance_id": new_id.0,
                                "recipe": recipe_id,
                                "type": "Component"
                            }
                        })
                    }
                    Err(e) => json!({
                        "status": "error",
                        "message": e
                    })
                }
            }
            // Try Composite recipe
            else if let Some(recipe) = registry.get_composite_recipe(&recipe_id_obj) {
                // Clone recipe and get output item info before mutable borrow
                let recipe_clone = recipe.clone();
                let output_item_id = recipe.output.clone();
                
                // Get the output item to determine slot order
                let output_def = match registry.get_item(&output_item_id) {
                    Some(def) => def,
                    None => return json!({
                        "status": "error",
                        "message": format!("Output item {:?} not found", output_item_id)
                    })
                };
                
                let composite_def = match &output_def.kind {
                    crate::ItemKind::Composite(def) => def,
                    _ => return json!({
                        "status": "error",
                        "message": format!("Recipe output {:?} is not a Composite item", output_item_id)
                    })
                };
                
                // Match indices to slots in order
                if input_instance_ids.len() != composite_def.slots.len() {
                    return json!({
                        "status": "error",
                        "message": format!("Composite recipe requires {} components (one per slot), got {}", 
                            composite_def.slots.len(), input_instance_ids.len())
                    });
                }
                
                let provided_components: Vec<(String, ItemInstanceId)> = composite_def.slots.iter()
                    .zip(input_instance_ids.iter())
                    .map(|(slot, &id)| (slot.name.clone(), id))
                    .collect();
                
                match registry.execute_composite_recipe(&recipe_clone, provided_components.clone(), None, None) {
                    Ok(new_instance) => {
                        // Remove consumed instances
                        for (_, id) in provided_components {
                            registry.remove_instance(id);
                        }
                        
                        // Register the new instance
                        let new_id = new_instance.id();
                        registry.register_instance(new_instance);
                        
                        json!({
                            "status": "success",
                            "data": {
                                "instance_id": new_id.0,
                                "recipe": recipe_id,
                                "type": "Composite"
                            }
                        })
                    }
                    Err(e) => json!({
                        "status": "error",
                        "message": e
                    })
                }
            }
            else {
                json!({
                    "status": "error",
                    "message": format!("Recipe not found: {}", recipe_id)
                })
            }
        }
        Command::Help => {
            json!({
                "status": "success",
                "data": {
                    "commands": [
                        {"command": "inventory (i/inv/ls)", "description": "Show your inventory (items with indices)"},
                        {"command": "list items (li)", "description": "List all item definitions"},
                        {"command": "list recipes (lr)", "description": "List all recipes"},
                        {"command": "list instances (ls)", "description": "List all item instances (with indices)"},
                        {"command": "show item <id> (si)", "description": "Show detailed item definition"},
                        {"command": "show recipe <id> (sr)", "description": "Show recipe with requirements"},
                        {"command": "show instance <id> (sin)", "description": "Show instance details"},
                        {"command": "new <item_id> (n)", "description": "Create raw Simple material instance"},
                        {"command": "craft <recipe_id> [index1] [index2] ... (c)", "description": "Craft an item using a recipe and inventory indices"},
                        {"command": "help (h/?)", "description": "Show this help"},
                        {"command": "exit (q)", "description": "Exit REPL"},
                    ],
                    "note": "Shorthands: i/inv/ls (inventory), lr (list recipes), li (list items), c (craft), si (show item), sr (show recipe), sin (show instance), n (new), h/? (help), q (exit). Use 'inventory' to see numbered items. Use those numbers with 'craft' command. Use --human-readable flag for readable output format."
                }
            })
        }
        Command::Exit => {
            json!({
                "status": "exit"
            })
        }
    }
}

/// Serialize an instance to JSON
fn serialize_instance(instance: &ItemInstance) -> Value {
    match instance {
        ItemInstance::Simple(i) => json!({
            "id": i.id.0,
            "kind": "Simple",
            "item": i.definition.0,
            "provenance": {
                "recipe": i.provenance.recipe_id.0,
                "consumed": i.provenance.consumed_inputs.iter().map(|ci| json!({
                    "instance_id": ci.instance_id.0,
                    "quantity": ci.quantity,
                })).collect::<Vec<_>>(),
            }
        }),
        ItemInstance::Component(i) => json!({
            "id": i.id.0,
            "kind": "Component",
            "component_kind": i.component_kind.0,
            "submaterial": i.submaterial.0,
            "provenance": {
                "recipe": i.provenance.recipe_id.0,
                "consumed": i.provenance.consumed_inputs.iter().map(|ci| json!({
                    "instance_id": ci.instance_id.0,
                    "quantity": ci.quantity,
                })).collect::<Vec<_>>(),
            }
        }),
        ItemInstance::Composite(i) => json!({
            "id": i.id.0,
            "kind": "Composite",
            "item": i.definition.0,
            "quality": format!("{:?}", i.quality),
            "components": i.components.iter().map(|(name, comp)| {
                (name.clone(), json!({
                    "component_kind": comp.component_kind.0,
                    "submaterial": comp.submaterial.0,
                }))
            }).collect::<serde_json::Map<_, _>>(),
            "provenance": {
                "recipe": i.provenance.recipe_id.0,
                "consumed": i.provenance.consumed_inputs.iter().map(|ci| json!({
                    "instance_id": ci.instance_id.0,
                    "quantity": ci.quantity,
                })).collect::<Vec<_>>(),
            }
        }),
    }
}

/// Format JSON output in human-readable form
fn format_human_readable(value: &Value) -> String {
    // Enable colors only if stdout is a TTY
    let use_colors = atty::is(atty::Stream::Stdout);
    colored::control::set_override(use_colors);
    
    match value {
        Value::Object(map) => {
            if let Some(status) = map.get("status") {
                match status.as_str() {
                    Some("success") => {
                        if let Some(data) = map.get("data") {
                            format_human_readable_data(data)
                        } else {
                            "Success".green().to_string()
                        }
                    }
                    Some("error") => {
                        if let Some(msg) = map.get("message") {
                            format!("{}: {}", "Error".red().bold(), msg.as_str().unwrap_or("Unknown error").red())
                        } else {
                            "Error".red().bold().to_string()
                        }
                    }
                    Some("exit") => "Exiting...".yellow().to_string(),
                    _ => serde_json::to_string_pretty(value).unwrap_or_default()
                }
            } else {
                serde_json::to_string_pretty(value).unwrap_or_default()
            }
        }
        _ => serde_json::to_string_pretty(value).unwrap_or_default()
    }
}

/// Format the data section of a success response
fn format_human_readable_data(data: &Value) -> String {
    let mut output = String::new();
    
    if let Some(items) = data.get("items") {
        if let Some(items_array) = items.as_array() {
            output.push_str(&format!("{} ({}):\n", "Items".bold().cyan(), items_array.len().to_string().cyan()));
            for item in items_array {
                if let Some(obj) = item.as_object() {
                    let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                    let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                    let kind = obj.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
                    output.push_str(&format!("  {} {} {}\n", 
                        format!("[{}]", id).cyan().bold(),
                        name.bold(),
                        format!("[{}]", kind).yellow()));
                }
            }
        }
    }
    
    if let Some(recipes) = data.get("recipes") {
        if let Some(recipes_array) = recipes.as_array() {
            output.push_str(&format!("{} ({}):\n", "Recipes".bold().cyan(), recipes_array.len().to_string().cyan()));
            for recipe in recipes_array {
                if let Some(obj) = recipe.as_object() {
                    let index = obj.get("index").and_then(|v| v.as_u64()).map(|v| v.to_string()).unwrap_or_else(|| "?".to_string());
                    let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                    let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                    let rtype = obj.get("type").and_then(|v| v.as_str()).unwrap_or("?");
                    let output_item = obj.get("output").and_then(|v| v.as_str()).unwrap_or("?");
                    if let Some(qty) = obj.get("quantity").and_then(|v| v.as_u64()) {
                        output.push_str(&format!("  {} {} {} {} {} {} {}\n", 
                            format!("[{}]", index).bright_magenta().bold(),
                            format!("[{}]", id).cyan().bold(),
                            name.bold(),
                            format!("[{}]", rtype).yellow(),
                            "->".bright_black(),
                            output_item.green(),
                            format!("x{}", qty).green()));
                    } else {
                        output.push_str(&format!("  {} {} {} {} {} {}\n", 
                            format!("[{}]", index).bright_magenta().bold(),
                            format!("[{}]", id).cyan().bold(),
                            name.bold(),
                            format!("[{}]", rtype).yellow(),
                            "->".bright_black(),
                            output_item.green()));
                    }
                }
            }
        }
    }
    
    if let Some(instances) = data.get("instances") {
        if let Some(instances_array) = instances.as_array() {
            output.push_str(&format!("{} ({}):\n", "Inventory".bold().cyan(), instances_array.len().to_string().cyan()));
            for instance in instances_array {
                if let Some(obj) = instance.as_object() {
                    let index = obj.get("index").and_then(|v| v.as_u64()).map(|v| v.to_string()).unwrap_or_else(|| "?".to_string());
                    let id = obj.get("id").and_then(|v| v.as_u64()).map(|v| v.to_string()).unwrap_or_else(|| "?".to_string());
                    let kind = obj.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
                    if let Some(item) = obj.get("item").and_then(|v| v.as_str()) {
                        output.push_str(&format!("  {} {} {} {} {} {}\n", 
                            format!("[{}]", index).bright_magenta().bold(),
                            format!("Instance #{}", index).white(),
                            format!("(id: {})", id).cyan(),
                            format!("[{}]", kind).yellow(),
                            "->".bright_black(),
                            item.green()));
                    } else if let Some(comp_kind) = obj.get("component_kind").and_then(|v| v.as_str()) {
                        let submaterial = obj.get("submaterial").and_then(|v| v.as_str()).unwrap_or("?");
                        output.push_str(&format!("  {} {} {} {} {} {} {}\n", 
                            format!("[{}]", index).bright_magenta().bold(),
                            format!("Instance #{}", index).white(),
                            format!("(id: {})", id).cyan(),
                            format!("[{}]", kind).yellow(),
                            "->".bright_black(),
                            comp_kind.green(),
                            format!("({})", submaterial).bright_black()));
                    } else {
                        output.push_str(&format!("  {} {} {} {}\n", 
                            format!("[{}]", index).bright_magenta().bold(),
                            format!("Instance #{}", index).white(),
                            format!("(id: {})", id).cyan(),
                            format!("[{}]", kind).yellow()));
                    }
                }
            }
        }
    }
    
    if let Some(item_obj) = data.as_object() {
        // Show item details
        if item_obj.contains_key("id") && item_obj.contains_key("name") {
            let id = item_obj.get("id").and_then(|v| v.as_str()).unwrap_or("?");
            let name = item_obj.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            output.push_str(&format!("{}: {} ({})\n", "Item".bold().cyan(), name.bold(), format!("{}", id).cyan()));
            
            if let Some(desc) = item_obj.get("description").and_then(|v| v.as_str()) {
                output.push_str(&format!("{}: {}\n", "Description".bright_black(), desc));
            }
            
            if let Some(kind) = item_obj.get("kind") {
                output.push_str(&format!("{}: {}\n", "Kind".bright_black(), format_kind(kind)));
            }
        }
        
        // Show recipe details
        if item_obj.contains_key("type") && item_obj.get("type").and_then(|v| v.as_str()) == Some("Simple") {
            let id = item_obj.get("id").and_then(|v| v.as_str()).unwrap_or("?");
            let name = item_obj.get("name").and_then(|v| v.as_str()).unwrap_or("?");
            let output_item = item_obj.get("output").and_then(|v| v.as_str()).unwrap_or("?");
            let qty = item_obj.get("output_quantity").and_then(|v| v.as_u64()).unwrap_or(1);
            output.push_str(&format!("Recipe: {} ({})\n", name, id));
            output.push_str(&format!("Output: {} x{}\n", output_item, qty));
            
            if let Some(inputs) = item_obj.get("inputs").and_then(|v| v.as_array()) {
                if !inputs.is_empty() {
                    output.push_str("Inputs:\n");
                    for input in inputs {
                        if let Some(inp_obj) = input.as_object() {
                            let item_id = inp_obj.get("item_id").and_then(|v| v.as_str()).unwrap_or("?");
                            let qty = inp_obj.get("quantity").and_then(|v| v.as_u64()).unwrap_or(1);
                            output.push_str(&format!("  - {} x{}\n", item_id, qty));
                        }
                    }
                }
            }
            
            if let Some(tool) = item_obj.get("tool") {
                output.push_str(&format!("Tool: {}\n", format_tool(tool)));
            }
        }
        
        // Show instance details
        if item_obj.contains_key("kind") && !item_obj.contains_key("name") {
            let id = item_obj.get("id").and_then(|v| v.as_u64()).map(|v| v.to_string()).unwrap_or_else(|| "?".to_string());
            let kind = item_obj.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
            output.push_str(&format!("Instance #{} [{}]\n", id, kind));
            
            if let Some(item) = item_obj.get("item").and_then(|v| v.as_str()) {
                output.push_str(&format!("Item: {}\n", item));
            }
            
            if let Some(comp_kind) = item_obj.get("component_kind").and_then(|v| v.as_str()) {
                output.push_str(&format!("Component Kind: {}\n", comp_kind));
            }
            
            if let Some(submaterial) = item_obj.get("submaterial").and_then(|v| v.as_str()) {
                output.push_str(&format!("Submaterial: {}\n", submaterial));
            }
            
            if let Some(quality) = item_obj.get("quality").and_then(|v| v.as_str()) {
                output.push_str(&format!("Quality: {}\n", quality));
            }
            
            if let Some(components) = item_obj.get("components").and_then(|v| v.as_object()) {
                if !components.is_empty() {
                    output.push_str("Components:\n");
                    for (slot_name, comp) in components {
                        if let Some(comp_obj) = comp.as_object() {
                            let comp_kind = comp_obj.get("component_kind").and_then(|v| v.as_str()).unwrap_or("?");
                            let submaterial = comp_obj.get("submaterial").and_then(|v| v.as_str()).unwrap_or("?");
                            output.push_str(&format!("  - {}: {} ({})\n", slot_name, comp_kind, submaterial));
                        }
                    }
                }
            }
            
            if let Some(provenance) = item_obj.get("provenance") {
                output.push_str(&format!("Provenance: {}\n", format_provenance(provenance)));
            }
        }
        
        // Show commands help
        if let Some(commands) = item_obj.get("commands").and_then(|v| v.as_array()) {
            output.push_str(&format!("{}\n", "Available Commands:".bold().cyan()));
            for cmd in commands {
                if let Some(cmd_obj) = cmd.as_object() {
                    let cmd_str = cmd_obj.get("command").and_then(|v| v.as_str()).unwrap_or("?");
                    let desc = cmd_obj.get("description").and_then(|v| v.as_str()).unwrap_or("?");
                    output.push_str(&format!("  {} - {}\n", cmd_str.cyan().bold(), desc));
                }
            }
            if let Some(note) = item_obj.get("note").and_then(|v| v.as_str()) {
                output.push_str(&format!("\n{}: {}\n", "Note".yellow().bold(), note.bright_black()));
            }
        }
        
        // Show instance_id from new command
        if let Some(instance_id) = item_obj.get("instance_id").and_then(|v| v.as_u64()) {
            if let Some(recipe) = item_obj.get("recipe").and_then(|v| v.as_str()) {
                // This is a craft command result
                let craft_type = item_obj.get("type").and_then(|v| v.as_str()).unwrap_or("?");
                output.push_str(&format!("{} {} {} {} {}\n", 
                    "Crafted".green().bold(),
                    craft_type.green(),
                    format!("instance #{}", instance_id).cyan(),
                    "using recipe:".bright_black(),
                    recipe.cyan().bold()));
            } else {
                // This is a new command result
                let item = item_obj.get("item").and_then(|v| v.as_str()).unwrap_or("?");
                output.push_str(&format!("{} {} {} {}\n", 
                    "Created".green().bold(),
                    format!("instance #{}", instance_id).cyan(),
                    "for item:".bright_black(),
                    item.cyan().bold()));
            }
        }
    }
    
    if output.is_empty() {
        serde_json::to_string_pretty(data).unwrap_or_default()
    } else {
        output.trim_end().to_string()
    }
}

fn format_kind(kind: &Value) -> String {
    if let Some(obj) = kind.as_object() {
        if let Some(typ) = obj.get("type").and_then(|v| v.as_str()) {
            match typ {
                "Simple" => {
                    if let Some(submaterial) = obj.get("submaterial").and_then(|v| v.as_str()) {
                        format!("Simple (Submaterial: {})", submaterial)
                    } else {
                        "Simple".to_string()
                    }
                }
                "Component" => {
                    if let Some(comp_kind) = obj.get("component_kind").and_then(|v| v.as_str()) {
                        format!("Component ({})", comp_kind)
                    } else {
                        "Component".to_string()
                    }
                }
                "Composite" => {
                    let mut parts = vec!["Composite".to_string()];
                    if let Some(cat) = obj.get("category").and_then(|v| v.as_str()) {
                        parts.push(format!("Category: {}", cat));
                    }
                    if let Some(tool_type) = obj.get("tool_type").and_then(|v| v.as_str()) {
                        parts.push(format!("Tool Type: {}", tool_type));
                    }
                    if let Some(slots) = obj.get("slots").and_then(|v| v.as_array()) {
                        parts.push(format!("Slots: {}", slots.len()));
                    }
                    parts.join(", ")
                }
                _ => typ.to_string()
            }
        } else {
            serde_json::to_string_pretty(kind).unwrap_or_default()
        }
    } else {
        serde_json::to_string_pretty(kind).unwrap_or_default()
    }
}

fn format_tool(tool: &Value) -> String {
    if let Some(obj) = tool.as_object() {
        let mut parts = vec![];
        if let Some(typ) = obj.get("type").and_then(|v| v.as_str()) {
            parts.push(typ.to_string());
        }
        if let Some(min_quality) = obj.get("min_quality").and_then(|v| v.as_str()) {
            if min_quality != "None" {
                parts.push(format!("Min Quality: {}", min_quality));
            }
        }
        if parts.is_empty() {
            "None".to_string()
        } else {
            parts.join(", ")
        }
    } else {
        "None".to_string()
    }
}

fn format_provenance(provenance: &Value) -> String {
    if let Some(obj) = provenance.as_object() {
        let mut parts = vec![];
        if let Some(recipe) = obj.get("recipe").and_then(|v| v.as_str()) {
            parts.push(format!("Recipe: {}", recipe));
        }
        if let Some(consumed) = obj.get("consumed").and_then(|v| v.as_array()) {
            if !consumed.is_empty() {
                let consumed_strs: Vec<String> = consumed.iter()
                    .filter_map(|c| {
                        if let Some(c_obj) = c.as_object() {
                            let inst_id = c_obj.get("instance_id").and_then(|v| v.as_u64())?;
                            let qty = c_obj.get("quantity").and_then(|v| v.as_u64()).unwrap_or(1);
                            Some(format!("#{} x{}", inst_id, qty))
                        } else {
                            None
                        }
                    })
                    .collect();
                if !consumed_strs.is_empty() {
                    parts.push(format!("Consumed: {}", consumed_strs.join(", ")));
                }
            }
        }
        if parts.is_empty() {
            "None".to_string()
        } else {
            parts.join(", ")
        }
    } else {
        "None".to_string()
    }
}

/// Run the REPL loop
pub fn run_repl(registry: &mut Registry, human_readable: bool) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        // Don't print prompt if not a TTY (for script testing)
        if atty::is(atty::Stream::Stdin) {
            print!("> ");
            stdout.flush()?;
        }
        
        let mut input = String::new();
        if stdin.read_line(&mut input)? == 0 {
            // EOF reached
            break;
        }
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        match parse_command(input) {
            Ok(command) => {
                let result = execute_command(command, registry);
                
                // Check for exit command
                if result["status"] == "exit" {
                    break;
                }
                
                if human_readable {
                    println!("{}", format_human_readable(&result));
                } else {
                    println!("{}", serde_json::to_string(&result).unwrap());
                }
            }
            Err(err) => {
                // Don't print errors for comments
                if err != "Comment" {
                    let error_response = json!({
                        "status": "error",
                        "message": err
                    });
                    if human_readable {
                        println!("{}", format_human_readable(&error_response));
                    } else {
                        println!("{}", serde_json::to_string(&error_response).unwrap());
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_items() {
        let cmd = parse_command("list items").unwrap();
        assert_eq!(cmd, Command::ListItems);
    }

    #[test]
    fn test_parse_list_recipes() {
        let cmd = parse_command("list recipes").unwrap();
        assert_eq!(cmd, Command::ListRecipes);
    }

    #[test]
    fn test_parse_list_instances() {
        let cmd = parse_command("list instances").unwrap();
        assert_eq!(cmd, Command::ListInstances);
    }

    #[test]
    fn test_parse_inventory() {
        let cmd = parse_command("inventory").unwrap();
        assert_eq!(cmd, Command::ListInstances);
        
        let cmd = parse_command("inv").unwrap();
        assert_eq!(cmd, Command::ListInstances);
        
        let cmd = parse_command("i").unwrap();
        assert_eq!(cmd, Command::ListInstances);
    }

    #[test]
    fn test_parse_shorthands() {
        // List recipes shorthand
        let cmd = parse_command("lr").unwrap();
        assert_eq!(cmd, Command::ListRecipes);
        
        // List items shorthand
        let cmd = parse_command("li").unwrap();
        assert_eq!(cmd, Command::ListItems);
        
        // List instances shorthand
        let cmd = parse_command("ls").unwrap();
        assert_eq!(cmd, Command::ListInstances);
        
        // Craft shorthand
        let cmd = parse_command("c recipe_id 0 1").unwrap();
        assert_eq!(cmd, Command::Craft {
            recipe_id: "recipe_id".to_string(),
            input_indices: vec![0, 1],
        });
        
        // Show item shorthand
        let cmd = parse_command("si item_id").unwrap();
        assert_eq!(cmd, Command::ShowItem("item_id".to_string()));
        
        // Show recipe shorthand
        let cmd = parse_command("sr recipe_id").unwrap();
        assert_eq!(cmd, Command::ShowRecipe("recipe_id".to_string()));
        
        // Show instance shorthand
        let cmd = parse_command("sin 123").unwrap();
        assert_eq!(cmd, Command::ShowInstance(123));
        
        // New shorthand
        let cmd = parse_command("n item_id").unwrap();
        assert_eq!(cmd, Command::New {
            item_id: "item_id".to_string(),
        });
        
        // Help shorthand
        let cmd = parse_command("h").unwrap();
        assert_eq!(cmd, Command::Help);
        
        let cmd = parse_command("?").unwrap();
        assert_eq!(cmd, Command::Help);
        
        // Exit shorthand
        let cmd = parse_command("q").unwrap();
        assert_eq!(cmd, Command::Exit);
    }

    #[test]
    fn test_parse_show_item() {
        let cmd = parse_command("show item copper_ore").unwrap();
        assert_eq!(cmd, Command::ShowItem("copper_ore".to_string()));
    }

    #[test]
    fn test_parse_new() {
        let cmd = parse_command("new copper_ore").unwrap();
        assert_eq!(cmd, Command::New {
            item_id: "copper_ore".to_string(),
        });
    }

    #[test]
    fn test_parse_help() {
        let cmd = parse_command("help").unwrap();
        assert_eq!(cmd, Command::Help);
    }

    #[test]
    fn test_parse_exit() {
        let cmd = parse_command("exit").unwrap();
        assert_eq!(cmd, Command::Exit);
        
        let cmd = parse_command("quit").unwrap();
        assert_eq!(cmd, Command::Exit);
    }
}
