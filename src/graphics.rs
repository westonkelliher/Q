use crate::render::Renderer;
use crate::types::{Land, World};

/// View mode for the camera
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Terrain view: Shows biome overview (one tile per land)
    Terrain,
    /// Land view: Shows detailed 8x8 tile grid of selected land
    Land,
}

/// Camera/viewport state for rendering
pub struct Camera {
    /// Current camera position (for smooth following)
    pub x: f32,
    pub y: f32,
    
    /// Target camera position (where we want to be)
    pub target_x: f32,
    pub target_y: f32,
    
    /// Currently selected land coordinates
    pub selected_land_x: i32,
    pub selected_land_y: i32,
    
    /// Currently selected tile within the land (for land view)
    pub selected_tile_x: usize,
    pub selected_tile_y: usize,
    
    /// Current view mode
    pub view_mode: ViewMode,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            selected_land_x: 0,
            selected_land_y: 0,
            selected_tile_x: 0,
            selected_tile_y: 0,
            view_mode: ViewMode::Terrain,
        }
    }

    /// Get tile size based on view mode
    pub fn get_tile_size(&self) -> f32 {
        match self.view_mode {
            ViewMode::Terrain => 48.0,  // Larger tiles for terrain view
            ViewMode::Land => 64.0,     // Larger tiles for detailed land view (8x8 grid)
        }
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let tile_size = self.get_tile_size();
        let screen_x = (world_x - self.x) * tile_size + screen_width / 2.0;
        let screen_y = (world_y - self.y) * tile_size + screen_height / 2.0;
        (screen_x, screen_y)
    }

    /// Get the world position of the selected land center
    pub fn get_selected_land_world_pos(&self) -> (f32, f32) {
        (self.selected_land_x as f32, self.selected_land_y as f32)
    }

    /// Get the world position of the selected tile (for land view)
    pub fn get_selected_tile_world_pos(&self) -> (f32, f32) {
        let land_x = self.selected_land_x as f32;
        let land_y = self.selected_land_y as f32;
        let tile_x = self.selected_tile_x as f32 / 8.0;
        let tile_y = self.selected_tile_y as f32 / 8.0;
        (land_x + tile_x, land_y + tile_y)
    }

    /// Update target position based on current selection
    pub fn update_target(&mut self) {
        match self.view_mode {
            ViewMode::Terrain => {
                self.target_x = self.selected_land_x as f32;
                self.target_y = self.selected_land_y as f32;
            }
            ViewMode::Land => {
                let (tx, ty) = self.get_selected_tile_world_pos();
                self.target_x = tx;
                self.target_y = ty;
            }
        }
    }

    /// Smoothly move camera towards target
    pub fn update(&mut self, delta_time: f32) {
        let follow_speed = 8.0; // How fast camera follows (higher = faster)
        let t = (follow_speed * delta_time).min(1.0);
        
        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;
    }

    /// Move selection in terrain view
    pub fn move_terrain_selection(&mut self, dx: i32, dy: i32) {
        self.selected_land_x += dx;
        self.selected_land_y += dy;
        self.update_target();
    }

    /// Move selection in land view
    pub fn move_land_selection(&mut self, dx: i32, dy: i32) {
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

    /// Switch to terrain view
    pub fn switch_to_terrain_view(&mut self) {
        self.view_mode = ViewMode::Terrain;
        self.update_target();
    }

    /// Switch to land view
    pub fn switch_to_land_view(&mut self) {
        self.view_mode = ViewMode::Land;
        // Reset tile selection to center
        self.selected_tile_x = 4;
        self.selected_tile_y = 4;
        self.update_target();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// Render terrain view - shows biome overview
pub fn render_terrain_view<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &Camera,
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
                
                renderer.draw_biome_overview(screen_x, screen_y, tile_size, &land.biome);
            }
        }
    }

    // Draw selection indicator
    let (sel_x, sel_y) = camera.get_selected_land_world_pos();
    let (screen_x, screen_y) = camera.world_to_screen(sel_x, sel_y, screen_width, screen_height);
    renderer.draw_selection_indicator(screen_x, screen_y, tile_size);

    Ok(())
}

/// Render land view - shows detailed 8x8 tile grid
pub fn render_land_view<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &Camera,
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

/// Render a detailed view of a single land (8x8 tile grid)
pub fn render_land<R: Renderer>(
    renderer: &mut R,
    land: &Land,
    camera: &Camera,
    offset_x: f32,
    offset_y: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let tile_size = camera.get_tile_size();
    let (screen_width, screen_height) = renderer.window_size();

    for (tile_y, row) in land.tiles.iter().enumerate() {
        for (tile_x, tile) in row.iter().enumerate() {
            let world_x = offset_x + tile_x as f32 * tile_size;
            let world_y = offset_y + tile_y as f32 * tile_size;
            let (screen_x, screen_y) = camera.world_to_screen(world_x, world_y, screen_width, screen_height);
            
            renderer.draw_tile(screen_x, screen_y, tile_size, &tile.substrate, &tile.objects);
        }
    }

    Ok(())
}

/// Render a single tile
pub fn render_tile<R: Renderer>(
    renderer: &mut R,
    x: f32,
    y: f32,
    size: f32,
    substrate: &crate::types::Substrate,
    objects: &[crate::types::Object],
) {
    renderer.draw_tile(x, y, size, substrate, objects);
}

/// Handle input for discrete movement and view switching
pub fn handle_input(camera: &mut Camera, keys: &[crate::render::Key]) {
    // Track if we need to check for key presses (not held keys)
    // For discrete movement, we want key_pressed, not key_down
    // But we'll handle this in the graphics_loop by checking key_pressed there
    
    // View switching
    for key in keys {
        match key {
            crate::render::Key::Z => {
                camera.switch_to_land_view();
            }
            crate::render::Key::X => {
                camera.switch_to_terrain_view();
            }
            _ => {}
        }
    }
}
