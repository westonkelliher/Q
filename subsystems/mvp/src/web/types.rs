use crate::game::game_state::GameState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
