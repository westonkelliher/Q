use super::game_state::{GameState, CurrentMode};
use super::combat::CombatResult;

/// Execute a command and return (success, message)
pub fn execute_command(state: &mut GameState, command: &str) -> (bool, String) {
    // Handle commands with arguments first
    if command.starts_with("craft ") || command.starts_with("c ") {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 {
            return (false, "Usage: craft <recipe_id> (e.g., 'craft knap_flint_blade')".to_string());
        }
        
        let recipe_id_str = parts[1];
        let recipe_id = crate::game::crafting::RecipeId(recipe_id_str.to_string());
        
        // Try to find and execute the recipe
        // First check simple recipes
        if let Some(recipe) = state.crafting_registry.get_simple_recipe(&recipe_id).cloned() {
            // Collect matching items from inventory
            let mut provided_inputs = Vec::new();
            for input in &recipe.inputs {
                // Find matching item in inventory
                let mut found = false;
                for (i, inv_item_id) in state.character.inventory.items.iter().enumerate() {
                    if let Some(instance) = state.crafting_registry.get_instance(*inv_item_id) {
                        if let crate::game::crafting::ItemInstance::Simple(s) = instance {
                            if s.definition == input.item_id {
                                provided_inputs.push(*inv_item_id);
                                // Remove from inventory
                                state.character.inventory.remove_item(i);
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if !found {
                    // Put back items we already removed
                    for item_id in provided_inputs.iter() {
                        state.character.inventory.add_item(*item_id);
                    }
                    return (false, format!("Missing required item: {}", input.item_id.0));
                }
            }
            
            let tool_used = state.character.get_equipped();
            
            // Execute recipe
            match state.crafting_registry.execute_simple_recipe(&recipe, provided_inputs, tool_used, None) {
                Ok(result) => {
                    let item_name = match &result {
                        crate::game::crafting::ItemInstance::Simple(s) => {
                            state.crafting_registry.get_item(&s.definition)
                                .map(|def| def.name.clone())
                                .unwrap_or_else(|| "Unknown".to_string())
                        }
                        _ => "Item".to_string(),
                    };
                    
                    let result_id = result.id();
                    state.crafting_registry.register_instance(result);
                    state.character.inventory.add_item(result_id);
                    
                    return (true, format!("🔨 Crafted {}", item_name));
                }
                Err(e) => {
                    // Return consumed items on error
                    // (They were already consumed by execute_simple_recipe)
                    return (false, format!("Crafting failed: {}", e));
                }
            }
        }
        
        // Check component recipes
        if let Some(_recipe) = state.crafting_registry.get_component_recipe(&recipe_id) {
            return (false, "Component recipes not yet supported in REPL".to_string());
        }
        
        // Check composite recipes
        if let Some(_recipe) = state.crafting_registry.get_composite_recipe(&recipe_id) {
            return (false, "Composite recipes not yet supported in REPL".to_string());
        }
        
        return (false, format!("Recipe not found: {}", recipe_id_str));
    }
    
    if command.starts_with("equip ") || command.starts_with("e ") {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 {
            return (false, "Usage: equip <inventory_index> (e.g., 'equip 0' to equip first item)".to_string());
        }
        
        let index: usize = match parts[1].parse() {
            Ok(i) => i,
            Err(_) => return (false, "Invalid index. Use a number (e.g., 'equip 0')".to_string()),
        };
        
        // Get item name before equipping
        let item_name = state.character.get_inventory().items.get(index)
            .and_then(|instance_id| {
                state.crafting_registry.get_instance(*instance_id)
                    .and_then(|instance| {
                        match instance {
                            crate::game::crafting::ItemInstance::Simple(s) => {
                                state.crafting_registry.get_item(&s.definition)
                                    .map(|def| def.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Component(c) => {
                                state.crafting_registry.get_component_kind(&c.component_kind)
                                    .map(|ck| ck.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Composite(c) => {
                                state.crafting_registry.get_item(&c.definition)
                                    .map(|def| def.name.clone())
                            }
                        }
                    })
            })
            .unwrap_or_else(|| "Unknown Item".to_string());
        
        return match state.character.equip_from_inventory(index) {
            Ok(_) => (true, format!("⚔️ Equipped {}", item_name)),
            Err(e) => (false, e),
        };
    }
    
    match command {
        "u" | "up" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    state.move_terrain(0, -1);
                    let (x, y) = state.current_land();
                    (true, format!("⬆️ L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'x' to flee.".to_string())
                }
                CurrentMode::Land => {
                    state.move_land(0, -1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬆️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "d" | "down" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    state.move_terrain(0, 1);
                    let (x, y) = state.current_land();
                    (true, format!("⬇️ L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'x' to flee.".to_string())
                }
                CurrentMode::Land => {
                    state.move_land(0, 1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬇️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "l" | "left" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    state.move_terrain(-1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("⬅️ L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'x' to flee.".to_string())
                }
                CurrentMode::Land => {
                    state.move_land(-1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬅️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "r" | "right" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    state.move_terrain(1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("➡️ L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'x' to flee.".to_string())
                }
                CurrentMode::Land => {
                    state.move_land(1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("➡️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "enter" | "exit" | "x" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    let (land_x, land_y) = state.current_land();
                    state.enter_land();
                    
                    if state.current_mode == CurrentMode::Combat {
                        (true, "⚔️ Combat!".to_string())
                    } else {
                        (true, format!("🔽 Enter L[{},{}]", land_x, land_y))
                    }
                }
                CurrentMode::Land => {
                    let (x, y) = state.current_land();
                    state.exit_land();
                    (true, format!("🔼 Exit L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    state.combat_flee();
                    (true, "🏃 Flee!".to_string())
                }
            }
        }
        "e" => {
            // Alias for equip command - must provide an index
            (false, "Usage: e <inventory_index> or equip <inventory_index> (e.g., 'e 0' to equip first item)".to_string())
        }
        "c" => {
            // Alias for craft command - must provide a recipe
            (false, "Usage: c <recipe_id> or craft <recipe_id> (e.g., 'c knap_flint_blade'). Type 'recipes' to see available recipes.".to_string())
        }
        "attack" | "a" => {
            if state.current_mode == CurrentMode::Combat {
                let result = state.combat_attack();
                match result {
                    CombatResult::Ongoing => {
                        let (land_x, land_y) = state.current_land();
                        let enemy = state.world.terrain.get(&(land_x, land_y))
                            .and_then(|land| land.enemy.as_ref())
                            .unwrap();
                        (true, format!("⚔️ Attack! P:{}/{} E:{}/{}", 
                            state.character.get_health(),
                            state.character.get_max_health(),
                            enemy.health,
                            enemy.max_health))
                    }
                    CombatResult::PlayerWins => {
                        (true, "⚔️ Victory!".to_string())
                    }
                    CombatResult::EnemyWins | CombatResult::Draw => {
                        (true, "⚔️ Defeated!".to_string())
                    }
                }
            } else {
                (false, "Not in combat. Use 'X' to enter a land with enemies.".to_string())
            }
        }
        "flee" | "f" => {
            // 'X' is now the primary command for flee (and enter/exit)
            // Keep this for backward compatibility
            if state.current_mode == CurrentMode::Combat {
                state.combat_flee();
                (true, "🏃 Flee!".to_string())
            } else {
                (false, "Use 'X' to flee combat (or enter/exit based on context)".to_string())
            }
        }
        "pickup" | "p" | "take" | "get" => {
            // Can only pickup in land view
            if state.current_mode != CurrentMode::Land {
                return (false, "Can only pickup items in land view".to_string());
            }
            
            let (land_x, land_y) = state.current_land();
            let (tile_x, tile_y) = match state.current_tile() {
                Some(pos) => pos,
                None => return (false, "Not in land view".to_string()),
            };
            
            // Get the tile
            if let Some(land) = state.world.terrain.get_mut(&(land_x, land_y)) {
                let tile = &mut land.tiles[tile_y][tile_x];
                
                if tile.objects.is_empty() {
                    return (false, "No items here to pick up".to_string());
                }
                
                // Pick up first item
                let item_id = tile.objects.remove(0);
                
                // Get item name for display
                let item_name = state.crafting_registry.get_instance(item_id)
                    .and_then(|instance| {
                        match instance {
                            crate::game::crafting::ItemInstance::Simple(s) => {
                                state.crafting_registry.get_item(&s.definition).map(|def| def.name.clone())
                            }
                            _ => None
                        }
                    })
                    .unwrap_or_else(|| "Unknown Item".to_string());
                
                state.character.inventory.add_item(item_id);
                
                (true, format!("📦 Picked up {}", item_name))
            } else {
                (false, "Land not found".to_string())
            }
        }
        "drop" => {
            // Can only drop in land view
            if state.current_mode != CurrentMode::Land {
                return (false, "Can only drop items in land view".to_string());
            }
            
            if state.character.inventory.is_empty() {
                return (false, "Inventory is empty".to_string());
            }
            
            let (land_x, land_y) = state.current_land();
            let (tile_x, tile_y) = match state.current_tile() {
                Some(pos) => pos,
                None => return (false, "Not in land view".to_string()),
            };
            
            // Remove first item from inventory
            let item_id = match state.character.inventory.remove_item(0) {
                Some(id) => id,
                None => return (false, "Failed to remove item from inventory".to_string()),
            };
            
            // Get item name for display
            let item_name = state.crafting_registry.get_instance(item_id)
                .and_then(|instance| {
                    match instance {
                        crate::game::crafting::ItemInstance::Simple(s) => {
                            state.crafting_registry.get_item(&s.definition).map(|def| def.name.clone())
                        }
                        _ => None
                    }
                })
                .unwrap_or_else(|| "Unknown Item".to_string());
            
            // Add to tile
            if let Some(land) = state.world.terrain.get_mut(&(land_x, land_y)) {
                land.tiles[tile_y][tile_x].objects.push(item_id);
                (true, format!("📤 Dropped {}", item_name))
            } else {
                // Return item to inventory if land not found (shouldn't happen)
                state.character.inventory.add_item(item_id);
                (false, "Land not found".to_string())
            }
        }
        "unequip" => {
            match state.character.unequip() {
                Some(item_id) => {
                    // Get item name for display
                    let item_name = state.crafting_registry.get_instance(item_id)
                        .and_then(|instance| {
                            match instance {
                                crate::game::crafting::ItemInstance::Simple(s) => {
                                    state.crafting_registry.get_item(&s.definition)
                                        .map(|def| def.name.clone())
                                }
                                crate::game::crafting::ItemInstance::Component(c) => {
                                    state.crafting_registry.get_component_kind(&c.component_kind)
                                        .map(|ck| ck.name.clone())
                                }
                                crate::game::crafting::ItemInstance::Composite(c) => {
                                    state.crafting_registry.get_item(&c.definition)
                                        .map(|def| def.name.clone())
                                }
                            }
                        })
                        .unwrap_or_else(|| "Unknown Item".to_string());
                    
                    (true, format!("📤 Unequipped {}", item_name))
                }
                None => (false, "No item equipped".to_string()),
            }
        }
        "recipes" | "recipe" => {
            let mut output = String::from("Available Recipes:\n");
            
            // List simple recipes
            output.push_str("\n=== Simple Recipes ===\n");
            for recipe in state.crafting_registry.all_simple_recipes() {
                output.push_str(&format!("  {} - {}\n", recipe.id.0, recipe.name));
            }
            
            // List component recipes
            output.push_str("\n=== Component Recipes ===\n");
            for recipe in state.crafting_registry.all_component_recipes() {
                output.push_str(&format!("  {} - {}\n", recipe.id.0, recipe.name));
            }
            
            // List composite recipes
            output.push_str("\n=== Composite Recipes ===\n");
            for recipe in state.crafting_registry.all_composite_recipes() {
                output.push_str(&format!("  {} - {}\n", recipe.id.0, recipe.name));
            }
            
            (true, output)
        }
        "help" | "h" | "?" => {
            let help_text = match state.current_mode {
                CurrentMode::Combat => {
                    r#"
Combat Commands:
  A, ATTACK       - Attack the enemy
  X, EXIT         - Flee combat (returns to terrain view)
  E, EQUIP <idx>  - Equip item from inventory (e.g., 'e 0')
  UNEQUIP         - Unequip current item
  STATUS, STATS   - Show character status
  INV, INVENTORY  - Show inventory
  H, HELP, ?      - Show this help
"#
                }
                CurrentMode::Land => {
                    r#"
Commands:
  Arrow Keys      - Move around
  X, EXIT         - Exit land view
  PICKUP, P, GET  - Pick up item from current tile
  DROP            - Drop first item from inventory
  E, EQUIP <idx>  - Equip item from inventory (e.g., 'e 0')
  UNEQUIP         - Unequip current item to inventory
  CRAFT <recipe>  - Craft item from recipe (e.g., 'craft knap_flint_blade')
  C <recipe>      - Shortcut for craft
  RECIPES         - List available recipes
  STATUS, STATS   - Show character status
  INV, INVENTORY  - Show inventory
  H, HELP, ?      - Show this help
"#
                }
                _ => {
                    r#"
Commands:
  Arrow Keys      - Move around
  X, ENTER        - Enter land view (may trigger combat if enemy present)
  E, EQUIP <idx>  - Equip item from inventory (e.g., 'e 0')
  UNEQUIP         - Unequip current item
  CRAFT <recipe>  - Craft item from recipe
  C <recipe>      - Shortcut for craft
  RECIPES         - List available recipes
  STATUS, STATS   - Show character status
  INV, INVENTORY  - Show inventory
  H, HELP, ?      - Show this help
  
  (Enter land view to pickup/drop/craft items)
"#
                }
            };
            (true, help_text.trim().to_string())
        }
        "inventory" | "inv" | "i" => {
            let mut output = String::new();
            
            // Show equipped item
            if let Some(equipped_id) = state.character.get_equipped() {
                let equipped_name = state.crafting_registry.get_instance(equipped_id)
                    .and_then(|instance| {
                        match instance {
                            crate::game::crafting::ItemInstance::Simple(s) => {
                                state.crafting_registry.get_item(&s.definition)
                                    .map(|def| def.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Component(c) => {
                                state.crafting_registry.get_component_kind(&c.component_kind)
                                    .map(|ck| ck.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Composite(c) => {
                                state.crafting_registry.get_item(&c.definition)
                                    .map(|def| def.name.clone())
                            }
                        }
                    })
                    .unwrap_or_else(|| "Unknown Item".to_string());
                output.push_str(&format!("Equipped: {}\n", equipped_name));
            } else {
                output.push_str("Equipped: (none)\n");
            }
            
            // Show inventory
            let inv = state.character.get_inventory();
            if inv.items.is_empty() {
                output.push_str("Inventory: (empty)");
            } else {
                let item_names: Vec<String> = inv.items.iter().enumerate().filter_map(|(i, instance_id)| {
                    state.crafting_registry.get_instance(*instance_id)
                        .and_then(|instance| {
                            match instance {
                                crate::game::crafting::ItemInstance::Simple(s) => {
                                    state.crafting_registry.get_item(&s.definition)
                                        .map(|def| format!("[{}] {}", i, def.name.clone()))
                                }
                                crate::game::crafting::ItemInstance::Component(c) => {
                                    state.crafting_registry.get_component_kind(&c.component_kind)
                                        .map(|ck| format!("[{}] {}", i, ck.name.clone()))
                                }
                                crate::game::crafting::ItemInstance::Composite(c) => {
                                    state.crafting_registry.get_item(&c.definition)
                                        .map(|def| format!("[{}] {}", i, def.name.clone()))
                                }
                            }
                        })
                }).collect();
                output.push_str(&format!("Inventory:\n{}", item_names.join("\n")));
            }
            
            (true, output)
        }
        "status" | "stats" | "s" => {
            let (land_x, land_y) = state.current_land();
            
            // Get equipped item name
            let equipped_str = if let Some(equipped_id) = state.character.get_equipped() {
                state.crafting_registry.get_instance(equipped_id)
                    .and_then(|instance| {
                        match instance {
                            crate::game::crafting::ItemInstance::Simple(s) => {
                                state.crafting_registry.get_item(&s.definition)
                                    .map(|def| def.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Component(c) => {
                                state.crafting_registry.get_component_kind(&c.component_kind)
                                    .map(|ck| ck.name.clone())
                            }
                            crate::game::crafting::ItemInstance::Composite(c) => {
                                state.crafting_registry.get_item(&c.definition)
                                    .map(|def| def.name.clone())
                            }
                        }
                    })
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                "(none)".to_string()
            };
            
            let mode_str = match state.current_mode {
                CurrentMode::Terrain => "Terrain View",
                CurrentMode::Land => {
                    if let Some((tile_x, tile_y)) = state.current_tile() {
                        return (true, format!(
                            "Health: {}/{} | Attack: {} | Land: [{},{}] | Tile: [{},{}] | Equipped: {} | Mode: Land View",
                            state.character.get_health(),
                            state.character.get_max_health(),
                            state.character.get_attack(),
                            land_x, land_y,
                            tile_x, tile_y,
                            equipped_str
                        ));
                    }
                    "Land View"
                }
                CurrentMode::Combat => "Combat",
            };
            (true, format!(
                "Health: {}/{} | Attack: {} | Land: [{},{}] | Equipped: {} | Mode: {}",
                state.character.get_health(),
                state.character.get_max_health(),
                state.character.get_attack(),
                land_x, land_y,
                equipped_str,
                mode_str
            ))
        }
        "" => (false, "Empty command".to_string()),
        _ => (false, format!("Unknown command: {}. Type 'help' for commands.", command)),
    }
}
