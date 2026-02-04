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
        "help" | "h" | "?" => {
            let help_text = match state.current_mode {
                CurrentMode::Combat => {
                    r#"
Combat Commands:
  A, ATTACK - Attack the enemy
  E, ENTER  - Flee combat (returns to terrain view)
  H, HELP   - Show this help
"#
                }
                CurrentMode::Land => {
                    r#"
Commands:
  U, D, L, R - Move up, down, left, right
  E, ENTER   - Exit land view
  H, HELP, ? - Show this help
"#
                }
                _ => {
                    r#"
Commands:
  U, D, L, R - Move up, down, left, right
  E, ENTER   - Enter land view (may trigger combat if enemy present)
  H, HELP, ? - Show this help
"#
                }
            };
            (true, help_text.trim().to_string())
        }
        "" => (false, "Empty command".to_string()),
        _ => (false, format!("Unknown command: {}. Type 'help' for commands.", command)),
    }
}
