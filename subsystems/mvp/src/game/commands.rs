use super::game_state::{GameState, CurrentMode};
use super::combat::CombatResult;

/// Execute a command and return (success, message)
pub fn execute_command(state: &mut GameState, command: &str) -> (bool, String) {
    match command {
        "u" | "up" => {
            match state.current_mode {
                CurrentMode::Terrain => {
                    state.move_terrain(0, -1);
                    let (x, y) = state.current_land();
                    (true, format!("⬆️ L[{},{}]", x, y))
                }
                CurrentMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'e' to flee.".to_string())
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
                    (false, "Cannot move during combat. Use 'a' to attack or 'e' to flee.".to_string())
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
                    (false, "Cannot move during combat. Use 'a' to attack or 'e' to flee.".to_string())
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
                    (false, "Cannot move during combat. Use 'a' to attack or 'e' to flee.".to_string())
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
        "enter" | "e" => {
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
        "exit" | "x" => {
            // 'E' is now the primary command for exit (and enter/flee)
            // Keep this for backward compatibility
            if state.current_mode == CurrentMode::Land {
                let (x, y) = state.current_land();
                state.exit_land();
                (true, format!("🔼 Exit L[{},{}]", x, y))
            } else {
                (false, "Use 'E' to exit land view (or enter/flee based on context)".to_string())
            }
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
                (false, "Not in combat. Use 'E' to enter a land with enemies.".to_string())
            }
        }
        "flee" | "f" => {
            // 'E' is now the primary command for flee (and enter/exit)
            // Keep this for backward compatibility
            if state.current_mode == CurrentMode::Combat {
                state.combat_flee();
                (true, "🏃 Flee!".to_string())
            } else {
                (false, "Use 'E' to flee combat (or enter/exit based on context)".to_string())
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
  A, ATTACK     - Attack the enemy
  E, ENTER      - Flee combat (returns to terrain view)
  STATUS, STATS - Show character status
  INV, I        - Show inventory
  H, HELP, ?    - Show this help
"#
                }
                CurrentMode::Land => {
                    r#"
Commands:
  U, D, L, R     - Move up, down, left, right
  E, ENTER       - Exit land view
  PICKUP, P, GET - Pick up item from current tile
  DROP           - Drop first item from inventory
  RECIPES        - List available recipes
  STATUS, STATS  - Show character status
  INV, I         - Show inventory
  H, HELP, ?     - Show this help
"#
                }
                _ => {
                    r#"
Commands:
  U, D, L, R     - Move up, down, left, right
  E, ENTER       - Enter land view (may trigger combat if enemy present)
  RECIPES        - List available recipes
  STATUS, STATS  - Show character status
  INV, I         - Show inventory
  H, HELP, ?     - Show this help
  
  (Enter land view to pickup/drop items)
"#
                }
            };
            (true, help_text.trim().to_string())
        }
        "inventory" | "inv" | "i" => {
            let inv = state.character.get_inventory();
            if inv.items.is_empty() {
                (true, "Inventory is empty".to_string())
            } else {
                let item_names: Vec<String> = inv.items.iter().filter_map(|instance_id| {
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
                }).collect();
                (true, format!("Inventory: {}", item_names.join(", ")))
            }
        }
        "status" | "stats" | "s" => {
            let (land_x, land_y) = state.current_land();
            let mode_str = match state.current_mode {
                CurrentMode::Terrain => "Terrain View",
                CurrentMode::Land => {
                    if let Some((tile_x, tile_y)) = state.current_tile() {
                        return (true, format!(
                            "Health: {}/{} | Attack: {} | Land: [{},{}] | Tile: [{},{}] | Mode: Land View",
                            state.character.get_health(),
                            state.character.get_max_health(),
                            state.character.get_attack(),
                            land_x, land_y,
                            tile_x, tile_y
                        ));
                    }
                    "Land View"
                }
                CurrentMode::Combat => "Combat",
            };
            (true, format!(
                "Health: {}/{} | Attack: {} | Land: [{},{}] | Mode: {}",
                state.character.get_health(),
                state.character.get_max_health(),
                state.character.get_attack(),
                land_x, land_y,
                mode_str
            ))
        }
        "" => (false, "Empty command".to_string()),
        _ => (false, format!("Unknown command: {}. Type 'help' for commands.", command)),
    }
}
