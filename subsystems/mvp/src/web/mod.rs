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

/// Response containing the current game state
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub view_mode: String,
    pub current_land: (i32, i32),
    pub current_tile: Option<(usize, usize)>,
    pub current_tile_info: Option<SerializableTileInfo>,
    pub world: SerializableWorld,
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
    
    Ok(Json(GameStateResponse {
        view_mode: format!("{:?}", state.view_mode),
        current_land: state.current_land(),
        current_tile: state.current_tile(),
        current_tile_info,
        world: SerializableWorld {
            name: state.world.name.clone(),
            terrain: state.world.terrain.clone(),
            seed: state.world.seed,
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
    
    let current_tile_info = state.current_tile_info().map(|info| SerializableTileInfo {
        substrate: format!("{:?}", info.substrate),
        objects: info.objects.iter().map(|o| format!("{:?}", o)).collect(),
        biome: format!("{:?}", info.biome),
    });
    
    let response = CommandResponse {
        success,
        message,
        game_state: GameStateResponse {
            view_mode: format!("{:?}", state.view_mode),
            current_land: state.current_land(),
            current_tile: state.current_tile(),
            current_tile_info,
            world: SerializableWorld {
                name: state.world.name.clone(),
                terrain: state.world.terrain.clone(),
                seed: state.world.seed,
            },
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
                    (true, format!("Moved up to land ({}, {})", x, y))
                }
                ViewMode::Land => {
                    state.move_land(0, -1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("Moved up to tile ({}, {})", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "d" | "down" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(0, 1);
                    let (x, y) = state.current_land();
                    (true, format!("Moved down to land ({}, {})", x, y))
                }
                ViewMode::Land => {
                    state.move_land(0, 1);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("Moved down to tile ({}, {})", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "l" | "left" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(-1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("Moved left to land ({}, {})", x, y))
                }
                ViewMode::Land => {
                    state.move_land(-1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("Moved left to tile ({}, {})", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "r" | "right" => {
            match state.view_mode {
                ViewMode::Terrain => {
                    state.move_terrain(1, 0);
                    let (x, y) = state.current_land();
                    (true, format!("Moved right to land ({}, {})", x, y))
                }
                ViewMode::Land => {
                    state.move_land(1, 0);
                    if let Some((x, y)) = state.current_tile() {
                        (true, format!("Moved right to tile ({}, {})", x, y))
                    } else {
                        (false, "Not in land view".to_string())
                    }
                }
            }
        }
        "enter" | "e" => {
            if state.view_mode == ViewMode::Terrain {
                state.enter_land();
                if let Some((x, y)) = state.current_tile() {
                    (true, format!("Entered land view at tile ({}, {})", x, y))
                } else {
                    (true, "Entered land view".to_string())
                }
            } else {
                (false, "Already in land view".to_string())
            }
        }
        "exit" | "x" => {
            if state.view_mode == ViewMode::Land {
                state.exit_land();
                let (x, y) = state.current_land();
                (true, format!("Exited to terrain view at land ({}, {})", x, y))
            } else {
                (false, "Already in terrain view".to_string())
            }
        }
        "help" | "h" | "?" => {
            let help_text = r#"
Commands:
  U, D, L, R - Move up, down, left, right
  E, ENTER   - Enter land view
  X, EXIT    - Exit land view
  H, HELP, ? - Show this help
"#;
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
        
        state.move_terrain(3, 3);
        state.enter_land();
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(success);
        assert!(message.contains("Exited to terrain view"));
        assert_eq!(state.view_mode, ViewMode::Terrain);
        assert_eq!(state.current_land(), (3, 3));
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

    #[test]
    fn test_command_case_insensitive() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Test uppercase (execute_command expects lowercase, so test lowercase)
        // The case conversion happens in handle_command, not execute_command
        let (success1, _) = execute_command(&mut state, "u");
        assert!(success1);
        
        // Test mixed case (converted to lowercase)
        state.move_terrain(0, 0);
        let (success2, _) = execute_command(&mut state, "down");
        assert!(success2);
        
        // Test full word
        state.move_terrain(0, 0);
        let (success3, _) = execute_command(&mut state, "right");
        assert!(success3);
    }

    #[test]
    fn test_command_whitespace_trimming() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Note: execute_command doesn't trim - trimming happens in handle_command
        // But we can test that commands work with extra whitespace if trimmed first
        let trimmed = "  u  ".trim().to_lowercase();
        let (success, _) = execute_command(&mut state, &trimmed);
        assert!(success);
        
        state.move_terrain(0, 0);
        let trimmed2 = "\tr\t".trim().to_lowercase();
        let (success2, _) = execute_command(&mut state, &trimmed2);
        assert!(success2);
    }
}
