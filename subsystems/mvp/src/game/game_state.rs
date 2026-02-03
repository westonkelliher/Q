use super::world::types::{World, Substrate, Object, Biome};
use super::character::Character;
use super::combat::{CombatState, Combatant, CombatResult};

/// Information about a tile
#[derive(Debug, Clone)]
pub struct TileInfo {
    pub substrate: Substrate,
    pub objects: Vec<Object>,
    pub biome: Biome,
}

use super::world::terrain_view::TerrainCamera;
use super::world::land_view::LandCamera;

/// View mode enum for tracking which view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Terrain view: Shows biome overview (one tile per land)
    Terrain,
    /// Combat view: Combat sequence before entering a land
    Combat,
    /// Land view: Shows detailed 8x8 tile grid of selected land
    Land,
    /// Death screen: Shows when player dies in combat
    DeathScreen,
    /// Win screen: Shows when player wins combat
    WinScreen,
}

/// Game state that tracks the current world and player position
pub struct GameState {
    pub world: World,
    pub view_mode: ViewMode,
    pub terrain_camera: TerrainCamera,
    pub land_camera: LandCamera,
    pub character: Character,
    /// Active combat state (Some when in Combat view mode)
    pub combat_state: Option<CombatState>,
}

impl GameState {
    /// Create a new game state with the given world
    pub fn new(world: World) -> Self {
        let mut terrain_camera = TerrainCamera::new();
        terrain_camera.update_target();
        
        let mut land_camera = LandCamera::new();
        land_camera.update_target();

        let mut character = Character::new();
        character.set_land_position(0, 0);
        character.set_tile_position(None);

        Self {
            world,
            view_mode: ViewMode::Terrain,
            terrain_camera,
            land_camera,
            character,
            combat_state: None,
        }
    }

    /// Get current land coordinates (from character)
    pub fn current_land(&self) -> (i32, i32) {
        self.character.get_land_position()
    }

    /// Get current tile coordinates within the current land (from character)
    /// Returns None if in terrain view
    pub fn current_tile(&self) -> Option<(usize, usize)> {
        self.character.get_tile_position()
    }

    /// Get the center biome of the current land
    pub fn current_biome(&self) -> Option<&Biome> {
        let (x, y) = self.current_land();
        self.world.terrain.get(&(x, y)).map(|land| &land.center)
    }

    /// Get the biome for a specific tile within a land
    /// Based on tile position: corners, edges, or center
    fn get_tile_biome(land: &super::world::types::Land, tile_x: usize, tile_y: usize) -> &super::world::types::Biome {
        let is_top = tile_y == 0;
        let is_bottom = tile_y == 7;
        let is_left = tile_x == 0;
        let is_right = tile_x == 7;
        
        match (is_left, is_right, is_top, is_bottom) {
            // Corners
            (true, false, true, false) => &land.top_left,
            (false, true, true, false) => &land.top_right,
            (true, false, false, true) => &land.bottom_left,
            (false, true, false, true) => &land.bottom_right,
            // Edges
            (_, _, true, false) => &land.top,
            (_, _, false, true) => &land.bottom,
            (true, false, _, _) => &land.left,
            (false, true, _, _) => &land.right,
            // Center
            _ => &land.center,
        }
    }

    /// Get current tile information (substrate, objects, biome)
    /// Returns None if in terrain view or if land doesn't exist
    pub fn current_tile_info(&self) -> Option<TileInfo> {
        if self.view_mode != ViewMode::Land {
            return None;
        }

        let (land_x, land_y) = self.current_land();
        let (tile_x, tile_y) = self.current_tile()?;
        
        let land = self.world.terrain.get(&(land_x, land_y))?;
        let tile = &land.tiles[tile_y][tile_x];
        let biome = Self::get_tile_biome(land, tile_x, tile_y);
        
        Some(TileInfo {
            substrate: tile.substrate.clone(),
            objects: tile.objects.clone(),
            biome: biome.clone(),
        })
    }

    /// Move between lands (terrain view)
    /// Clamps coordinates to 0-4 range
    pub fn move_terrain(&mut self, dx: i32, dy: i32) {
        if self.view_mode != ViewMode::Terrain {
            return;
        }

        let (current_x, current_y) = self.character.get_land_position();
        let new_x = (current_x + dx).max(0).min(4);
        let new_y = (current_y + dy).max(0).min(4);
        
        // Update character position (source of truth)
        self.character.set_land_position(new_x, new_y);
        self.character.set_tile_position(None);
        
        // Update camera to follow character
        self.terrain_camera.selected_land_x = new_x;
        self.terrain_camera.selected_land_y = new_y;
        self.terrain_camera.update_target();
    }

    /// Move within the current land (land view)
    /// Clamps coordinates to 0-7 range
    pub fn move_land(&mut self, dx: i32, dy: i32) {
        if self.view_mode != ViewMode::Land {
            return;
        }

        let (current_x, current_y) = self.character.get_tile_position().unwrap_or((4, 4));
        let new_x = ((current_x as i32) + dx).max(0).min(7) as usize;
        let new_y = ((current_y as i32) + dy).max(0).min(7) as usize;
        
        // Update character position (source of truth)
        self.character.set_tile_position(Some((new_x, new_y)));
        
        // Update camera to follow character
        self.land_camera.selected_tile_x = new_x;
        self.land_camera.selected_tile_y = new_y;
        self.land_camera.update_target();
    }

    /// Enter land view for the currently selected land
    /// If the land has an enemy, enters combat mode instead
    pub fn enter_land(&mut self) {
        if self.view_mode != ViewMode::Terrain {
            return;
        }

        let (land_x, land_y) = self.character.get_land_position();
        
        // Check if land has an enemy
        if let Some(land) = self.world.terrain.get(&(land_x, land_y)) {
            if let Some(enemy) = &land.enemy {
                // Check if enemy is already defeated
                if enemy.is_defeated() {
                    // Enemy defeated, proceed to land view
                    self.enter_land_view_internal(land_x, land_y);
                } else {
                    // Start combat
                    self.start_combat(land_x, land_y, enemy.clone());
                }
            } else {
                // No enemy, proceed to land view
                self.enter_land_view_internal(land_x, land_y);
            }
        }
    }

    /// Internal helper to enter land view (after combat or if no enemy)
    fn enter_land_view_internal(&mut self, land_x: i32, land_y: i32) {
        // Update character to have a tile position (default to center)
        self.character.set_tile_position(Some((4, 4)));
        
        // Update camera to follow character
        self.land_camera.set_land(land_x, land_y);
        
        // Sync land camera position from terrain camera
        let land_center_x = land_x as f32 + 0.5;
        let land_center_y = land_y as f32 + 0.5;
        self.land_camera.sync_position_from(land_center_x, land_center_y);
        
        self.view_mode = ViewMode::Land;
    }

    /// Start combat with an enemy
    fn start_combat(&mut self, _land_x: i32, _land_y: i32, enemy: super::world::types::Enemy) {
        let player_combatant = Combatant::new(
            self.character.get_health(),
            self.character.get_attack(),
        );
        let enemy_combatant = Combatant::new(enemy.health, enemy.attack);
        
        self.combat_state = Some(CombatState::new(player_combatant, enemy_combatant));
        self.view_mode = ViewMode::Combat;
    }

    /// Execute a combat round (attack)
    /// Returns the combat result
    pub fn combat_attack(&mut self) -> CombatResult {
        if let Some(ref mut combat) = self.combat_state {
            let result = combat.execute_round();
            
            // Update character health from combat
            self.character.health = combat.player.health;
            
            // Update enemy health in world
            let (land_x, land_y) = self.character.get_land_position();
            if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
                if let Some(ref mut enemy) = land.enemy {
                    enemy.health = combat.enemy.health;
                }
            }
            
            // Check if combat is over
            match result {
                CombatResult::PlayerWins | CombatResult::Draw => {
                    // Combat won, show win screen
                    self.view_mode = ViewMode::WinScreen;
                }
                CombatResult::EnemyWins => {
                    // Player defeated, show death screen
                    self.view_mode = ViewMode::DeathScreen;
                }
                CombatResult::Ongoing => {
                    // Combat continues
                }
            }
            
            result
        } else {
            CombatResult::Ongoing
        }
    }

    /// Flee from combat (restore all health and return to terrain view)
    pub fn combat_flee(&mut self) {
        if let Some(ref mut combat) = self.combat_state {
            // Restore health
            combat.restore_health(
                self.character.get_max_health(),
                // Get enemy max health from world
                self.world.terrain.get(&self.character.get_land_position())
                    .and_then(|land| land.enemy.as_ref())
                    .map(|enemy| enemy.max_health)
                    .unwrap_or(0),
            );
            
            // Restore character health
            self.character.heal(self.character.get_max_health());
            
            // Restore enemy health in world
            let (land_x, land_y) = self.character.get_land_position();
            if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
                if let Some(ref mut enemy) = land.enemy {
                    enemy.restore_health();
                }
            }
        }
        
        // Exit combat and return to terrain view
        self.combat_state = None;
        self.view_mode = ViewMode::Terrain;
    }

    /// Exit land view and return to terrain view
    pub fn exit_land(&mut self) {
        if self.view_mode != ViewMode::Land {
            return;
        }

        let (land_x, land_y) = self.character.get_land_position();
        
        // Update character to remove tile position
        self.character.set_tile_position(None);
        
        // Update camera to follow character
        self.terrain_camera.set_selected_land(land_x, land_y);
        
        // Sync terrain camera position from land camera
        let land_center_x = land_x as f32;
        let land_center_y = land_y as f32;
        self.terrain_camera.sync_position_from(land_center_x, land_center_y);
        
        self.view_mode = ViewMode::Terrain;
    }

    /// Dismiss death screen (restore health and return to terrain view)
    pub fn dismiss_death_screen(&mut self) {
        if self.view_mode != ViewMode::DeathScreen {
            return;
        }

        // Restore health (same as fleeing)
        if let Some(ref mut combat) = self.combat_state {
            // Restore health
            combat.restore_health(
                self.character.get_max_health(),
                // Get enemy max health from world
                self.world.terrain.get(&self.character.get_land_position())
                    .and_then(|land| land.enemy.as_ref())
                    .map(|enemy| enemy.max_health)
                    .unwrap_or(0),
            );
            
            // Restore character health
            self.character.heal(self.character.get_max_health());
            
            // Restore enemy health in world
            let (land_x, land_y) = self.character.get_land_position();
            if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
                if let Some(ref mut enemy) = land.enemy {
                    enemy.restore_health();
                }
            }
        }
        
        // Exit combat and return to terrain view
        self.combat_state = None;
        self.view_mode = ViewMode::Terrain;
    }

    /// Dismiss win screen (enter land view)
    pub fn dismiss_win_screen(&mut self) {
        if self.view_mode != ViewMode::WinScreen {
            return;
        }

        let (land_x, land_y) = self.character.get_land_position();
        
        // Clear combat state
        self.combat_state = None;
        
        // Enter land view
        self.enter_land_view_internal(land_x, land_y);
    }

    /// Check if a land exists at the given coordinates
    pub fn land_exists(&self, x: i32, y: i32) -> bool {
        self.world.terrain.contains_key(&(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::create_hardcoded_world;

    #[test]
    fn test_terrain_movement() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Move right
        state.move_terrain(1, 0);
        assert_eq!(state.current_land(), (1, 0));
        
        // Move down
        state.move_terrain(0, 1);
        assert_eq!(state.current_land(), (1, 1));
        
        // Move left
        state.move_terrain(-1, 0);
        assert_eq!(state.current_land(), (0, 1));
        
        // Move up
        state.move_terrain(0, -1);
        assert_eq!(state.current_land(), (0, 0));
    }

    #[test]
    fn test_terrain_coordinate_clamping() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Try to move beyond bounds
        state.move_terrain(10, 10);
        assert_eq!(state.current_land(), (4, 4));
        
        // Try to move to negative coordinates
        state.move_terrain(-10, -10);
        assert_eq!(state.current_land(), (0, 0));
    }

    #[test]
    fn test_terrain_movement_only_in_terrain_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Enter land view
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
        
        // Try to move terrain (should not work)
        let original_land = state.current_land();
        state.move_terrain(1, 0);
        assert_eq!(state.current_land(), original_land);
    }

    #[test]
    fn test_enter_land_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.move_terrain(2, 2);
        assert_eq!(state.view_mode, ViewMode::Terrain);
        
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
        assert_eq!(state.current_land(), (2, 2));
        assert!(state.current_tile().is_some());
    }

    #[test]
    fn test_land_movement() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        
        // Move right
        state.move_land(1, 0);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0 + 1);
        assert_eq!(tile.1, initial_tile.1);
        
        // Move down
        state.move_land(0, 1);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0 + 1);
        assert_eq!(tile.1, initial_tile.1 + 1);
        
        // Move left
        state.move_land(-1, 0);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0);
        assert_eq!(tile.1, initial_tile.1 + 1);
        
        // Move up
        state.move_land(0, -1);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, initial_tile);
    }

    #[test]
    fn test_land_coordinate_clamping() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        // Initial position is (4, 4) - center of the land
        
        // Try to move beyond bounds from (4, 4)
        state.move_land(10, 10);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, (7, 7)); // Clamped to max
        
        // Try to move to negative coordinates from (7, 7)
        state.move_land(-10, -10);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, (0, 0)); // Clamped to min
    }

    #[test]
    fn test_land_movement_only_in_land_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        assert_eq!(state.view_mode, ViewMode::Terrain);
        
        // Try to move land (should not work)
        state.move_land(1, 0);
        assert_eq!(state.current_tile(), None);
    }

    #[test]
    fn test_exit_land_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        // Use (2,2) which has no enemy, or (0,0) which is the start
        state.move_terrain(2, 2);
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
        
        state.exit_land();
        assert_eq!(state.view_mode, ViewMode::Terrain);
        assert_eq!(state.current_land(), (2, 2));
        assert_eq!(state.current_tile(), None);
    }

    #[test]
    fn test_land_exists() {
        let world = create_hardcoded_world();
        let state = GameState::new(world);
        
        // Test existing lands
        assert!(state.land_exists(0, 0));
        assert!(state.land_exists(2, 2));
        assert!(state.land_exists(4, 4));
        
        // Test non-existing lands
        assert!(!state.land_exists(5, 5));
        assert!(!state.land_exists(-1, -1));
    }
}
