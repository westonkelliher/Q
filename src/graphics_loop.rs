use crate::land_view::{self, LandCamera};
use crate::render::macroquad::MacroquadRenderer;
use crate::render::{Color, Key, Renderer};
use crate::terrain_view::{self, TerrainCamera};
use crate::types::World;
use macroquad::prelude::*;

/// View mode enum for tracking which view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Terrain view: Shows biome overview (one tile per land)
    Terrain,
    /// Land view: Shows detailed 8x8 tile grid of selected land
    Land,
}

/// Run the graphics loop using macroquad
/// This is a convenience function that sets up macroquad and runs the rendering loop
pub async fn run_graphics_loop(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = MacroquadRenderer::new();
    renderer.init()?;

    // Separate cameras for each view
    let mut terrain_camera = TerrainCamera::new();
    terrain_camera.update_target(); // Initialize target position
    
    let mut land_camera = LandCamera::new();
    land_camera.update_target(); // Initialize target position
    
    // Track current view mode
    let mut view_mode = ViewMode::Terrain;

    loop {
        let delta_time = get_frame_time();
        
        // Check for discrete key presses (not held keys)
        let mut keys_pressed_this_frame = Vec::new();
        let key_codes = [
            (KeyCode::Up, Key::Up),
            (KeyCode::Down, Key::Down),
            (KeyCode::Left, Key::Left),
            (KeyCode::Right, Key::Right),
            (KeyCode::W, Key::W),
            (KeyCode::A, Key::A),
            (KeyCode::S, Key::S),
            (KeyCode::D, Key::D),
            (KeyCode::Z, Key::Z),
            (KeyCode::X, Key::X),
        ];
        
        for (mq_key, our_key) in key_codes {
            if is_key_pressed(mq_key) {
                keys_pressed_this_frame.push(our_key);
            }
        }
        
        // Handle input based on current view mode
        let should_switch_view = match view_mode {
            ViewMode::Terrain => {
                let should_switch = terrain_view::handle_input(&mut terrain_camera, &keys_pressed_this_frame);
                if should_switch {
                    // Sync land camera with terrain camera's selection
                    // Set the land first, then sync to the land center (not terrain camera position)
                    // This prevents the camera from moving when switching views
                    land_camera.set_land(terrain_camera.selected_land_x, terrain_camera.selected_land_y);
                    // Land view centers on (land_x + 0.5, land_y + 0.5), so sync to that position
                    let land_center_x = terrain_camera.selected_land_x as f32 + 0.5;
                    let land_center_y = terrain_camera.selected_land_y as f32 + 0.5;
                    land_camera.sync_position_from(land_center_x, land_center_y);
                    true
                } else {
                    false
                }
            }
            ViewMode::Land => {
                let should_switch = land_view::handle_input(&mut land_camera, &keys_pressed_this_frame);
                if should_switch {
                    // Sync terrain camera with land camera's selection
                    // Terrain view centers on (land_x, land_y), so sync to that position
                    terrain_camera.set_selected_land(land_camera.selected_land_x, land_camera.selected_land_y);
                    let land_center_x = land_camera.selected_land_x as f32;
                    let land_center_y = land_camera.selected_land_y as f32;
                    terrain_camera.sync_position_from(land_center_x, land_center_y);
                    true
                } else {
                    false
                }
            }
        };
        
        // Switch view if needed
        if should_switch_view {
            view_mode = match view_mode {
                ViewMode::Terrain => ViewMode::Land,
                ViewMode::Land => ViewMode::Terrain,
            };
        }

        // Update cameras
        terrain_camera.update(delta_time);
        land_camera.update(delta_time);

        // Check for exit
        if renderer.should_close() {
            break;
        }

        // Clear screen
        renderer.clear(Color::rgb(0.1, 0.1, 0.15)); // Dark blue-gray background

        // Render based on view mode
        match view_mode {
            ViewMode::Terrain => {
                terrain_view::render(&mut renderer, world, &terrain_camera)?;
            }
            ViewMode::Land => {
                land_view::render(&mut renderer, world, &land_camera)?;
            }
        }

        // Draw UI text
        let view_mode_text = match view_mode {
            ViewMode::Terrain => "Terrain View",
            ViewMode::Land => "Land View",
        };
        draw_text(
            &format!("WASD/Arrows: Move | Z: Land View | X: Terrain View | ESC: Exit | Mode: {}", view_mode_text),
            10.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );

        // Present frame (macroquad handles this automatically, but we call it for completeness)
        renderer.present()?;
        
        // Wait for next frame
        next_frame().await;
    }

    Ok(())
}
