use crate::graphics::{handle_input, render_land_view, render_terrain_view, Camera, ViewMode};
use crate::render::macroquad::MacroquadRenderer;
use crate::render::{Color, Key, Renderer};
use crate::types::World;
use macroquad::prelude::*;

/// Run the graphics loop using macroquad
/// This is a convenience function that sets up macroquad and runs the rendering loop
pub async fn run_graphics_loop(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = MacroquadRenderer::new();
    renderer.init()?;

    let mut camera = Camera::new();
    camera.update_target(); // Initialize target position

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
        
        // Handle discrete movement
        for key in &keys_pressed_this_frame {
            match key {
                Key::Up | Key::W => {
                    match camera.view_mode {
                        ViewMode::Terrain => camera.move_terrain_selection(0, -1),
                        ViewMode::Land => camera.move_land_selection(0, -1),
                    }
                }
                Key::Down | Key::S => {
                    match camera.view_mode {
                        ViewMode::Terrain => camera.move_terrain_selection(0, 1),
                        ViewMode::Land => camera.move_land_selection(0, 1),
                    }
                }
                Key::Left | Key::A => {
                    match camera.view_mode {
                        ViewMode::Terrain => camera.move_terrain_selection(-1, 0),
                        ViewMode::Land => camera.move_land_selection(-1, 0),
                    }
                }
                Key::Right | Key::D => {
                    match camera.view_mode {
                        ViewMode::Terrain => camera.move_terrain_selection(1, 0),
                        ViewMode::Land => camera.move_land_selection(1, 0),
                    }
                }
                _ => {}
            }
        }
        
        // Handle view switching and other input
        handle_input(&mut camera, &keys_pressed_this_frame);

        // Smoothly update camera position
        camera.update(delta_time);

        // Check for exit
        if renderer.should_close() {
            break;
        }

        // Clear screen
        renderer.clear(Color::rgb(0.1, 0.1, 0.15)); // Dark blue-gray background

        // Render based on view mode
        match camera.view_mode {
            ViewMode::Terrain => {
                render_terrain_view(&mut renderer, world, &camera)?;
            }
            ViewMode::Land => {
                render_land_view(&mut renderer, world, &camera)?;
            }
        }

        // Draw UI text
        let view_mode_text = match camera.view_mode {
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
