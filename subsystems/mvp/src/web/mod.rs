use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

pub mod display;

use crate::game::game_state::{GameState, ViewMode};
use crate::game::world::types::{Biome, Object, Substrate};
use crate::game::combat::CombatResult;

/// Shared game state wrapped in Arc<Mutex<>> for thread safety
pub type SharedGameState = Arc<Mutex<GameState>>;

/// Serializable Biome with color information
#[derive(Debug, Serialize)]
pub struct SerializableBiome {
    pub name: String,
    pub color: [f32; 3], // RGB values 0.0-1.0
}

impl From<&Biome> for SerializableBiome {
    fn from(biome: &Biome) -> Self {
        let (r, g, b) = biome.to_color();
        Self {
            name: format!("{:?}", biome),
            color: [r, g, b],
        }
    }
}

/// Serializable Substrate with color information
#[derive(Debug, Serialize)]
pub struct SerializableSubstrate {
    pub name: String,
    pub color: [f32; 3], // RGB values 0.0-1.0
}

impl From<&Substrate> for SerializableSubstrate {
    fn from(substrate: &Substrate) -> Self {
        let (r, g, b) = substrate.to_color();
        Self {
            name: format!("{:?}", substrate),
            color: [r, g, b],
        }
    }
}

/// Serializable Object with color information
#[derive(Debug, Serialize)]
pub struct SerializableObject {
    pub name: String,
    pub color: [f32; 3], // RGB values 0.0-1.0
}

impl From<&Object> for SerializableObject {
    fn from(object: &Object) -> Self {
        let (r, g, b) = object.to_color();
        Self {
            name: format!("{:?}", object),
            color: [r, g, b],
        }
    }
}

/// Serializable wrapper for World that converts HashMap<(i32, i32), Land> to JSON object
#[derive(Debug, Serialize)]
pub struct SerializableWorld {
    pub name: String,
    #[serde(serialize_with = "serialize_terrain")]
    pub terrain: HashMap<(i32, i32), crate::Land>,
    pub seed: u64,
}

fn serialize_terrain<S>(
    terrain: &HashMap<(i32, i32), crate::Land>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(terrain.len()))?;
    for ((x, y), land) in terrain {
        let key = format!("{},{}", x, y);
        map.serialize_entry(&key, land)?;
    }
    map.end()
}

/// Serializable tile information
#[derive(Debug, Serialize)]
pub struct SerializableTileInfo {
    pub substrate: String,
    pub objects: Vec<String>,
    pub biome: String,
}

/// Serializable character information
#[derive(Debug, Serialize)]
pub struct SerializableCharacter {
    pub land_position: (i32, i32),
    pub tile_position: Option<(usize, usize)>,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
}

/// Serializable combatant information
#[derive(Debug, Serialize)]
pub struct SerializableCombatant {
    pub health: i32,
    pub attack: i32,
}

/// Serializable combat state
#[derive(Debug, Serialize)]
pub struct SerializableCombatState {
    pub player: SerializableCombatant,
    pub enemy: SerializableCombatant,
    pub enemy_max_health: i32,
    pub round: u32,
}

/// Response containing the current game state
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub view_mode: String,
    pub current_land: (i32, i32),
    pub current_tile: Option<(usize, usize)>,
    pub current_tile_info: Option<SerializableTileInfo>,
    pub current_biome: Option<String>, // Center biome of current land (for Terrain view)
    pub world: SerializableWorld,
    pub character: SerializableCharacter,
    pub combat_state: Option<SerializableCombatState>,
}

/// Command request from the client
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub command: String,
}

/// Command response
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub message: String,
    pub game_state: GameStateResponse,
}

/// Create the web server router
pub fn create_router(game_state: SharedGameState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/state", get(get_state))
        .route("/api/command", post(handle_command))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("../../assets"))
        .with_state(game_state)
}

/// Serve the main HTML page
async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

/// Get the current game state
async fn get_state(State(game_state): State<SharedGameState>) -> Result<Json<GameStateResponse>, StatusCode> {
    let state = game_state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let current_tile_info = state.current_tile_info().map(|info| SerializableTileInfo {
        substrate: format!("{:?}", info.substrate),
        objects: info.objects.iter().map(|o| format!("{:?}", o)).collect(),
        biome: format!("{:?}", info.biome),
    });
    
    let current_biome = state.current_biome().map(|b| format!("{:?}", b));
    
    // Serialize combat state if in combat mode
    let combat_state = if state.view_mode == ViewMode::Combat {
        if let Some(ref combat) = state.combat_state {
            let (land_x, land_y) = state.current_land();
            let enemy_max_health = state.world.terrain.get(&(land_x, land_y))
                .and_then(|land| land.enemy.as_ref())
                .map(|enemy| enemy.max_health)
                .unwrap_or(combat.enemy.health); // Fallback to current health if max not available
            
            Some(SerializableCombatState {
                player: SerializableCombatant {
                    health: combat.player.health,
                    attack: combat.player.attack,
                },
                enemy: SerializableCombatant {
                    health: combat.enemy.health,
                    attack: combat.enemy.attack,
                },
                enemy_max_health,
                round: combat.round,
            })
        } else {
            None
        }
    } else {
        None
    };
    
    Ok(Json(GameStateResponse {
        view_mode: format!("{:?}", state.view_mode),
        current_land: state.current_land(),
        current_tile: state.current_tile(),
        current_tile_info,
        current_biome,
        world: SerializableWorld {
            name: state.world.name.clone(),
            terrain: state.world.terrain.clone(),
            seed: state.world.seed,
        },
        character: SerializableCharacter {
            land_position: state.character.get_land_position(),
            tile_position: state.character.get_tile_position(),
            health: state.character.get_health(),
            max_health: state.character.get_max_health(),
            attack: state.character.get_attack(),
        },
        combat_state,
    }))
}

/// Handle a command from the client
async fn handle_command(
    State(game_state): State<SharedGameState>,
    Json(req): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    let mut state = game_state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let command = req.command.trim().to_lowercase();
    let (success, message) = execute_command(&mut state, &command);
    
    let current_tile_info = state.current_tile_info().map(|info| SerializableTileInfo {
        substrate: format!("{:?}", info.substrate),
        objects: info.objects.iter().map(|o| format!("{:?}", o)).collect(),
        biome: format!("{:?}", info.biome),
    });
    
    let current_biome = state.current_biome().map(|b| format!("{:?}", b));
    
    // Serialize combat state if in combat mode
    let combat_state = if state.view_mode == ViewMode::Combat {
        if let Some(ref combat) = state.combat_state {
            let (land_x, land_y) = state.current_land();
            let enemy_max_health = state.world.terrain.get(&(land_x, land_y))
                .and_then(|land| land.enemy.as_ref())
                .map(|enemy| enemy.max_health)
                .unwrap_or(combat.enemy.health); // Fallback to current health if max not available
            
            Some(SerializableCombatState {
                player: SerializableCombatant {
                    health: combat.player.health,
                    attack: combat.player.attack,
                },
                enemy: SerializableCombatant {
                    health: combat.enemy.health,
                    attack: combat.enemy.attack,
                },
                enemy_max_health,
                round: combat.round,
            })
        } else {
            None
        }
    } else {
        None
    };
    
    let response = CommandResponse {
        success,
        message,
        game_state: GameStateResponse {
            view_mode: format!("{:?}", state.view_mode),
            current_land: state.current_land(),
            current_tile: state.current_tile(),
            current_tile_info,
            current_biome,
            world: SerializableWorld {
                name: state.world.name.clone(),
                terrain: state.world.terrain.clone(),
                seed: state.world.seed,
            },
            character: SerializableCharacter {
                land_position: state.character.get_land_position(),
                tile_position: state.character.get_tile_position(),
                health: state.character.get_health(),
                max_health: state.character.get_max_health(),
                attack: state.character.get_attack(),
            },
            combat_state,
        },
    };
    
    Ok(Json(response))
}

/// Execute a command and return (success, message)
fn execute_command(state: &mut GameState, command: &str) -> (bool, String) {
    match command {
        "u" | "up" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(0, -1);
                    let (x, y) = state.current_land();
                    (true, format!("⬆️ L[{},{}]", x, y))
                }
                ViewMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'f' to flee.".to_string())
                }
                ViewMode::Land => {
                    state.move_land(0, -1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬆️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
                ViewMode::DeathScreen | ViewMode::WinScreen => {
                    (false, "Press ENTER to continue.".to_string())
                }
            }
        }
        "d" | "down" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(0, 1);
                    let (x, y) = state.current_land();
                    (true, format!("⬇️ L[{},{}]", x, y))
                }
                ViewMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'f' to flee.".to_string())
                }
                ViewMode::Land => {
                    state.move_land(0, 1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬇️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
                ViewMode::DeathScreen | ViewMode::WinScreen => {
                    (false, "Press ENTER to continue.".to_string())
                }
            }
        }
        "l" | "left" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(-1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("⬅️ L[{},{}]", x, y))
                }
                ViewMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'f' to flee.".to_string())
                }
                ViewMode::Land => {
                    state.move_land(-1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("⬅️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
                ViewMode::DeathScreen | ViewMode::WinScreen => {
                    (false, "Press ENTER to continue.".to_string())
                }
            }
        }
        "r" | "right" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("➡️ L[{},{}]", x, y))
                }
                ViewMode::Combat => {
                    (false, "Cannot move during combat. Use 'a' to attack or 'f' to flee.".to_string())
                }
                ViewMode::Land => {
                    state.move_land(1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("➡️ T[{},{}]", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
                ViewMode::DeathScreen | ViewMode::WinScreen => {
                    (false, "Press ENTER to continue.".to_string())
                }
            }
        }
        "enter" | "e" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    let (land_x, land_y) = state.current_land();
                    state.enter_land();
                    
                    if state.view_mode == ViewMode::Combat {
                        (true, "⚔️ Combat!".to_string())
                    } else {
                        (true, format!("🔽 Enter L[{},{}]", land_x, land_y))
                    }
                }
                ViewMode::DeathScreen => {
                    state.dismiss_death_screen();
                    (true, "💀 Resurrected!".to_string())
                }
                ViewMode::WinScreen => {
                    state.dismiss_win_screen();
                    (true, "🎉 Victory!".to_string())
                }
                _ => {
                    (false, "Already in land view".to_string())
                }
            }
        }
        "exit" | "x" => {
            if state.view_mode == ViewMode::Land {
                let (x, y) = state.current_land();
                state.exit_land();
                (true, format!("🔼 Exit L[{},{}]", x, y))
            } else {
                (false, "Already in terrain view".to_string())
            }
        }
        "attack" | "a" => {
            if state.view_mode == ViewMode::Combat {
                let result = state.combat_attack();
                match result {
                    CombatResult::Ongoing => {
                        let combat = state.combat_state.as_ref().unwrap();
                        (true, format!("⚔️ Attack! P:{}/{} E:{}/{}", 
                            combat.player.health, state.character.get_max_health(),
                            combat.enemy.health, 
                            state.world.terrain.get(&state.current_land())
                                .and_then(|land| land.enemy.as_ref())
                                .map(|e| e.max_health)
                                .unwrap_or(0)))
                    }
                    CombatResult::PlayerWins => {
                        (true, "⚔️ Victory!".to_string())
                    }
                    CombatResult::EnemyWins => {
                        (true, "⚔️ Defeated!".to_string())
                    }
                    CombatResult::Draw => {
                        (true, "⚔️ Draw!".to_string())
                    }
                }
            } else {
                (false, "Not in combat. Use 'E' or 'ENTER' to enter a land with enemies.".to_string())
            }
        }
        "flee" | "f" => {
            if state.view_mode == ViewMode::Combat {
                state.combat_flee();
                (true, "🏃 Flee!".to_string())
            } else {
                (false, "Not in combat.".to_string())
            }
        }
        "help" | "h" | "?" => {
            let help_text = match state.view_mode {
                ViewMode::Combat => {
                    r#"
Combat Commands:
  A, ATTACK - Attack the enemy
  F, FLEE   - Flee combat (restores all health)
  H, HELP   - Show this help
"#
                }
                ViewMode::DeathScreen => {
                    r#"
Death Screen:
  E, ENTER  - Continue (restore health and return to terrain view)
  H, HELP   - Show this help
"#
                }
                ViewMode::WinScreen => {
                    r#"
Victory Screen:
  E, ENTER  - Continue (enter land view)
  H, HELP   - Show this help
"#
                }
                _ => {
                    r#"
Commands:
  U, D, L, R - Move up, down, left, right
  E, ENTER   - Enter land view (may trigger combat if enemy present)
  X, EXIT    - Exit land view
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::create_hardcoded_world;

    #[test]
    fn test_command_move_up_terrain() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "u");
        
        assert!(success);
        assert!(message.contains("Moved up"));
        assert_eq!(state.current_land(), (2, 1));
    }

    #[test]
    fn test_command_move_down_terrain() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "d");
        
        assert!(success);
        assert!(message.contains("Moved down"));
        assert_eq!(state.current_land(), (0, 1));
    }

    #[test]
    fn test_command_move_left_terrain() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "l");
        
        assert!(success);
        assert!(message.contains("Moved left"));
        assert_eq!(state.current_land(), (1, 2));
    }

    #[test]
    fn test_command_move_right_terrain() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "r");
        
        assert!(success);
        assert!(message.contains("Moved right"));
        assert_eq!(state.current_land(), (1, 0));
    }

    #[test]
    fn test_command_move_up_land() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        state.move_land(0, 2); // Move down first
        
        let (success, message) = execute_command(&mut state, "u");
        
        assert!(success);
        assert!(message.contains("Moved up"));
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.1, initial_tile.1 + 1); // Should be one up from where we moved
    }

    #[test]
    fn test_command_move_down_land() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        
        let (success, message) = execute_command(&mut state, "d");
        
        assert!(success);
        assert!(message.contains("Moved down"));
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.1, initial_tile.1 + 1);
    }

    #[test]
    fn test_command_enter_land() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "e");
        
        assert!(success);
        assert!(message.contains("Entered land view"));
        assert_eq!(state.view_mode, ViewMode::Land);
        assert_eq!(state.current_land(), (2, 2));
    }

    #[test]
    fn test_command_enter_land_already_in_land() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        let (success, message) = execute_command(&mut state, "e");
        
        assert!(!success);
        assert!(message.contains("Already in land view"));
    }

    #[test]
    fn test_command_exit_land() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Use (2,2) which has no enemy
        state.move_terrain(2, 2);
        state.enter_land();
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(success);
        assert!(message.contains("Exited to terrain view"));
        assert_eq!(state.view_mode, ViewMode::Terrain);
        assert_eq!(state.current_land(), (2, 2));
    }

    #[test]
    fn test_command_exit_land_already_in_terrain() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(!success);
        assert!(message.contains("Already in terrain view"));
    }

    #[test]
    fn test_command_help() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "help");
        
        assert!(success);
        assert!(message.contains("Commands"));
        assert!(message.contains("U, D, L, R"));
    }

    #[test]
    fn test_command_unknown() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "invalid");
        
        assert!(!success);
        assert!(message.contains("Unknown command"));
    }

    #[test]
    fn test_command_empty() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        let (success, message) = execute_command(&mut state, "");
        
        assert!(!success);
        assert!(message.contains("Empty command"));
    }

}
