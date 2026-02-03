/// Shared camera functionality for both terrain and land views
/// Handles position, zoom, smooth following, and coordinate conversion
pub struct CameraCore {
    /// Current camera position (for smooth following)
    pub x: f32,
    pub y: f32,

    /// Target camera position (where we want to be)
    pub target_x: f32,
    pub target_y: f32,

    /// Base tile size (before zoom)
    base_tile_size: f32,

    /// Zoom level (1.0 = normal, >1.0 = zoomed in, <1.0 = zoomed out)
    zoom: f32,
}

impl CameraCore {
    /// Create a new camera core with the given base tile size
    pub fn new(base_tile_size: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            base_tile_size,
            zoom: 1.0,
        }
    }

    /// Get tile size with zoom applied
    pub fn get_tile_size(&self) -> f32 {
        self.base_tile_size * self.zoom
    }

    /// Get current zoom level
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    /// Zoom in (increase zoom level)
    pub fn zoom_in(&mut self) {
        const ZOOM_STEP: f32 = 1.15;
        const MAX_ZOOM: f32 = 3.0;
        self.zoom = (self.zoom * ZOOM_STEP).min(MAX_ZOOM);
    }

    /// Zoom out (decrease zoom level)
    pub fn zoom_out(&mut self) {
        const ZOOM_STEP: f32 = 1.15;
        const MIN_ZOOM: f32 = 0.5;
        self.zoom = (self.zoom / ZOOM_STEP).max(MIN_ZOOM);
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let tile_size = self.get_tile_size();
        let screen_x = (world_x - self.x) * tile_size + screen_width / 2.0;
        let screen_y = (world_y - self.y) * tile_size + screen_height / 2.0;
        (screen_x, screen_y)
    }

    /// Smoothly move camera towards target
    pub fn update(&mut self, delta_time: f32) {
        const FOLLOW_SPEED: f32 = 8.0;
        let t = (FOLLOW_SPEED * delta_time).min(1.0);

        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;
    }

    /// Set target position
    pub fn set_target(&mut self, target_x: f32, target_y: f32) {
        self.target_x = target_x;
        self.target_y = target_y;
    }

    /// Sync position and target from another camera (for smooth view switching)
    pub fn sync_from(&mut self, other_x: f32, other_y: f32) {
        self.x = other_x;
        self.y = other_y;
        self.target_x = other_x;
        self.target_y = other_y;
    }
}
