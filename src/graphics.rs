use crate::render::Renderer;
use crate::types::{Land, World};

/// Camera/viewport state for rendering
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let screen_x = (world_x - self.x) * self.zoom + screen_width / 2.0;
        let screen_y = (world_y - self.y) * self.zoom + screen_height / 2.0;
        (screen_x, screen_y)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a biome overview of the world
/// Shows each land as a single colored tile representing its biome
pub fn render_world<R: Renderer>(
    renderer: &mut R,
    world: &World,
    camera: &Camera,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = renderer.window_size();
    let tile_size = 32.0 * camera.zoom;

    // Calculate starting position (centered)
    let start_x = -(x2 - x1 + 1) as f32 * tile_size / 2.0;
    let start_y = -(y2 - y1 + 1) as f32 * tile_size / 2.0;

    for y in y1..=y2 {
        for x in x1..=x2 {
            if let Some(land) = world.terrain.get(&(x, y)) {
                let world_x = start_x + (x - x1) as f32 * tile_size;
                let world_y = start_y + (y - y1) as f32 * tile_size;
                let (screen_x, screen_y) = camera.world_to_screen(world_x, world_y, screen_width, screen_height);
                
                renderer.draw_biome_overview(screen_x, screen_y, tile_size, &land.biome);
            }
        }
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
    let tile_size = 32.0 * camera.zoom;
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

/// Handle camera movement based on input
pub fn update_camera(camera: &mut Camera, keys: &[crate::render::Key], delta_time: f32) {
    let move_speed = 200.0 * delta_time / camera.zoom;

    for key in keys {
        match key {
            crate::render::Key::Up | crate::render::Key::W => {
                camera.y -= move_speed;
            }
            crate::render::Key::Down | crate::render::Key::S => {
                camera.y += move_speed;
            }
            crate::render::Key::Left | crate::render::Key::A => {
                camera.x -= move_speed;
            }
            crate::render::Key::Right | crate::render::Key::D => {
                camera.x += move_speed;
            }
            crate::render::Key::Z => {
                camera.zoom = (camera.zoom * 1.1).min(5.0);
            }
            crate::render::Key::X => {
                camera.zoom = (camera.zoom / 1.1).max(0.1);
            }
            _ => {}
        }
    }
}
