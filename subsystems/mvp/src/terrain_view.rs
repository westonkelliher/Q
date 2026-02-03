use crate::camera::CameraCore;

/// Camera for terrain view - manages land-level selection and camera following
pub struct TerrainCamera {
    /// Core camera functionality (position, zoom, smooth following)
    core: CameraCore,

    /// Currently selected land coordinates
    pub selected_land_x: i32,
    pub selected_land_y: i32,
}

impl TerrainCamera {
    const BASE_TILE_SIZE: f32 = 48.0;

    pub fn new() -> Self {
        Self {
            core: CameraCore::new(Self::BASE_TILE_SIZE),
            selected_land_x: 0,
            selected_land_y: 0,
        }
    }

    /// Get tile size for terrain view (with zoom applied)
    pub fn get_tile_size(&self) -> f32 {
        self.core.get_tile_size()
    }

    /// Zoom in (increase zoom level)
    pub fn zoom_in(&mut self) {
        self.core.zoom_in();
    }

    /// Zoom out (decrease zoom level)
    pub fn zoom_out(&mut self) {
        self.core.zoom_out();
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        self.core.world_to_screen(world_x, world_y, screen_width, screen_height)
    }

    /// Get the world position of the selected land
    pub fn get_selected_land_world_pos(&self) -> (f32, f32) {
        (self.selected_land_x as f32, self.selected_land_y as f32)
    }

    /// Update target position based on current selection
    pub fn update_target(&mut self) {
        let target_x = self.selected_land_x as f32;
        let target_y = self.selected_land_y as f32;
        self.core.set_target(target_x, target_y);
    }

    /// Smoothly move camera towards target
    pub fn update(&mut self, delta_time: f32) {
        self.core.update(delta_time);
    }

    /// Move selection
    pub fn move_selection(&mut self, dx: i32, dy: i32) {
        self.selected_land_x += dx;
        self.selected_land_y += dy;
        self.update_target();
    }

    /// Set the selected land (used when switching from land view)
    pub fn set_selected_land(&mut self, land_x: i32, land_y: i32) {
        self.selected_land_x = land_x;
        self.selected_land_y = land_y;
        self.update_target();
    }

    /// Sync camera position from another camera (for smooth view switching)
    pub fn sync_position_from(&mut self, x: f32, y: f32) {
        self.core.sync_from(x, y);
    }

    /// Get current camera position (for syncing to other cameras)
    pub fn get_position(&self) -> (f32, f32) {
        (self.core.x, self.core.y)
    }
}

impl Default for TerrainCamera {
    fn default() -> Self {
        Self::new()
    }
}
