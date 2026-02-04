use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

pub mod display;

use crate::game::game_state::{GameState, CurrentMode};
use crate::game::combat::CombatResult;

/// Shared game state wrapped in Arc<Mutex<>> for thread safety
pub type SharedGameState = Arc<Mutex<GameState>>;

/// Serializable tile for land view
#[derive(Debug, Serialize)]
pub struct SerializableTile {
    pub substrate: String,
    pub objects: Vec<String>,
}

/// Enemy info for terrain view (just status + stats for tooltips)
#[derive(Debug, Serialize)]
pub struct TerrainEnemyInfo {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub is_defeated: bool,
}

/// Land info for terrain view (biome + enemy, no tiles)
#[derive(Debug, Serialize)]
pub struct TerrainLandInfo {
    pub coords: (i32, i32),
    pub biome: String,
    pub enemy: Option<TerrainEnemyInfo>,
}

/// Terrain view state
#[derive(Debug, Serialize)]
pub struct TerrainGameState {
    pub current_land: (i32, i32),
    pub lands: Vec<TerrainLandInfo>,
}

/// Land view state
#[derive(Debug, Serialize)]
pub struct LandGameState {
    pub land_coords: (i32, i32),
    pub current_tile: (usize, usize),
    pub tiles: Vec<Vec<SerializableTile>>,
    pub biome: String,
}

/// Combat view state
#[derive(Debug, Serialize)]
pub struct CombatGameState {
    pub land_coords: (i32, i32),
    pub player: SerializableCombatant,
    pub enemy: SerializableCombatant,
    pub enemy_max_health: i32,
    pub round: u32,
}

/// Core game state discriminated union
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum CoreGameState {
    Terrain(TerrainGameState),
    Land(LandGameState),
    Combat(CombatGameState),
}

/// Serializable character information
#[derive(Debug, Serialize)]
pub struct SerializableCharacter {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub inventory: Vec<String>,
}

/// Serializable combatant information
#[derive(Debug, Serialize)]
pub struct SerializableCombatant {
    pub health: i32,
    pub attack: i32,
}

/// Response containing the current game state
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub core_state: CoreGameState,
    pub character: SerializableCharacter,
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

/// Build terrain view state (all lands with biome + enemy info, no tiles)
fn build_terrain_state(state: &GameState) -> TerrainGameState {
    let mut lands = Vec::new();
    
    // Iterate through all lands in the world (5x5 grid)
    for y in 0..5 {
        for x in 0..5 {
            let coords = (x, y);
            if let Some(land) = state.world.terrain.get(&coords) {
                let enemy = land.enemy.as_ref().map(|e| TerrainEnemyInfo {
                    health: e.health,
                    max_health: e.max_health,
                    attack: e.attack,
                    is_defeated: e.is_defeated(),
                });
                
                lands.push(TerrainLandInfo {
                    coords,
                    biome: format!("{:?}", land.center),
                    enemy,
                });
            }
        }
    }
    
    TerrainGameState {
        current_land: state.current_land(),
        lands,
    }
}

/// Build land view state (current land's tiles only)
fn build_land_state(state: &GameState) -> LandGameState {
    let (land_x, land_y) = state.current_land();
    let current_tile = state.current_tile().unwrap_or((4, 4));
    
    let land = state.world.terrain.get(&(land_x, land_y))
        .expect("Land should exist when in land view");
    
    // Serialize the 8x8 tile grid
    let tiles: Vec<Vec<SerializableTile>> = land.tiles.iter().map(|row| {
        row.iter().map(|tile| {
            SerializableTile {
                substrate: format!("{:?}", tile.substrate),
                objects: tile.objects.iter().map(|o| format!("{:?}", o)).collect(),
            }
        }).collect()
    }).collect();
    
    LandGameState {
        land_coords: (land_x, land_y),
        current_tile,
        tiles,
        biome: format!("{:?}", land.center),
    }
}

/// Build combat view state
fn build_combat_state(state: &GameState) -> CombatGameState {
    let (land_x, land_y) = state.current_land();
    let enemy = state.world.terrain.get(&(land_x, land_y))
        .and_then(|land| land.enemy.as_ref())
        .expect("Enemy should exist when in combat view");
    
    CombatGameState {
        land_coords: (land_x, land_y),
        player: SerializableCombatant {
            health: state.character.get_health(),
            attack: state.character.get_attack(),
        },
        enemy: SerializableCombatant {
            health: enemy.health,
            attack: enemy.attack,
        },
        enemy_max_health: enemy.max_health,
        round: state.combat_round,
    }
}

/// Get the current game state
async fn get_state(State(game_state): State<SharedGameState>) -> Result<Json<GameStateResponse>, StatusCode> {
    let state = game_state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let core_state = match state.current_mode {
        CurrentMode::Terrain => CoreGameState::Terrain(build_terrain_state(&state)),
        CurrentMode::Land => CoreGameState::Land(build_land_state(&state)),
        CurrentMode::Combat => CoreGameState::Combat(build_combat_state(&state)),
    };
    
    Ok(Json(GameStateResponse {
        core_state,
        character: SerializableCharacter {
            health: state.character.get_health(),
            max_health: state.character.get_max_health(),
            attack: state.character.get_attack(),
            inventory: state.character.get_inventory().items.iter().map(|o| format!("{:?}", o)).collect(),
        },
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
    
    let core_state = match state.current_mode {
        CurrentMode::Terrain => CoreGameState::Terrain(build_terrain_state(&state)),
        CurrentMode::Land => CoreGameState::Land(build_land_state(&state)),
        CurrentMode::Combat => CoreGameState::Combat(build_combat_state(&state)),
    };
    
    let response = CommandResponse {
        success,
        message,
        game_state: GameStateResponse {
            core_state,
            character: SerializableCharacter {
                health: state.character.get_health(),
                max_health: state.character.get_max_health(),
                attack: state.character.get_attack(),
                inventory: state.character.get_inventory().items.iter().map(|o| format!("{:?}", o)).collect(),
            },
        },
    };
    
    Ok(Json(response))
}

/// Execute a command and return (success, message)
fn execute_command(state: &mut GameState, command: &str) -> (bool, String) {
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
        assert_eq!(state.current_mode, CurrentMode::Land);
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
        assert_eq!(state.current_mode, CurrentMode::Terrain);
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
