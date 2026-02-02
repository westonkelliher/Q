use crate::{
    ItemId, ItemInstanceId, Quality, RecipeId, Registry, Provenance,
    ItemInstance, SimpleInstance, ItemKind,
};
use serde_json::{json, Value};
use std::io::{self, Write as IoWrite};

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
    /// Create a raw material instance
    New { item_id: String, quality: Option<Quality> },
    /// Execute a recipe with given material instances
    Craft { recipe_id: String, instance_ids: Vec<u64> },
    /// Show full provenance tree
    Trace(u64),
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
                return Err("new requires: new <item_id> [quality]".to_string());
            }
            let item_id = parts[1].to_string();
            let quality = if parts.len() >= 3 {
                Some(parse_quality(parts[2])?)
            } else {
                None // Will inherit from ItemDefinition.default_quality
            };
            Ok(Command::New { item_id, quality })
        }
        "craft" => {
            if parts.len() < 2 {
                return Err("craft requires: craft <recipe_id> [instance_id...]".to_string());
            }
            let recipe_id = parts[1].to_string();
            let mut instance_ids = Vec::new();
            for id_str in &parts[2..] {
                let id = id_str.parse::<u64>()
                    .map_err(|_| format!("Invalid instance ID: {}", id_str))?;
                instance_ids.push(id);
            }
            Ok(Command::Craft { recipe_id, instance_ids })
        }
        "trace" => {
            if parts.len() < 2 {
                return Err("trace requires: trace <instance_id>".to_string());
            }
            let id = parts[1].parse::<u64>()
                .map_err(|_| format!("Invalid instance ID: {}", parts[1]))?;
            Ok(Command::Trace(id))
        }
        "help" => Ok(Command::Help),
        "exit" | "quit" => Ok(Command::Exit),
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}

/// Parse a quality string
fn parse_quality(s: &str) -> Result<Quality, String> {
    match s.to_lowercase().as_str() {
        "makeshift" => Ok(Quality::Makeshift),
        "crude" => Ok(Quality::Crude),
        "common" => Ok(Quality::Common),
        "uncommon" => Ok(Quality::Uncommon),
        "rare" => Ok(Quality::Rare),
        "epic" => Ok(Quality::Epic),
        "legendary" => Ok(Quality::Legendary),
        _ => Err(format!("Unknown quality: {}", s)),
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
                                "Simple (submaterial)"
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
                        "kind": kind_str
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
            let mut recipes: Vec<Value> = Vec::new();

            // Add all simple recipes
            for recipe in registry.all_simple_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Simple",
                    "output": recipe.output.0,
                    "quantity": recipe.output_quantity,
                }));
            }

            // Add all component recipes
            for recipe in registry.all_component_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Component",
                    "output": recipe.output.0,
                }));
            }

            // Add all composite recipes
            for recipe in registry.all_composite_recipes() {
                recipes.push(json!({
                    "id": recipe.id.0,
                    "name": recipe.name,
                    "type": "Composite",
                    "output": recipe.output.0,
                }));
            }

            json!({
                "status": "success",
                "data": {
                    "recipes": recipes,
                    "count": recipes.len()
                }
            })
        }
        Command::ListInstances => {
            let instances: Vec<Value> = registry.all_instances()
                .map(|instance| {
                    let (item_id, instance_type) = match instance {
                        ItemInstance::Simple(s) => (s.definition.0.clone(), "Simple"),
                        ItemInstance::Component(c) => (format!("{} (component)", c.component_kind.0), "Component"),
                        ItemInstance::Composite(c) => (c.definition.0.clone(), "Composite"),
                    };
                    json!({
                        "id": instance.id().0,
                        "item": item_id,
                        "type": instance_type,
                    })
                })
                .collect();
            json!({
                "status": "success",
                "data": {
                    "instances": instances,
                    "count": instances.len()
                }
            })
        }
        Command::ShowItem(id_str) => {
            let item_id = ItemId(id_str.clone());
            match registry.get_item(&item_id) {
                Some(item) => {
                    let mut data = json!({
                        "id": item.id.0,
                        "name": item.name,
                        "description": item.description,
                    });

                    // Add kind-specific data
                    match &item.kind {
                        ItemKind::Simple { submaterial } => {
                            data["kind"] = json!("Simple");
                            if let Some(submat_id) = submaterial {
                                data["submaterial"] = json!(submat_id.0);
                            }
                        }
                        ItemKind::Component { component_kind } => {
                            data["kind"] = json!("Component");
                            data["component_kind"] = json!(component_kind.0);
                        }
                        ItemKind::Composite(composite_def) => {
                            data["kind"] = json!("Composite");
                            data["slots"] = json!(composite_def.slots.iter().map(|slot| json!({
                                "name": slot.name,
                                "component_kind": slot.component_kind.0,
                            })).collect::<Vec<_>>());
                            data["category"] = json!(format!("{:?}", composite_def.category));
                            if let Some(tool_type) = &composite_def.tool_type {
                                data["tool_type"] = json!(format!("{:?}", tool_type));
                            }
                        }
                    }

                    json!({
                        "status": "success",
                        "data": data
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

            // Try to find as simple recipe
            if let Some(recipe) = registry.get_simple_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Simple",
                        "output": recipe.output.0,
                        "output_quantity": recipe.output_quantity,
                        "inputs": recipe.inputs.iter().map(|input| json!({
                            "item_id": input.item_id.0,
                            "quantity": input.quantity,
                        })).collect::<Vec<_>>(),
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "tool_type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                        "world_object": recipe.world_object.as_ref().map(|wo| json!({
                            "kind": wo.kind.as_ref().map(|k| format!("{:?}", k)),
                            "required_tags": wo.required_tags.iter().map(|t| &t.0).collect::<Vec<_>>(),
                        })),
                    }
                });
            }

            // Try to find as component recipe
            if let Some(recipe) = registry.get_component_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Component",
                        "output": recipe.output.0,
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "tool_type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                        "world_object": recipe.world_object.as_ref().map(|wo| json!({
                            "kind": wo.kind.as_ref().map(|k| format!("{:?}", k)),
                            "required_tags": wo.required_tags.iter().map(|t| &t.0).collect::<Vec<_>>(),
                        })),
                    }
                });
            }

            // Try to find as composite recipe
            if let Some(recipe) = registry.get_composite_recipe(&recipe_id) {
                return json!({
                    "status": "success",
                    "data": {
                        "id": recipe.id.0,
                        "name": recipe.name,
                        "type": "Composite",
                        "output": recipe.output.0,
                        "tool": recipe.tool.as_ref().map(|t| json!({
                            "tool_type": format!("{:?}", t.tool_type),
                            "min_quality": format!("{:?}", t.min_quality),
                        })),
                        "world_object": recipe.world_object.as_ref().map(|wo| json!({
                            "kind": wo.kind.as_ref().map(|k| format!("{:?}", k)),
                            "required_tags": wo.required_tags.iter().map(|t| &t.0).collect::<Vec<_>>(),
                        })),
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
        Command::New { item_id, quality: _ } => {
            let item_id_obj = ItemId(item_id.clone());

            // Get item definition
            let item_def = match registry.get_item(&item_id_obj) {
                Some(def) => def,
                None => return json!({
                    "status": "error",
                    "message": format!("Item not found: {}", item_id)
                }),
            };

            // Verify it's a Simple item
            match &item_def.kind {
                ItemKind::Simple { .. } => {}
                _ => return json!({
                    "status": "error",
                    "message": format!("Can only create instances of Simple items with 'new' command. {} is {:?}", item_id, item_def.kind)
                }),
            }

            let instance_id = registry.next_instance_id();
            let instance = ItemInstance::Simple(SimpleInstance {
                id: instance_id,
                definition: item_id_obj.clone(),
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
        Command::Craft { recipe_id: _, instance_ids: _ } => {
            json!({
                "status": "error",
                "message": "Craft command not yet implemented for new three-tier system"
            })
        }
        Command::Trace(_id) => {
            json!({
                "status": "error",
                "message": "Trace command not yet implemented for new three-tier system"
            })
        }
        Command::Help => {
            json!({
                "status": "success",
                "data": {
                    "commands": [
                        {"command": "list items", "description": "List all item definitions"},
                        {"command": "list recipes", "description": "List all recipes"},
                        {"command": "list instances", "description": "List all item instances"},
                        {"command": "show item <id>", "description": "Show detailed item definition"},
                        {"command": "show recipe <id>", "description": "Show recipe with requirements"},
                        {"command": "show instance <id>", "description": "Show instance with components and provenance"},
                        {"command": "new <item_id> [quality]", "description": "Create raw material instance (defaults to common)"},
                        {"command": "craft <recipe_id> <instance_id>...", "description": "Execute recipe with materials"},
                        {"command": "trace <instance_id>", "description": "Show full provenance tree"},
                        {"command": "help", "description": "Show this help"},
                        {"command": "exit", "description": "Exit REPL"},
                    ]
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
    let provenance = instance.provenance();
    let base_provenance = json!({
        "recipe": provenance.recipe_id.0,
        "consumed": provenance.consumed_inputs.iter().map(|ci| json!({
            "instance_id": ci.instance_id.0,
            "quantity": ci.quantity,
        })).collect::<Vec<_>>(),
        "tool_used": provenance.tool_used.map(|id| id.0),
        "world_object_used": provenance.world_object_used.map(|id| id.0),
        "crafted_at": provenance.crafted_at,
    });

    match instance {
        ItemInstance::Simple(s) => json!({
            "id": s.id.0,
            "type": "Simple",
            "item": s.definition.0,
            "provenance": base_provenance,
        }),
        ItemInstance::Component(c) => json!({
            "id": c.id.0,
            "type": "Component",
            "component_kind": c.component_kind.0,
            "submaterial": c.submaterial.0,
            "provenance": base_provenance,
        }),
        ItemInstance::Composite(c) => json!({
            "id": c.id.0,
            "type": "Composite",
            "item": c.definition.0,
            "quality": format!("{:?}", c.quality),
            "components": c.components.iter().map(|(name, comp)| {
                (name.clone(), json!({
                    "component_kind": comp.component_kind.0,
                    "submaterial": comp.submaterial.0,
                }))
            }).collect::<serde_json::Map<_, _>>(),
            "provenance": base_provenance,
        }),
    }
}

/// Build a recursive provenance tree
/// NOTE: Currently unused, stubbed out pending implementation for new three-tier system
#[allow(dead_code)]
fn build_provenance_tree(instance: &ItemInstance, registry: &Registry) -> Value {
    let mut children = Vec::new();
    let provenance = instance.provenance();

    for consumed in &provenance.consumed_inputs {
        if let Some(child_instance) = registry.get_instance(consumed.instance_id) {
            children.push(build_provenance_tree(child_instance, registry));
        }
    }

    match instance {
        ItemInstance::Simple(s) => json!({
            "instance_id": s.id.0,
            "type": "Simple",
            "item": s.definition.0,
            "recipe": provenance.recipe_id.0,
            "inputs": children
        }),
        ItemInstance::Component(c) => json!({
            "instance_id": c.id.0,
            "type": "Component",
            "component_kind": c.component_kind.0,
            "submaterial": c.submaterial.0,
            "recipe": provenance.recipe_id.0,
            "inputs": children
        }),
        ItemInstance::Composite(comp) => json!({
            "instance_id": comp.id.0,
            "type": "Composite",
            "item": comp.definition.0,
            "quality": format!("{:?}", comp.quality),
            "recipe": provenance.recipe_id.0,
            "inputs": children
        }),
    }
}

/// Run the REPL loop
pub fn run_repl(registry: &mut Registry) -> io::Result<()> {
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
                
                println!("{}", serde_json::to_string(&result).unwrap());
            }
            Err(err) => {
                // Don't print errors for comments
                if err != "Comment" {
                    let error_response = json!({
                        "status": "error",
                        "message": err
                    });
                    println!("{}", serde_json::to_string(&error_response).unwrap());
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
    fn test_parse_show_item() {
        let cmd = parse_command("show item copper_ore").unwrap();
        assert_eq!(cmd, Command::ShowItem("copper_ore".to_string()));
    }

    #[test]
    fn test_parse_show_recipe() {
        let cmd = parse_command("show recipe make_bronze").unwrap();
        assert_eq!(cmd, Command::ShowRecipe("make_bronze".to_string()));
    }

    #[test]
    fn test_parse_show_instance() {
        let cmd = parse_command("show instance 42").unwrap();
        assert_eq!(cmd, Command::ShowInstance(42));
    }

    #[test]
    fn test_parse_new() {
        let cmd = parse_command("new copper_ore common").unwrap();
        assert_eq!(cmd, Command::New {
            item_id: "copper_ore".to_string(),
            quality: Some(Quality::Common),
        });
    }

    #[test]
    fn test_parse_craft() {
        let cmd = parse_command("craft make_bronze 1 2 3").unwrap();
        assert_eq!(cmd, Command::Craft {
            recipe_id: "make_bronze".to_string(),
            instance_ids: vec![1, 2, 3],
        });
    }

    #[test]
    fn test_parse_trace() {
        let cmd = parse_command("trace 5").unwrap();
        assert_eq!(cmd, Command::Trace(5));
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

    #[test]
    fn test_parse_quality() {
        assert_eq!(parse_quality("makeshift").unwrap(), Quality::Makeshift);
        assert_eq!(parse_quality("crude").unwrap(), Quality::Crude);
        assert_eq!(parse_quality("common").unwrap(), Quality::Common);
        assert_eq!(parse_quality("uncommon").unwrap(), Quality::Uncommon);
        assert_eq!(parse_quality("rare").unwrap(), Quality::Rare);
        assert_eq!(parse_quality("epic").unwrap(), Quality::Epic);
        assert_eq!(parse_quality("legendary").unwrap(), Quality::Legendary);
    }

    #[test]
    fn test_parse_invalid_command() {
        assert!(parse_command("invalid").is_err());
    }

    #[test]
    fn test_parse_empty_command() {
        assert!(parse_command("").is_err());
    }

    #[test]
    fn test_parse_incomplete_list() {
        assert!(parse_command("list").is_err());
    }

    #[test]
    fn test_parse_incomplete_show() {
        assert!(parse_command("show item").is_err());
    }

    #[test]
    fn test_parse_new_with_default_quality() {
        // Without explicit quality, parse returns None (will inherit from ItemDefinition)
        let cmd = parse_command("new copper_ore").unwrap();
        assert_eq!(cmd, Command::New {
            item_id: "copper_ore".to_string(),
            quality: None,
        });
    }

    #[test]
    fn test_parse_incomplete_new() {
        // Now requires at least item_id
        assert!(parse_command("new").is_err());
    }

    #[test]
    fn test_parse_invalid_instance_id() {
        assert!(parse_command("show instance abc").is_err());
    }
}
