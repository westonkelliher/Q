use crate::render::Renderer;
use crate::types::World;
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

/// Render terrain view - shows biome overview
pub fn render<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &TerrainCamera,
) -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = renderer.window_size();
    let tile_size = camera.get_tile_size();
    
    // Calculate how many tiles we can fit on screen
    let tiles_per_row = (screen_width / tile_size) as i32 + 2;
    let tiles_per_col = (screen_height / tile_size) as i32 + 2;
    
    let start_x = camera.selected_land_x - tiles_per_row / 2;
    let start_y = camera.selected_land_y - tiles_per_col / 2;
    let end_x = camera.selected_land_x + tiles_per_row / 2;
    let end_y = camera.selected_land_y + tiles_per_col / 2;

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if let Some(land) = world.terrain.get(&(x, y)) {
                let world_x = x as f32;
                let world_y = y as f32;
                let (screen_x, screen_y) = camera.world_to_screen(world_x, world_y, screen_width, screen_height);
                
                // Draw single square per land with center biome color and colored borders from edge/corner biomes
                let border_width = 2.0; // Borders to show biome transitions
                renderer.draw_biome_overview_with_borders(
                    screen_x,
                    screen_y,
                    tile_size,
                    &land.center,
                    &land.top,
                    &land.bottom,
                    &land.left,
                    &land.right,
                    &land.top_left,
                    &land.top_right,
                    &land.bottom_left,
                    &land.bottom_right,
                    border_width,
                );
            }
        }
    }

    // Draw selection indicator
    let (sel_x, sel_y) = camera.get_selected_land_world_pos();
    let (screen_x, screen_y) = camera.world_to_screen(sel_x, sel_y, screen_width, screen_height);
    renderer.draw_selection_indicator(screen_x, screen_y, tile_size);

    Ok(())
}

/// Handle input for terrain view
/// Returns true if view should switch to land view
pub fn handle_input(camera: &mut TerrainCamera, keys: &[crate::render::Key]) -> bool {
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
            crate::render::Key::Z => {
                // Switch to land view
                return true;
            }
            crate::render::Key::Minus => {
                // Zoom out
                camera.zoom_out();
            }
            crate::render::Key::Equal => {
                // Zoom in
                camera.zoom_in();
            }
            _ => {}
        }
    }
    false
}
