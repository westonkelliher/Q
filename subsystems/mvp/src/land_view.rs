use crate::camera::CameraCore;

/// Spacing between adjacent lands in pixels
const LAND_SPACING: f32 = 30.0;

/// Scale factor for tiles when showing adjacent lands (makes tiles smaller to fit more)
const ADJACENT_SCALE: f32 = 0.65;

/// Camera for land view - manages tile-level selection within a land
pub struct LandCamera {
    /// Core camera functionality (position, zoom, smooth following)
    core: CameraCore,

    /// Currently selected land coordinates
    pub selected_land_x: i32,
    pub selected_land_y: i32,

    /// Currently selected tile within the land
    pub selected_tile_x: usize,
    pub selected_tile_y: usize,

    /// Whether to show the 8 adjacent lands
    pub show_adjacent: bool,
}

impl LandCamera {
    const BASE_TILE_SIZE: f32 = 64.0;

    pub fn new() -> Self {
        Self {
            core: CameraCore::new(Self::BASE_TILE_SIZE),
            selected_land_x: 0,
            selected_land_y: 0,
            selected_tile_x: 4, // Start at center
            selected_tile_y: 4,
            show_adjacent: false,
        }
    }

    /// Get tile size for land view (base size with zoom applied)
    pub fn get_tile_size(&self) -> f32 {
        self.core.get_tile_size()
    }

    /// Get effective tile size (scaled when showing adjacent lands, with zoom applied)
    pub fn get_effective_tile_size(&self) -> f32 {
        let zoomed_size = self.core.get_tile_size();
        if self.show_adjacent {
            zoomed_size * ADJACENT_SCALE
        } else {
            zoomed_size
        }
    }

    /// Zoom in (increase zoom level)
    pub fn zoom_in(&mut self) {
        self.core.zoom_in();
    }

    /// Zoom out (decrease zoom level)
    pub fn zoom_out(&mut self) {
        self.core.zoom_out();
    }

    /// Get spacing between lands (only used when showing adjacent)
    pub fn get_land_spacing(&self) -> f32 {
        if self.show_adjacent {
            LAND_SPACING
        } else {
            0.0
        }
    }

    /// Convert world coordinates to screen coordinates (for land center)
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        self.core.world_to_screen(world_x, world_y, screen_width, screen_height)
    }

    /// Get the world position of the selected tile
    pub fn get_selected_tile_world_pos(&self) -> (f32, f32) {
        let land_x = self.selected_land_x as f32;
        let land_y = self.selected_land_y as f32;
        let tile_x = self.selected_tile_x as f32 / 8.0;
        let tile_y = self.selected_tile_y as f32 / 8.0;
        (land_x + tile_x, land_y + tile_y)
    }

    /// Update target position based on current selection
    pub fn update_target(&mut self) {
        let (tx, ty) = self.get_selected_tile_world_pos();
        self.core.set_target(tx, ty);
    }

    /// Smoothly move camera towards target
    pub fn update(&mut self, delta_time: f32) {
        self.core.update(delta_time);
    }

    /// Move selection within the land
    pub fn move_selection(&mut self, dx: i32, dy: i32) {
        let new_x = self.selected_tile_x as i32 + dx;
        let new_y = self.selected_tile_y as i32 + dy;

        // Clamp to 0-7 range
        if new_x >= 0 && new_x < 8 {
            self.selected_tile_x = new_x as usize;
        }
        if new_y >= 0 && new_y < 8 {
            self.selected_tile_y = new_y as usize;
        }

        self.update_target();
    }

    /// Set which land is being viewed (used when switching from terrain view)
    pub fn set_land(&mut self, land_x: i32, land_y: i32) {
        self.selected_land_x = land_x;
        self.selected_land_y = land_y;
        // Reset tile selection to center
        self.selected_tile_x = 4;
        self.selected_tile_y = 4;
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

impl Default for LandCamera {
    fn default() -> Self {
        Self::new()
    }
}
