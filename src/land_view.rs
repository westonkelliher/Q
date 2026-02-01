use crate::render::Renderer;
use crate::types::World;
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

/// Render land view - shows detailed 8x8 tile grid
pub fn render<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &LandCamera,
) -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = renderer.window_size();
    // Use effective tile size (scaled when showing adjacent lands)
    let tile_size = camera.get_effective_tile_size();
    let land_spacing = camera.get_land_spacing();
    
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

        // Render adjacent lands if enabled
        if camera.show_adjacent {
            // Define the 8 adjacent positions relative to the current land
            let adjacent_offsets = [
                (-1, -1), // top-left
                (0, -1),  // top
                (1, -1),  // top-right
                (-1, 0),  // left
                (1, 0),   // right
                (-1, 1),  // bottom-left
                (0, 1),   // bottom
                (1, 1),   // bottom-right
            ];

            for (dx, dy) in adjacent_offsets.iter() {
                let adj_land_x = camera.selected_land_x + dx;
                let adj_land_y = camera.selected_land_y + dy;

                if let Some(adj_land) = world.terrain.get(&(adj_land_x, adj_land_y)) {
                    // Calculate screen position offset for this adjacent land
                    // Add spacing between lands: offset = (grid_width + spacing) * direction
                    let offset_x = *dx as f32 * (grid_width + land_spacing);
                    let offset_y = *dy as f32 * (grid_height + land_spacing);
                    
                    let adj_grid_start_x = grid_start_x + offset_x;
                    let adj_grid_start_y = grid_start_y + offset_y;

                    // Render tiles for adjacent land
                    for (tile_y, row) in adj_land.tiles.iter().enumerate() {
                        for (tile_x, tile) in row.iter().enumerate() {
                            let screen_x = adj_grid_start_x + tile_x as f32 * tile_size;
                            let screen_y = adj_grid_start_y + tile_y as f32 * tile_size;
                            
                            renderer.draw_tile(screen_x, screen_y, tile_size, &tile.substrate, &tile.objects);
                        }
                    }

                    // Draw grid overlay for adjacent land
                    renderer.draw_grid(adj_grid_start_x, adj_grid_start_y, grid_width, grid_height, 8, 8);
                }
            }
        }
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
                // Toggle show adjacent lands
                camera.show_adjacent = !camera.show_adjacent;
            }
            crate::render::Key::Z => {
                // Switch to terrain view (Z toggles views)
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
