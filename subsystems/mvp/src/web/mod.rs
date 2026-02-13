use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub mod display;
pub mod types;
pub mod serialization;
pub mod state_builder;

// Re-export public types for convenient access
pub use types::*;

use crate::game::game_state::CurrentMode;
use crate::game::commands::execute_command;
use state_builder::{build_terrain_state, build_land_state, build_combat_state, build_serializable_character};

/// Create the web server router
pub fn create_router(game_state: SharedGameState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/state", get(get_state))
        .route("/api/command", post(handle_command))
        .route("/api/recipes", get(get_recipes))
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
    
    let core_state = match state.current_mode {
        CurrentMode::Terrain => CoreGameState::Terrain(build_terrain_state(&state)),
        CurrentMode::Land => CoreGameState::Land(build_land_state(&state)),
        CurrentMode::Combat => CoreGameState::Combat(build_combat_state(&state)),
    };
    
    Ok(Json(GameStateResponse {
        core_state,
        character: build_serializable_character(&state),
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
            character: build_serializable_character(&state),
        },
    };
    
    Ok(Json(response))
}

/// Get all registered recipes
async fn get_recipes(State(game_state): State<SharedGameState>) -> Result<Json<RecipesResponse>, StatusCode> {
    let state = game_state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let simple_recipes: Vec<_> = state.crafting_registry.all_simple_recipes()
        .map(|r| SerializableSimpleRecipe {
            id: r.id.0.clone(),
            name: r.name.clone(),
            output: r.output.0.clone(),
            output_quantity: r.output_quantity,
            inputs: r.inputs.iter().map(|i| SerializableSimpleInput {
                item_id: i.item_id.0.clone(),
                quantity: i.quantity,
            }).collect(),
            tool: r.tool.as_ref().map(|t| SerializableToolRequirement {
                tool_type: format!("{:?}", t.tool_type),
                min_quality: format!("{:?}", t.min_quality),
            }),
            world_object: r.world_object.as_ref().map(|wo| SerializableWorldObjectRequirement {
                kind: wo.kind.as_ref().map(|k| format!("{:?}", k)),
                required_tags: wo.required_tags.iter().map(|t| t.0.clone()).collect(),
            }),
        })
        .collect();
    
    let component_recipes: Vec<_> = state.crafting_registry.all_component_recipes()
        .map(|r| SerializableComponentRecipe {
            id: r.id.0.clone(),
            name: r.name.clone(),
            output: r.output.0.clone(),
            tool: r.tool.as_ref().map(|t| SerializableToolRequirement {
                tool_type: format!("{:?}", t.tool_type),
                min_quality: format!("{:?}", t.min_quality),
            }),
            world_object: r.world_object.as_ref().map(|wo| SerializableWorldObjectRequirement {
                kind: wo.kind.as_ref().map(|k| format!("{:?}", k)),
                required_tags: wo.required_tags.iter().map(|t| t.0.clone()).collect(),
            }),
        })
        .collect();
    
    let composite_recipes: Vec<_> = state.crafting_registry.all_composite_recipes()
        .map(|r| SerializableCompositeRecipe {
            id: r.id.0.clone(),
            name: r.name.clone(),
            output: r.output.0.clone(),
            tool: r.tool.as_ref().map(|t| SerializableToolRequirement {
                tool_type: format!("{:?}", t.tool_type),
                min_quality: format!("{:?}", t.min_quality),
            }),
            world_object: r.world_object.as_ref().map(|wo| SerializableWorldObjectRequirement {
                kind: wo.kind.as_ref().map(|k| format!("{:?}", k)),
                required_tags: wo.required_tags.iter().map(|t| t.0.clone()).collect(),
            }),
        })
        .collect();
    
    Ok(Json(RecipesResponse {
        simple_recipes,
        component_recipes,
        composite_recipes,
    }))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::create_hardcoded_world;
    use crate::game::crafting::CraftingRegistry;
    use crate::game::game_state::GameState;
    
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
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(success);
        assert!(message.contains("Enter L["));
        assert_eq!(state.current_mode, CurrentMode::Land);
        assert_eq!(state.current_land(), (2, 2));
    }

    #[test]
    fn test_command_enter_land_already_in_land() {
        let mut state = create_test_state();
        
        state.enter_land();
        let (success, message) = execute_command(&mut state, "x");
        
        // When in land view, 'x' exits the land (not an error)
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
        
        // When in terrain view, 'x' enters the land (not an error)
        let (success, message) = execute_command(&mut state, "x");
        
        assert!(success);
        // Should enter land or combat at starting position (0,0)
        assert!(message.contains("Enter L[") || message.contains("Combat"));
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
