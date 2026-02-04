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
                // Find required quantity of matching items in inventory
                let mut found_count = 0u32;
                let mut indices_to_remove = Vec::new();
                
                for (i, inv_item_id) in state.character.inventory.items.iter().enumerate() {
                    if found_count >= input.quantity {
                        break;
                    }
                    
                    if let Some(instance) = state.crafting_registry.get_instance(*inv_item_id) {
                        if let crate::game::crafting::ItemInstance::Simple(s) = instance {
                            if s.definition == input.item_id {
                                provided_inputs.push(*inv_item_id);
                                indices_to_remove.push(i);
                                found_count += 1;
                            }
                        }
                    }
                }
                
                if found_count < input.quantity {
                    // Put back items we already collected
                    for item_id in provided_inputs.iter() {
                        state.character.inventory.add_item(*item_id);
                    }
                    return (false, format!("Missing required item: {} (need {}, have {})", 
                        input.item_id.0, input.quantity, found_count));
                }
                
                // Remove collected items from inventory (in reverse order to maintain indices)
                for &i in indices_to_remove.iter().rev() {
                    state.character.inventory.remove_item(i);
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
    
    if command.starts_with("move ") || command.starts_with("m ") {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 {
            return (false, "Usage: m <direction> or move <direction> (e.g., 'm u' or 'move up'). Directions: u/up, d/down, l/left, r/right".to_string());
        }
        
        let direction = parts[1].to_lowercase();
        let (dx, dy, emoji) = match direction.as_str() {
            "u" | "up" => (0, -1, "⬆️"),
            "d" | "down" => (0, 1, "⬇️"),
            "l" | "left" => (-1, 0, "⬅️"),
            "r" | "right" => (1, 0, "➡️"),
            _ => return (false, "Invalid direction. Use u/up, d/down, l/left, or r/right".to_string()),
        };
        
        return match state.current_mode {
            CurrentMode::Terrain => {
                state.move_terrain(dx, dy);
                let (x, y) = state.current_land();
                (true, format!("{} L[{},{}]", emoji, x, y))
            }
            CurrentMode::Combat => {
                (false, "Cannot move during combat. Use 'a' to attack or 'x' to flee.".to_string())
            }
            CurrentMode::Land => {
                state.move_land(dx, dy);
                if let Some((x, y)) = state.current_tile() {
                    (true, format!("{} T[{},{}]", emoji, x, y))
                } else {
                    (false, "Not in land view".to_string())
                }
            }
        };
    }
    
    if command.starts_with("place ") || command.starts_with("l ") {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 {
            return (false, "Usage: l <inventory_index> or place <inventory_index> (e.g., 'l 0' to place first item)".to_string());
        }
        
        // Can only place in land view
        if state.current_mode != CurrentMode::Land {
            return (false, "Can only place items in land view".to_string());
        }
        
        let index: usize = match parts[1].parse() {
            Ok(i) => i,
            Err(_) => return (false, "Invalid index. Use a number (e.g., 'place 0')".to_string()),
        };
        
        // Get item from inventory
        let item_instance_id = match state.character.inventory.items.get(index) {
            Some(&id) => id,
            None => return (false, format!("No item at index {}. Use 'inv' to see your inventory.", index)),
        };
        
        // Get item definition
        let item_instance = match state.crafting_registry.get_instance(item_instance_id) {
            Some(inst) => inst,
            None => return (false, "Item instance not found in registry".to_string()),
        };
        
        let item_def_id = match item_instance {
            crate::game::crafting::ItemInstance::Simple(s) => &s.definition,
            _ => return (false, "Can only place simple items".to_string()),
        };
        
        let item_def = match state.crafting_registry.get_item(item_def_id) {
            Some(def) => def,
            None => return (false, "Item definition not found".to_string()),
        };
        
        // Check if item is placeable and clone what we need before mutable borrows
        let world_object_kind = match &item_def.placeable {
            Some(kind) => kind.clone(),
            None => return (false, format!("{} cannot be placed", item_def.name)),
        };
        let item_name = item_def.name.clone();
        
        // Get current tile position
        let (land_x, land_y) = state.current_land();
        let (tile_x, tile_y) = match state.current_tile() {
            Some(pos) => pos,
            None => return (false, "Not in land view".to_string()),
        };
        
        // Create world object instance
        let world_object_id = state.crafting_registry.next_world_object_id();
        let world_object = crate::game::crafting::WorldObjectInstance {
            id: world_object_id,
            kind: world_object_kind,
            tags: vec![], // TODO: Add tags based on item type if needed
        };
        state.crafting_registry.register_world_object(world_object);
        
        // Add to tile
        if let Some(land) = state.world.terrain.get_mut(&(land_x, land_y)) {
            let tile = &mut land.tiles[tile_y][tile_x];
            
            // Check if tile already has a world object
            if tile.world_object.is_some() {
                return (false, "This tile already has a world object. Choose a different location.".to_string());
            }
            
            tile.world_object = Some(world_object_id);
            
            // Remove from inventory
            state.character.inventory.remove_item(index);
            
            return (true, format!("🏗️ Placed {} at tile [{},{}]", item_name, tile_x, tile_y));
        } else {
            return (false, "Land not found".to_string());
        }
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
        "m" => {
            // Standalone m - show usage
            (false, "Usage: m <direction> (e.g., 'm u' for up). Directions: u/up, d/down, l/left, r/right".to_string())
        }
        "up" | "down" | "right" => {
            // Legacy single-letter commands - redirect to move command
            (false, format!("Movement now requires 'm' prefix. Use 'm {}' instead. Type 'help' for more info.", command))
        }
        "left" => {
            // Legacy left command - redirect to move command
            (false, "Movement now requires 'm' prefix. Use 'm l' instead. Type 'help' for more info.".to_string())
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
        "l" | "place" => {
            // Alias for place command - must provide an index
            (false, "Usage: l <inventory_index> or place <inventory_index> (e.g., 'l 0' to place first item)".to_string())
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
                
                if tile.items.is_empty() {
                    return (false, "No items here to pick up".to_string());
                }
                
                // Get first item (without removing yet)
                let item_id = tile.items[0];
                
                // Check if item is pickupable
                let (item_name, is_pickupable) = state.crafting_registry.get_instance(item_id)
                    .and_then(|instance| {
                        match instance {
                            crate::game::crafting::ItemInstance::Simple(s) => {
                                state.crafting_registry.get_item(&s.definition)
                                    .map(|def| (def.name.clone(), def.pickupable))
                            }
                            _ => None
                        }
                    })
                    .unwrap_or_else(|| ("Unknown Item".to_string(), false));
                
                if !is_pickupable {
                    return (false, format!("{} cannot be picked up. You may need to use a tool to harvest it.", item_name));
                }
                
                // Remove from tile and add to inventory
                tile.items.remove(0);
                state.character.inventory.add_item(item_id);
                
                (true, format!("📦 Picked up {}", item_name))
            } else {
                (false, "Land not found".to_string())
            }
        }
        "drop" | "d" => {
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
                land.tiles[tile_y][tile_x].items.push(item_id);
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
        "use" | "u" => {
            // Can only use in land view
            if state.current_mode != CurrentMode::Land {
                return (false, "Can only use tools in land view".to_string());
            }
            
            // Check if player has equipped tool
            let equipped_id = match state.character.get_equipped() {
                Some(id) => id,
                None => return (false, "No tool equipped. Equip a tool first.".to_string()),
            };
            
            // Get tool type from equipped item
            let tool_type = state.crafting_registry.get_instance(equipped_id)
                .and_then(|instance| {
                    let item_def = match instance {
                        crate::game::crafting::ItemInstance::Simple(s) => {
                            state.crafting_registry.get_item(&s.definition)
                        }
                        crate::game::crafting::ItemInstance::Composite(c) => {
                            state.crafting_registry.get_item(&c.definition)
                        }
                        _ => None
                    };
                    
                    item_def.and_then(|def| {
                        if let crate::game::crafting::ItemKind::Composite(comp_def) = &def.kind {
                            comp_def.tool_type.clone()
                        } else {
                            // Check for makeshift tools
                            if def.id.0 == "rock" {
                                Some(crate::game::crafting::ToolType::Hammer)
                            } else if def.id.0 == "stick" {
                                Some(crate::game::crafting::ToolType::Shovel)
                            } else {
                                None
                            }
                        }
                    })
                });
            
            let tool_type = match tool_type {
                Some(t) => t,
                None => return (false, "Equipped item is not a usable tool".to_string()),
            };
            
            let (land_x, land_y) = state.current_land();
            let (tile_x, tile_y) = match state.current_tile() {
                Some(pos) => pos,
                None => return (false, "Not in land view".to_string()),
            };
            
            // Get the tile
            if let Some(land) = state.world.terrain.get_mut(&(land_x, land_y)) {
                let tile = &mut land.tiles[tile_y][tile_x];
                
                // Priority 1: Check for world object at tile
                if let Some(_world_object_id) = tile.world_object {
                    // Try to find a recipe that uses this world object + equipped tool
                    // For now, check if there's a tree and we have an axe
                    if !tile.items.is_empty() {
                        let first_item_id = tile.items[0];
                        if let Some(instance) = state.crafting_registry.get_instance(first_item_id) {
                            if let crate::game::crafting::ItemInstance::Simple(s) = instance {
                                if let Some(item_def) = state.crafting_registry.get_item(&s.definition) {
                                    // Check if this is a tree and we have an axe
                                    if item_def.id.0 == "tree" && tool_type == crate::game::crafting::ToolType::Axe {
                                        // Try to craft via chop_tree recipe
                                        return execute_command(state, "craft chop_tree");
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Priority 2: Check substrate interaction
                let substrate = &tile.substrate;
                match (&tool_type, substrate) {
                    (crate::game::crafting::ToolType::Shovel, crate::game::world::types::Substrate::Clay) => {
                        // Harvest clay from clay substrate
                        let clay_instance_id = state.crafting_registry.next_instance_id();
                        let clay_instance = crate::game::crafting::ItemInstance::Simple(
                            crate::game::crafting::SimpleInstance {
                                id: clay_instance_id,
                                definition: crate::game::crafting::ItemId("clay".to_string()),
                                provenance: crate::game::crafting::Provenance {
                                    recipe_id: crate::game::crafting::RecipeId("harvest_clay".to_string()),
                                    consumed_inputs: vec![],
                                    tool_used: Some(equipped_id),
                                    world_object_used: None,
                                    crafted_at: state.combat_round as i64,
                                },
                            }
                        );
                        state.crafting_registry.register_instance(clay_instance);
                        state.character.inventory.add_item(clay_instance_id);
                        
                        return (true, "⛏️ Harvested Clay from clay substrate".to_string());
                    }
                    _ => {
                        return (false, format!("Cannot use {:?} on {:?} substrate or current tile contents", tool_type, substrate));
                    }
                }
            } else {
                (false, "Land not found".to_string())
            }
        }
        "recipes" | "recipe" | "r" => {
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
        "craftable" | "can" | "available" => {
            let mut output = String::from("Craftable Recipes:\n");
            let mut found_any = false;
            
            // Get all world objects in current land
            let world_objects_in_land = state.get_world_objects_in_current_land();
            
            // Get inventory items
            let inventory = &state.character.inventory.items;
            
            // Check simple recipes
            output.push_str("\n=== Simple Recipes ===\n");
            for recipe in state.crafting_registry.all_simple_recipes() {
                // Check if we have all required inputs
                let mut can_craft = true;
                let mut missing = Vec::new();
                
                for required_input in &recipe.inputs {
                    let mut found_count = 0;
                    for &inv_item_id in inventory.iter() {
                        if let Some(instance) = state.crafting_registry.get_instance(inv_item_id) {
                            if let crate::game::crafting::ItemInstance::Simple(s) = instance {
                                if let Some(def) = state.crafting_registry.get_item(&s.definition) {
                                    if def.id == required_input.item_id {
                                        found_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    if found_count < required_input.quantity {
                        can_craft = false;
                        missing.push(format!("{} (need {})", required_input.item_id.0, required_input.quantity));
                    }
                }
                
                // Check tool requirement
                if let Some(ref tool_req) = recipe.tool {
                    let has_tool = inventory.iter().any(|&inv_item_id| {
                        if let Some(instance) = state.crafting_registry.get_instance(inv_item_id) {
                            // Check if this item can act as the required tool type
                            let item_def = match instance {
                                crate::game::crafting::ItemInstance::Simple(s) => {
                                    state.crafting_registry.get_item(&s.definition)
                                }
                                crate::game::crafting::ItemInstance::Composite(c) => {
                                    state.crafting_registry.get_item(&c.definition)
                                }
                                _ => None
                            };
                            
                            if let Some(def) = item_def {
                                if let crate::game::crafting::ItemKind::Composite(comp_def) = &def.kind {
                                    if let Some(ref item_tool_type) = comp_def.tool_type {
                                        return item_tool_type == &tool_req.tool_type;
                                    }
                                }
                                // Check makeshift tools (rock = hammer, stick = shovel, etc.)
                                if def.id.0 == "rock" && tool_req.tool_type == crate::game::crafting::ToolType::Hammer {
                                    return true;
                                }
                                if def.id.0 == "stick" && tool_req.tool_type == crate::game::crafting::ToolType::Shovel {
                                    return true;
                                }
                            }
                            false
                        } else {
                            false
                        }
                    });
                    
                    if !has_tool {
                        can_craft = false;
                        missing.push(format!("Tool: {:?}", tool_req.tool_type));
                    }
                }
                
                // Check world object requirement
                if let Some(ref wo_req) = recipe.world_object {
                    let has_workstation = world_objects_in_land.iter().any(|&wo_id| {
                        state.crafting_registry.validate_world_object_requirement(wo_id, wo_req).is_ok()
                    });
                    
                    if !has_workstation {
                        can_craft = false;
                        if let Some(ref kind) = wo_req.kind {
                            missing.push(format!("Workstation: {:?}", kind));
                        } else {
                            missing.push(format!("Workstation with tags: {:?}", wo_req.required_tags));
                        }
                    }
                }
                
                if can_craft {
                    output.push_str(&format!("  ✓ {} - {}\n", recipe.id.0, recipe.name));
                    found_any = true;
                }
            }
            
            if !found_any {
                output.push_str("  (none)\n");
            }
            
            // Note: Component and Composite recipes not yet supported for craftable query
            output.push_str("\n(Component and Composite recipes not yet included in craftable query)\n");
            
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
  
  Note: Cannot move during combat
"#
                }
                CurrentMode::Land => {
                    r#"
Commands:
  M <dir>, MOVE   - Move (e.g., 'm u' or 'move up'). Directions: u/d/l/r
  X, EXIT         - Exit land view
  PICKUP, P, GET  - Pick up item from current tile
  D, DROP         - Drop first item from inventory
  U, USE          - Use equipped tool on world object or substrate
  L, PLACE <idx>  - Place item as world object (e.g., 'l 0' to place forge)
  E, EQUIP <idx>  - Equip item from inventory (e.g., 'e 0')
  UNEQUIP         - Unequip current item to inventory
  CRAFT <recipe>  - Craft item from recipe (e.g., 'craft knap_flint_blade')
  C <recipe>      - Shortcut for craft
  R, RECIPES      - List all recipes
  CRAFTABLE       - Show craftable recipes based on inventory + workstations
  STATUS, STATS   - Show character status
  INV, INVENTORY  - Show inventory
  H, HELP, ?      - Show this help
"#
                }
                _ => {
                    r#"
Commands:
  M <dir>, MOVE   - Move (e.g., 'm u' or 'move up'). Directions: u/d/l/r
  X, ENTER        - Enter land view (may trigger combat if enemy present)
  E, EQUIP <idx>  - Equip item from inventory (e.g., 'e 0')
  UNEQUIP         - Unequip current item
  CRAFT <recipe>  - Craft item from recipe
  C <recipe>      - Shortcut for craft
  R, RECIPES      - List all recipes
  CRAFTABLE       - Show craftable recipes based on inventory + workstations
  STATUS, STATS   - Show character status
  INV, INVENTORY  - Show inventory
  H, HELP, ?      - Show this help
  
  (Enter land view to pickup/drop/use/craft items)
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
                            "Health: {}/{} | Attack: {} | Defense: {} | Accuracy: {} | Evasion: {} | Land: [{},{}] | Tile: [{},{}] | Equipped: {} | Mode: Land View",
                            state.character.get_health(),
                            state.character.get_max_health(),
                            state.get_total_attack(),
                            state.get_total_defense(),
                            state.get_total_accuracy(),
                            state.get_total_evasion(),
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
                "Health: {}/{} | Attack: {} | Defense: {} | Accuracy: {} | Evasion: {} | Land: [{},{}] | Equipped: {} | Mode: {}",
                state.character.get_health(),
                state.character.get_max_health(),
                state.get_total_attack(),
                state.get_total_defense(),
                state.get_total_accuracy(),
                state.get_total_evasion(),
                land_x, land_y,
                equipped_str,
                mode_str
            ))
        }
        "" => (false, "Empty command".to_string()),
        _ => (false, format!("Unknown command: {}. Type 'help' for commands.", command)),
    }
}
