use crate::render::Renderer;
use crate::types::World;

/// Camera for land view - manages tile-level selection within a land
pub struct LandCamera {
    /// Current camera position (for smooth following)
    pub x: f32,
    pub y: f32,
    
    /// Target camera position (where we want to be)
    pub target_x: f32,
    pub target_y: f32,
    
    /// Currently selected land coordinates
    pub selected_land_x: i32,
    pub selected_land_y: i32,
    
    /// Currently selected tile within the land
    pub selected_tile_x: usize,
    pub selected_tile_y: usize,
    
    /// Tile size for land view (fixed)
    tile_size: f32,
}

impl LandCamera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            selected_land_x: 0,
            selected_land_y: 0,
            selected_tile_x: 4, // Start at center
            selected_tile_y: 4,
            tile_size: 64.0,
        }
    }

    /// Get tile size for land view
    pub fn get_tile_size(&self) -> f32 {
        self.tile_size
    }

    /// Convert world coordinates to screen coordinates (for land center)
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let screen_x = (world_x - self.x) * self.tile_size + screen_width / 2.0;
        let screen_y = (world_y - self.y) * self.tile_size + screen_height / 2.0;
        (screen_x, screen_y)
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
        self.target_x = tx;
        self.target_y = ty;
    }

    /// Smoothly move camera towards target
    pub fn update(&mut self, delta_time: f32) {
        let follow_speed = 8.0; // How fast camera follows (higher = faster)
        let t = (follow_speed * delta_time).min(1.0);
        
        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;
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
        // Don't update target here - let sync_position_from handle it
    }

    /// Sync camera position from another camera (for smooth view switching)
    /// This should be called after set_land() to ensure smooth transition
    /// When syncing to the land center, sets both position and target to prevent movement
    pub fn sync_position_from(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        // Update target based on current selection
        self.update_target();
        // If we're syncing to the land center (which is where the target should be),
        // set both position and target to match to prevent any camera movement
        let (expected_x, expected_y) = self.get_selected_tile_world_pos();
        // Check if we're syncing to approximately the land center (within 0.1 units)
        if (x - expected_x).abs() < 0.1 && (y - expected_y).abs() < 0.1 {
            // Set both position and target to the land center to prevent movement
            self.x = expected_x;
            self.y = expected_y;
            self.target_x = expected_x;
            self.target_y = expected_y;
        }
    }
}

impl Default for LandCamera {
    fn default() -> Self {
        Self::new()
    }
}

/// Render land view - shows detailed 8x8 tile grid
pub fn render<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &LandCamera,
) -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = renderer.window_size();
    let tile_size = camera.get_tile_size();
    
    // Get the selected land
    if let Some(land) = world.terrain.get(&(camera.selected_land_x, camera.selected_land_y)) {
        // Calculate the center of the land in world coordinates
        let land_center_world_x = camera.selected_land_x as f32 + 0.5;
        let land_center_world_y = camera.selected_land_y as f32 + 0.5;
        
        // Get screen position of the land center
        let (land_center_screen_x, land_center_screen_y) = camera.world_to_screen(
            land_center_world_x,
            land_center_world_y,
            screen_width,
            screen_height
        );
        
        // Calculate the top-left corner of the 8x8 grid in screen space
        // The grid should be centered on the land center
        let grid_width = tile_size * 8.0;
        let grid_height = tile_size * 8.0;
        let grid_start_x = land_center_screen_x - grid_width / 2.0;
        let grid_start_y = land_center_screen_y - grid_height / 2.0;
        
        // Render all tiles in the land - position them directly in screen space
        for (tile_y, row) in land.tiles.iter().enumerate() {
            for (tile_x, tile) in row.iter().enumerate() {
                // Calculate screen position directly based on grid layout
                let screen_x = grid_start_x + tile_x as f32 * tile_size;
                let screen_y = grid_start_y + tile_y as f32 * tile_size;
                
                renderer.draw_tile(screen_x, screen_y, tile_size, &tile.substrate, &tile.objects);
            }
        }
        
        // Draw grid overlay (8x8 grid) - aligned with the tiles
        renderer.draw_grid(grid_start_x, grid_start_y, grid_width, grid_height, 8, 8);
        
        // Draw selection indicator on selected tile
        let selected_screen_x = grid_start_x + camera.selected_tile_x as f32 * tile_size;
        let selected_screen_y = grid_start_y + camera.selected_tile_y as f32 * tile_size;
        renderer.draw_selection_indicator(selected_screen_x, selected_screen_y, tile_size);
    }

    Ok(())
}

/// Handle input for land view
/// Returns true if view should switch to terrain view
pub fn handle_input(camera: &mut LandCamera, keys: &[crate::render::Key]) -> bool {
    // Handle movement
    for key in keys {
        match key {
            crate::render::Key::Up | crate::render::Key::W => {
                camera.move_selection(0, -1);
            }
            crate::render::Key::Down | crate::render::Key::S => {
                camera.move_selection(0, 1);
            }
            crate::render::Key::Left | crate::render::Key::A => {
                camera.move_selection(-1, 0);
            }
            crate::render::Key::Right | crate::render::Key::D => {
                camera.move_selection(1, 0);
            }
            crate::render::Key::X => {
                // Switch to terrain view
                return true;
            }
            _ => {}
        }
    }
    false
}
