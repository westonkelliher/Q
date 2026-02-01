use crate::graphics::{render_world, update_camera, Camera};
use crate::render::macroquad::MacroquadRenderer;
use crate::render::{Color, Renderer};
use crate::types::World;
use macroquad::prelude::*;

/// Run the graphics loop using macroquad
/// This is a convenience function that sets up macroquad and runs the rendering loop
pub async fn run_graphics_loop(world: &World) -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = MacroquadRenderer::new();
    renderer.init()?;

    let mut camera = Camera::new();
    
    // Initial view bounds (can be adjusted)
    let view_x1 = -10;
    let view_y1 = -10;
    let view_x2 = 10;
    let view_y2 = 10;

    loop {
        // Handle input
        let keys = renderer.get_keys_pressed();
        
        // Update camera based on input
        let delta_time = get_frame_time();
        update_camera(&mut camera, &keys, delta_time);

        // Check for exit
        if renderer.should_close() {
            break;
        }

        // Clear screen
        renderer.clear(Color::rgb(0.1, 0.1, 0.15)); // Dark blue-gray background

        // Render world
        render_world(
            &mut renderer,
            world,
            &camera,
            view_x1,
            view_y1,
            view_x2,
            view_y2,
        )?;

        // Draw UI text
        draw_text(
            "WASD/Arrows: Move | Z/X: Zoom | ESC: Exit",
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
