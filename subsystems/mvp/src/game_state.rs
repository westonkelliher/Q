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
