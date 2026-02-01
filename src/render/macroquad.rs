use macroquad::prelude::*;
use crate::render::{Color, Key, RenderError, Renderer};
use crate::types::{Biome, Object, Substrate};

/// Macroquad-based implementation of the Renderer trait
pub struct MacroquadRenderer {
    initialized: bool,
}

impl MacroquadRenderer {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    /// Convert our Color type to macroquad's Color
    fn to_mq_color(color: Color) -> macroquad::prelude::Color {
        macroquad::prelude::Color::new(color.r, color.g, color.b, color.a)
    }

    /// Convert substrate to color
    fn substrate_color(substrate: &Substrate) -> Color {
        match substrate {
            Substrate::Grass => Color::rgb(0.2, 0.8, 0.2),      // Green
            Substrate::Dirt => Color::rgb(0.6, 0.4, 0.2),       // Brown
            Substrate::Stone => Color::rgb(0.7, 0.7, 0.7),      // Gray
            Substrate::Mud => Color::rgb(0.4, 0.3, 0.2),        // Dark brown
            Substrate::Water => Color::rgb(0.2, 0.4, 0.9),      // Blue
            Substrate::Brush => Color::rgb(0.6, 0.8, 0.3),      // Yellow-green
        }
    }

    /// Convert biome to color
    fn biome_color(biome: &Biome) -> Color {
        match biome {
            Biome::Forest => Color::rgb(0.1, 0.5, 0.1),         // Dark green
            Biome::Meadow => Color::rgb(0.7, 0.9, 0.4),         // Light green/yellow
            Biome::Lake => Color::rgb(0.2, 0.5, 0.9),           // Blue
            Biome::Mountain => Color::rgb(0.8, 0.8, 0.85),      // Gray/white
        }
    }

    /// Create a natural shadow color using shifting approach
    /// Darkens colors and applies blue shift through cascading color shifts
    fn shadow_color(color: Color) -> Color {
        // Apply shifting: darken and shift colors toward blue
        let shadow_r = color.r * 0.7;
        let shadow_g = color.g * 0.72 + color.r * 0.05;
        let shadow_b = color.b * 0.75 + color.g * 0.05; // Higher blue component creates the blue shift
            
        Color::rgb(
            shadow_r.min(1.0),
            shadow_g.min(1.0),
            shadow_b.min(1.0),
        )
    }

    /// Convert object to color
    fn object_color(object: &Object) -> Color {
        match object {
            Object::Rock => Color::rgb(0.3, 0.3, 0.3),          // Dark gray
            Object::Tree => Color::rgb(0.1, 0.6, 0.1),          // Green
            Object::Stick => Color::rgb(0.5, 0.3, 0.1),         // Brown
        }
    }

    /// Convert macroquad key to our Key enum
    fn mq_key_to_key(mq_key: macroquad::prelude::KeyCode) -> Option<Key> {
        match mq_key {
            macroquad::prelude::KeyCode::Up => Some(Key::Up),
            macroquad::prelude::KeyCode::Down => Some(Key::Down),
            macroquad::prelude::KeyCode::Left => Some(Key::Left),
            macroquad::prelude::KeyCode::Right => Some(Key::Right),
            macroquad::prelude::KeyCode::Space => Some(Key::Space),
            macroquad::prelude::KeyCode::Enter => Some(Key::Enter),
            macroquad::prelude::KeyCode::Escape => Some(Key::Escape),
            macroquad::prelude::KeyCode::Q => Some(Key::Q),
            macroquad::prelude::KeyCode::W => Some(Key::W),
            macroquad::prelude::KeyCode::A => Some(Key::A),
            macroquad::prelude::KeyCode::S => Some(Key::S),
            macroquad::prelude::KeyCode::D => Some(Key::D),
            macroquad::prelude::KeyCode::Z => Some(Key::Z),
            macroquad::prelude::KeyCode::X => Some(Key::X),
            _ => None,
        }
    }
}

impl Default for MacroquadRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for MacroquadRenderer {
    fn init(&mut self) -> Result<(), RenderError> {
        // Macroquad handles initialization automatically
        // Window is created when macroquad::window::request_new_screen_size is called
        // or when the first frame is rendered
        self.initialized = true;
        Ok(())
    }

    fn clear(&mut self, color: Color) {
        clear_background(Self::to_mq_color(color));
    }

    fn draw_tile(&mut self, x: f32, y: f32, size: f32, substrate: &Substrate, objects: &[Object]) {
        // Draw substrate as base rectangle
        let substrate_color = Self::substrate_color(substrate);
        draw_rectangle(x, y, size, size, Self::to_mq_color(substrate_color));

        // Draw objects on top (smaller rectangles or circles)
        if !objects.is_empty() {
            // Draw first object (or indicator for multiple)
            if objects.len() == 1 {
                let obj_color = Self::object_color(&objects[0]);
                // Draw object as a smaller rectangle in the center
                let obj_size = size * 0.6;
                let obj_x = x + (size - obj_size) / 2.0;
                let obj_y = y + (size - obj_size) / 2.0;
                draw_rectangle(obj_x, obj_y, obj_size, obj_size, Self::to_mq_color(obj_color));
            } else {
                // Multiple objects - draw indicator
                let indicator_color = Color::rgb(0.9, 0.1, 0.1); // Red for multiple
                let obj_size = size * 0.4;
                let obj_x = x + (size - obj_size) / 2.0;
                let obj_y = y + (size - obj_size) / 2.0;
                draw_rectangle(obj_x, obj_y, obj_size, obj_size, Self::to_mq_color(indicator_color));
            }
        }

        // Draw border
        draw_rectangle_lines(x, y, size, size, 1.0, Self::to_mq_color(Color::rgb(0.1, 0.1, 0.1)));
    }

    fn draw_biome_overview(&mut self, x: f32, y: f32, size: f32, biome: &Biome) {
        let biome_color = Self::biome_color(biome);
        draw_rectangle(x, y, size, size, Self::to_mq_color(biome_color));
        
        // Draw subtle border
        draw_rectangle_lines(x, y, size, size, 0.5, Self::to_mq_color(Color::rgb(0.2, 0.2, 0.2)));
    }

    fn draw_biome_overview_with_borders(
        &mut self,
        x: f32,
        y: f32,
        size: f32,
        center: &Biome,
        top: &Biome,
        bottom: &Biome,
        left: &Biome,
        right: &Biome,
        top_left: &Biome,
        top_right: &Biome,
        bottom_left: &Biome,
        bottom_right: &Biome,
        border_width: f32,
    ) {
        // Draw center biome as the main rectangle
        let center_color = Self::biome_color(center);
        draw_rectangle(x, y, size, size, Self::to_mq_color(center_color));
        
        // Apply natural shadow effect to border colors (darker, blue-shifted)
        // This makes grid lines distinguishable while showing biome transitions
        
        // Draw borders using edge biome colors with shadow effect
        // Top edge
        let top_color = Self::shadow_color(Self::biome_color(top));
        draw_rectangle(x, y, size, border_width, Self::to_mq_color(top_color));
        
        // Bottom edge
        let bottom_color = Self::shadow_color(Self::biome_color(bottom));
        draw_rectangle(x, y + size - border_width, size, border_width, Self::to_mq_color(bottom_color));
        
        // Left edge
        let left_color = Self::shadow_color(Self::biome_color(left));
        draw_rectangle(x, y, border_width, size, Self::to_mq_color(left_color));
        
        // Right edge
        let right_color = Self::shadow_color(Self::biome_color(right));
        draw_rectangle(x + size - border_width, y, border_width, size, Self::to_mq_color(right_color));
        
        // Draw corners using corner biome colors with shadow effect (overlay on top of edges)
        let top_left_color = Self::shadow_color(Self::biome_color(top_left));
        draw_rectangle(x, y, border_width, border_width, Self::to_mq_color(top_left_color));
        
        let top_right_color = Self::shadow_color(Self::biome_color(top_right));
        draw_rectangle(x + size - border_width, y, border_width, border_width, Self::to_mq_color(top_right_color));
        
        let bottom_left_color = Self::shadow_color(Self::biome_color(bottom_left));
        draw_rectangle(x, y + size - border_width, border_width, border_width, Self::to_mq_color(bottom_left_color));
        
        let bottom_right_color = Self::shadow_color(Self::biome_color(bottom_right));
        draw_rectangle(x + size - border_width, y + size - border_width, border_width, border_width, Self::to_mq_color(bottom_right_color));
    }

    fn draw_selection_indicator(&mut self, x: f32, y: f32, size: f32) {
        // Draw a bright yellow/orange border to indicate selection
        let indicator_color = Color::rgb(1.0, 0.8, 0.0); // Bright yellow-orange
        let border_width = 3.0;
        
        // Draw border lines (thicker than normal)
        draw_rectangle_lines(x, y, size, size, border_width, Self::to_mq_color(indicator_color));
        
        // Draw corner indicators for extra visibility
        let corner_size = size * 0.15;
        let corner_thickness = 2.0;
        
        // Top-left corner
        draw_line(x, y, x + corner_size, y, corner_thickness, Self::to_mq_color(indicator_color));
        draw_line(x, y, x, y + corner_size, corner_thickness, Self::to_mq_color(indicator_color));
        
        // Top-right corner
        draw_line(x + size - corner_size, y, x + size, y, corner_thickness, Self::to_mq_color(indicator_color));
        draw_line(x + size, y, x + size, y + corner_size, corner_thickness, Self::to_mq_color(indicator_color));
        
        // Bottom-left corner
        draw_line(x, y + size - corner_size, x, y + size, corner_thickness, Self::to_mq_color(indicator_color));
        draw_line(x, y + size, x + corner_size, y + size, corner_thickness, Self::to_mq_color(indicator_color));
        
        // Bottom-right corner
        draw_line(x + size - corner_size, y + size, x + size, y + size, corner_thickness, Self::to_mq_color(indicator_color));
        draw_line(x + size, y + size - corner_size, x + size, y + size, corner_thickness, Self::to_mq_color(indicator_color));
    }

    fn draw_grid(&mut self, x: f32, y: f32, width: f32, height: f32, rows: usize, cols: usize) {
        let grid_color = Color::rgb(0.3, 0.3, 0.3); // Dark gray grid lines
        let line_width = 1.0;
        
        let cell_width = width / cols as f32;
        let cell_height = height / rows as f32;
        
        // Draw vertical lines
        for i in 0..=cols {
            let line_x = x + i as f32 * cell_width;
            draw_line(
                line_x, y,
                line_x, y + height,
                line_width,
                Self::to_mq_color(grid_color)
            );
        }
        
        // Draw horizontal lines
        for i in 0..=rows {
            let line_y = y + i as f32 * cell_height;
            draw_line(
                x, line_y,
                x + width, line_y,
                line_width,
                Self::to_mq_color(grid_color)
            );
        }
    }

    fn present(&mut self) -> Result<(), RenderError> {
        // Macroquad handles presentation automatically after each frame
        // next_frame() is called in the main loop, not here
        Ok(())
    }

    fn should_close(&self) -> bool {
        // Check if escape is pressed
        // Note: In macroquad, the window close is handled automatically
        is_key_pressed(macroquad::prelude::KeyCode::Escape)
    }

    fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        Some((mouse_position().0, mouse_position().1))
    }

    fn get_keys_pressed(&self) -> Vec<Key> {
        let mut keys = Vec::new();
        
        // Check all relevant keys (using is_key_down for continuous input)
        let key_codes = [
            macroquad::prelude::KeyCode::Up,
            macroquad::prelude::KeyCode::Down,
            macroquad::prelude::KeyCode::Left,
            macroquad::prelude::KeyCode::Right,
            macroquad::prelude::KeyCode::Space,
            macroquad::prelude::KeyCode::Enter,
            macroquad::prelude::KeyCode::Escape,
            macroquad::prelude::KeyCode::Q,
            macroquad::prelude::KeyCode::W,
            macroquad::prelude::KeyCode::A,
            macroquad::prelude::KeyCode::S,
            macroquad::prelude::KeyCode::D,
            macroquad::prelude::KeyCode::Z,
            macroquad::prelude::KeyCode::X,
        ];

        for key_code in key_codes {
            if is_key_down(key_code) {
                if let Some(key) = Self::mq_key_to_key(key_code) {
                    keys.push(key);
                }
            }
        }

        keys
    }

    fn window_size(&self) -> (f32, f32) {
        (screen_width(), screen_height())
    }
}
