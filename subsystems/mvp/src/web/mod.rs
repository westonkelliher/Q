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
use crate::game::commands::execute_command;

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
    pub equipped: Option<String>,
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
            let object_names: Vec<String> = tile.objects.iter().filter_map(|instance_id| {
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
            
            SerializableTile {
                substrate: format!("{:?}", tile.substrate),
                objects: object_names,
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
    
    let inventory_names: Vec<String> = state.character.get_inventory().items.iter().filter_map(|instance_id| {
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
    
    let equipped_name = state.character.get_equipped()
        .and_then(|equipped_id| {
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
        });
    
    Ok(Json(GameStateResponse {
        core_state,
        character: SerializableCharacter {
            health: state.character.get_health(),
            max_health: state.character.get_max_health(),
            attack: state.character.get_attack(),
            inventory: inventory_names,
            equipped: equipped_name,
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
    
    let inventory_names: Vec<String> = state.character.get_inventory().items.iter().filter_map(|instance_id| {
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
    
    let equipped_name = state.character.get_equipped()
        .and_then(|equipped_id| {
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
        });
    
    let response = CommandResponse {
        success,
        message,
        game_state: GameStateResponse {
            core_state,
            character: SerializableCharacter {
                health: state.character.get_health(),
                max_health: state.character.get_max_health(),
                attack: state.character.get_attack(),
                inventory: inventory_names,
                equipped: equipped_name,
            },
        },
    };
    
    Ok(Json(response))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::create_hardcoded_world;
    use crate::game::crafting::CraftingRegistry;
    
    fn create_test_state() -> GameState {
        let mut crafting_registry = CraftingRegistry::new();
        crate::game::crafting::content::register_sample_content(&mut crafting_registry);
        let world = create_hardcoded_world(&mut crafting_registry);
        GameState::new(world, crafting_registry)
    }

    #[test]
    fn test_command_move_up_terrain() {
        let mut state = create_test_state();
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "u");
        
        assert!(success);
        assert!(message.contains("L["));
        assert_eq!(state.current_land(), (2, 1));
    }

    #[test]
    fn test_command_move_down_terrain() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "d");
        
        assert!(success);
        assert!(message.contains("L["));
        assert_eq!(state.current_land(), (0, 1));
    }

    #[test]
    fn test_command_move_left_terrain() {
        let mut state = create_test_state();
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "l");
        
        assert!(success);
        assert!(message.contains("L["));
        assert_eq!(state.current_land(), (1, 2));
    }

    #[test]
    fn test_command_move_right_terrain() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "r");
        
        assert!(success);
        assert!(message.contains("L["));
        assert_eq!(state.current_land(), (1, 0));
    }

    #[test]
    fn test_command_move_up_land() {
        let mut state = create_test_state();
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        state.move_land(0, 2); // Move down first
        
        let (success, message) = execute_command(&mut state, "u");
        
        assert!(success);
        assert!(message.contains("T["));
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.1, initial_tile.1 + 1); // Should be one up from where we moved
    }

    #[test]
    fn test_command_move_down_land() {
        let mut state = create_test_state();
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        
        let (success, message) = execute_command(&mut state, "d");
        
        assert!(success);
        assert!(message.contains("T["));
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.1, initial_tile.1 + 1);
    }

    #[test]
    fn test_command_enter_land() {
        let mut state = create_test_state();
        
        state.move_terrain(2, 2);
        let (success, message) = execute_command(&mut state, "e");
        
        assert!(success);
        assert!(message.contains("Enter L["));
        assert_eq!(state.current_mode, CurrentMode::Land);
        assert_eq!(state.current_land(), (2, 2));
    }

    #[test]
    fn test_command_enter_land_already_in_land() {
        let mut state = create_test_state();
        
        state.enter_land();
        let (success, message) = execute_command(&mut state, "e");
        
        // When in land view, 'e' exits the land (not an error)
        assert!(success);
        assert!(message.contains("Exit L["));
    }

    #[test]
    fn test_command_exit_land() {
        let mut state = create_test_state();
        
        // Use (2,2) which has no enemy
        state.move_terrain(2, 2);
        state.enter_land();
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(success);
        assert!(message.contains("Exit L["));
        assert_eq!(state.current_mode, CurrentMode::Terrain);
        assert_eq!(state.current_land(), (2, 2));
    }

    #[test]
    fn test_command_exit_land_already_in_terrain() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(!success);
        assert!(message.contains("Use 'E' to exit"));
    }

    #[test]
    fn test_command_help() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "help");
        
        assert!(success);
        assert!(message.contains("Commands"));
        assert!(message.contains("Arrow Keys"));
    }

    #[test]
    fn test_command_unknown() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "invalid");
        
        assert!(!success);
        assert!(message.contains("Unknown command"));
    }

    #[test]
    fn test_command_empty() {
        let mut state = create_test_state();
        
        let (success, message) = execute_command(&mut state, "");
        
        assert!(!success);
        assert!(message.contains("Empty command"));
    }

}
