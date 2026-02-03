use crate::types::World;
use crate::terrain_view::TerrainCamera;
use crate::land_view::LandCamera;

/// View mode enum for tracking which view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Terrain view: Shows biome overview (one tile per land)
    Terrain,
    /// Land view: Shows detailed 8x8 tile grid of selected land
    Land,
}

/// Game state that tracks the current world and player position
pub struct GameState {
    pub world: World,
    pub view_mode: ViewMode,
    pub terrain_camera: TerrainCamera,
    pub land_camera: LandCamera,
}

impl GameState {
    /// Create a new game state with the given world
    pub fn new(world: World) -> Self {
        let mut terrain_camera = TerrainCamera::new();
        terrain_camera.update_target();
        
        let mut land_camera = LandCamera::new();
        land_camera.update_target();

        Self {
            world,
            view_mode: ViewMode::Terrain,
            terrain_camera,
            land_camera,
        }
    }

    /// Get current land coordinates
    pub fn current_land(&self) -> (i32, i32) {
        match self.view_mode {
            ViewMode::Terrain => (self.terrain_camera.selected_land_x, self.terrain_camera.selected_land_y),
            ViewMode::Land => (self.land_camera.selected_land_x, self.land_camera.selected_land_y),
        }
    }

    /// Get current tile coordinates within the current land
    /// Returns None if in terrain view
    pub fn current_tile(&self) -> Option<(usize, usize)> {
        match self.view_mode {
            ViewMode::Terrain => None,
            ViewMode::Land => Some((self.land_camera.selected_tile_x, self.land_camera.selected_tile_y)),
        }
    }

    /// Move between lands (terrain view)
    /// Clamps coordinates to 0-4 range
    pub fn move_terrain(&mut self, dx: i32, dy: i32) {
        if self.view_mode != ViewMode::Terrain {
            return;
        }

        let new_x = (self.terrain_camera.selected_land_x + dx).max(0).min(4);
        let new_y = (self.terrain_camera.selected_land_y + dy).max(0).min(4);
        
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

        self.land_camera.move_selection(dx, dy);
    }

    /// Enter land view for the currently selected land
    pub fn enter_land(&mut self) {
        if self.view_mode != ViewMode::Terrain {
            return;
        }

        let (land_x, land_y) = self.current_land();
        self.land_camera.set_land(land_x, land_y);
        
        // Sync land camera position from terrain camera
        let land_center_x = land_x as f32 + 0.5;
        let land_center_y = land_y as f32 + 0.5;
        self.land_camera.sync_position_from(land_center_x, land_center_y);
        
        self.view_mode = ViewMode::Land;
    }

    /// Exit land view and return to terrain view
    pub fn exit_land(&mut self) {
        if self.view_mode != ViewMode::Land {
            return;
        }

        let (land_x, land_y) = self.current_land();
        self.terrain_camera.set_selected_land(land_x, land_y);
        
        // Sync terrain camera position from land camera
        let land_center_x = land_x as f32;
        let land_center_y = land_y as f32;
        self.terrain_camera.sync_position_from(land_center_x, land_center_y);
        
        self.view_mode = ViewMode::Terrain;
    }

    /// Check if a land exists at the given coordinates
    pub fn land_exists(&self, x: i32, y: i32) -> bool {
        self.world.terrain.contains_key(&(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_hardcoded_world;

    #[test]
    fn test_initial_state() {
        let world = create_hardcoded_world();
        let state = GameState::new(world);
        
        assert_eq!(state.view_mode, ViewMode::Terrain);
        assert_eq!(state.current_land(), (0, 0));
        assert_eq!(state.current_tile(), None);
    }

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
        
        state.move_terrain(3, 3);
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
        
        state.exit_land();
        assert_eq!(state.view_mode, ViewMode::Terrain);
        assert_eq!(state.current_land(), (3, 3));
        assert_eq!(state.current_tile(), None);
    }

    #[test]
    fn test_enter_land_only_in_terrain_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
        
        // Try to enter land again (should not work)
        state.enter_land();
        assert_eq!(state.view_mode, ViewMode::Land);
    }

    #[test]
    fn test_exit_land_only_in_land_view() {
        let world = create_hardcoded_world();
        let mut state = GameState::new(world);
        
        assert_eq!(state.view_mode, ViewMode::Terrain);
        
        // Try to exit land (should not work)
        state.exit_land();
        assert_eq!(state.view_mode, ViewMode::Terrain);
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
