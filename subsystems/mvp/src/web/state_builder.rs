use crate::game::game_state::GameState;
use super::serialization::{get_item_name, serialize_inventory, serialize_equipped};
use super::types::*;

/// Build terrain view state (all lands with biome + enemy info, no tiles)
pub fn build_terrain_state(state: &GameState) -> TerrainGameState {
    let mut lands = Vec::new();
    
    // Iterate through all lands in the world (5x5 grid)
    for y in 0..5 {
        for x in 0..5 {
            let coords = (x, y);
            if let Some(land) = state.world.terrain.get(&coords) {
                let enemy = land.enemy.as_ref().map(|e| TerrainEnemyInfo {
                    enemy_type: e.enemy_type.display_name().to_string(),
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
pub fn build_land_state(state: &GameState) -> LandGameState {
    let (land_x, land_y) = state.current_land();
    let current_tile = state.current_tile().unwrap_or((4, 4));
    
    let land = state.world.terrain.get(&(land_x, land_y))
        .expect("Land should exist when in land view");
    
    // Serialize the 8x8 tile grid
    let tiles: Vec<Vec<SerializableTile>> = land.tiles.iter().map(|row| {
        row.iter().map(|tile| {
            let object_names: Vec<String> = tile.items.iter()
                .filter_map(|instance_id| get_item_name(&state.crafting_registry, *instance_id))
                .collect();
            
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
pub fn build_combat_state(state: &GameState) -> CombatGameState {
    let (land_x, land_y) = state.current_land();
    let enemy = state.world.terrain.get(&(land_x, land_y))
        .and_then(|land| land.enemy.as_ref())
        .expect("Enemy should exist when in combat view");
    
    CombatGameState {
        land_coords: (land_x, land_y),
        player: SerializableCombatant {
            health: state.character.get_health(),
            attack: state.get_total_attack(),
        },
        enemy: SerializableCombatant {
            health: enemy.health,
            attack: enemy.attack,
        },
        enemy_type: enemy.enemy_type.display_name().to_string(),
        enemy_max_health: enemy.max_health,
        round: state.combat_round,
    }
}

/// Build serializable character from game state
pub fn build_serializable_character(state: &GameState) -> SerializableCharacter {
    SerializableCharacter {
        health: state.character.get_health(),
        max_health: state.character.get_max_health(),
        attack: state.get_total_attack(),
        inventory: serialize_inventory(state),
        equipped: serialize_equipped(state),
    }
}
