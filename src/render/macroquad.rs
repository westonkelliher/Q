use macroquad::prelude::*;
use crate::render::{Color, Key, RenderError, Renderer};
use crate::render::textures::{load_png_from_bytes, get_object_png};
use crate::types::{Biome, Object, Substrate};
use std::collections::HashMap;

/// Macroquad-based implementation of the Renderer trait
pub struct MacroquadRenderer {
    initialized: bool,
    textures: HashMap<String, Texture2D>,
}

impl MacroquadRenderer {
    pub fn new() -> Self {
        let mut textures = HashMap::new();
        
        // Try to load embedded PNG textures
        for obj_name in ["rock", "tree", "stick"] {
            if let Some(png_data) = get_object_png(obj_name) {
                match load_png_from_bytes(png_data) {
                    Ok(texture) => {
                        textures.insert(obj_name.to_string(), texture);
                        println!("Loaded texture: {}", obj_name);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load {} texture: {}. Using geometric fallback.", obj_name, e);
                    }
                }
            }
        }
        
        Self {
            initialized: false,
            textures,
        }
    }
    
    /// Get a texture for an object
    fn get_texture(&self, object: &Object) -> Option<&Texture2D> {
        let name = match object {
            Object::Rock => "rock",
            Object::Tree => "tree",
            Object::Stick => "stick",
        };
        self.textures.get(name)
    }

    /// Convert our Color type to macroquad's Color
    fn to_mq_color(color: Color) -> macroquad::prelude::Color {
        macroquad::prelude::Color::new(color.r, color.g, color.b, color.a)
    }

    /// Convert substrate to color
    fn substrate_color(substrate: &Substrate) -> Color {
        match substrate {
            Substrate::Grass => Color::rgb(0.7, 0.9, 0.4),      // Light green/yellow (same as meadow)
            Substrate::Dirt => Color::rgb(0.6, 0.4, 0.2),       // Brown
            Substrate::Stone => Color::rgb(0.7, 0.7, 0.7),      // Gray
            Substrate::Mud => Color::rgb(0.4, 0.3, 0.2),        // Dark brown
            Substrate::Water => Color::rgb(0.2, 0.4, 0.9),      // Blue
            Substrate::Brush => Color::rgb(0.2, 0.6, 0.15),    // Dark green, similar to forest
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

        // Draw objects on top
        if !objects.is_empty() {
            // Show first object (for single or multiple objects)
            let object = &objects[0];
            let center_x = x + size / 2.0;
            let center_y = y + size / 2.0;
            let obj_size = size * 0.75; // 50% larger than before (was 0.5, now 0.75)
            
            // Try to use texture first, fallback to geometric rendering
            if let Some(texture) = self.get_texture(object) {
                let texture_x = center_x - obj_size / 2.0;
                let texture_y = center_y - obj_size / 2.0;
                draw_texture_ex(
                    texture,
                    texture_x,
                    texture_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(obj_size, obj_size)),
                        ..Default::default()
                    },
                );
            } else {
                // Fallback to geometric rendering
                match object {
                Object::Rock => {
                    // Draw rock in Super Auto Pets style (from rock.svg)
                    let scale = obj_size / 100.0;
                    let offset_x = center_x - obj_size / 2.0;
                    let offset_y = center_y - obj_size / 2.0;
                    
                    // Helper to convert SVG coords to screen coords
                    let sx = |x: f32| offset_x + x * scale;
                    let sy = |y: f32| offset_y + y * scale;
                    
                    // Rock colors
                    let rock_base = Color::rgb(0.38, 0.38, 0.38);      // #606060
                    let rock_highlight = Color::rgb(0.50, 0.50, 0.50); // #808080
                    let rock_shadow = Color::rgb(0.25, 0.25, 0.25);    // #404040
                    let black = Color::rgb(0.0, 0.0, 0.0);
                    
                    // Draw main rock body (irregular shape with overlapping ellipses)
                    draw_ellipse(sx(50.0), sy(55.0), 35.0 * scale, 30.0 * scale, 0.0, Self::to_mq_color(rock_base));
                    draw_ellipse_lines(sx(50.0), sy(55.0), 35.0 * scale, 30.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                    
                    draw_ellipse(sx(35.0), sy(50.0), 18.0 * scale, 20.0 * scale, 0.0, Self::to_mq_color(rock_base));
                    draw_ellipse_lines(sx(35.0), sy(50.0), 18.0 * scale, 20.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                    
                    draw_ellipse(sx(65.0), sy(52.0), 15.0 * scale, 18.0 * scale, 0.0, Self::to_mq_color(rock_base));
                    draw_ellipse_lines(sx(65.0), sy(52.0), 15.0 * scale, 18.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                    
                    draw_ellipse(sx(50.0), sy(35.0), 20.0 * scale, 18.0 * scale, 0.0, Self::to_mq_color(rock_base));
                    draw_ellipse_lines(sx(50.0), sy(35.0), 20.0 * scale, 18.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                    
                    // Highlights (top left areas)
                    draw_ellipse(sx(40.0), sy(40.0), 15.0 * scale, 12.0 * scale, 0.0, Self::to_mq_color(rock_highlight));
                    draw_ellipse(sx(32.0), sy(48.0), 8.0 * scale, 10.0 * scale, 0.0, Self::to_mq_color(rock_highlight));
                    
                    // Shadows (bottom right areas)
                    draw_ellipse(sx(58.0), sy(60.0), 18.0 * scale, 15.0 * scale, 0.0, Self::to_mq_color(rock_shadow));
                    draw_ellipse(sx(65.0), sy(55.0), 10.0 * scale, 12.0 * scale, 0.0, Self::to_mq_color(rock_shadow));
                }
                Object::Tree => {
                    // Draw tree in Super Auto Pets style (from tree.svg)
                    // Scale SVG coordinates (0-100) to fit in tile
                    let scale = obj_size / 100.0;
                    let offset_x = center_x - obj_size / 2.0;
                    let offset_y = center_y - obj_size / 2.0;
                    
                    // Helper to convert SVG coords to screen coords
                    let sx = |x: f32| offset_x + x * scale;
                    let sy = |y: f32| offset_y + y * scale;
                    
                    // Trunk colors
                    let trunk_base = Color::rgb(0.36, 0.25, 0.20);   // #5C4033
                    let trunk_highlight = Color::rgb(0.42, 0.30, 0.23); // #6B4D3B
                    let trunk_shadow = Color::rgb(0.29, 0.20, 0.16);    // #4A3329
                    
                    // Foliage colors
                    let foliage_base = Color::rgb(0.18, 0.31, 0.09);    // #2D5016
                    let foliage_highlight = Color::rgb(0.23, 0.42, 0.12); // #3A6B1E
                    let foliage_shadow = Color::rgb(0.12, 0.23, 0.06);   // #1F3A0F
                    
                    // Draw trunk base
                    draw_rectangle(
                        sx(42.0), sy(55.0), 16.0 * scale, 30.0 * scale,
                        Self::to_mq_color(trunk_base)
                    );
                    draw_rectangle_lines(
                        sx(42.0), sy(55.0), 16.0 * scale, 30.0 * scale,
                        5.0 * scale, Self::to_mq_color(Color::rgb(0.0, 0.0, 0.0))
                    );
                    
                    // Trunk highlight (left side)
                    draw_rectangle(
                        sx(42.0), sy(55.0), 6.0 * scale, 30.0 * scale,
                        Self::to_mq_color(trunk_highlight)
                    );
                    
                    // Trunk shadow (right side)
                    draw_rectangle(
                        sx(52.0), sy(55.0), 6.0 * scale, 30.0 * scale,
                        Self::to_mq_color(trunk_shadow)
                    );
                    
                    // Main foliage circle
                    draw_circle(sx(50.0), sy(45.0), 28.0 * scale, Self::to_mq_color(foliage_base));
                    draw_circle_lines(sx(50.0), sy(45.0), 28.0 * scale, 5.0 * scale, Self::to_mq_color(Color::rgb(0.0, 0.0, 0.0)));
                    
                    // Foliage highlight (top left)
                    draw_circle(sx(42.0), sy(38.0), 15.0 * scale, Self::to_mq_color(foliage_highlight));
                    
                    // Foliage shadow (bottom right)
                    draw_circle(sx(58.0), sy(52.0), 12.0 * scale, Self::to_mq_color(foliage_shadow));
                    
                    // Small foliage details
                    draw_circle(sx(35.0), sy(50.0), 12.0 * scale, Self::to_mq_color(foliage_base));
                    draw_circle_lines(sx(35.0), sy(50.0), 12.0 * scale, 5.0 * scale, Self::to_mq_color(Color::rgb(0.0, 0.0, 0.0)));
                    
                    draw_circle(sx(65.0), sy(48.0), 10.0 * scale, Self::to_mq_color(foliage_base));
                    draw_circle_lines(sx(65.0), sy(48.0), 10.0 * scale, 5.0 * scale, Self::to_mq_color(Color::rgb(0.0, 0.0, 0.0)));
                    
                    draw_circle(sx(50.0), sy(25.0), 14.0 * scale, Self::to_mq_color(foliage_base));
                    draw_circle_lines(sx(50.0), sy(25.0), 14.0 * scale, 5.0 * scale, Self::to_mq_color(Color::rgb(0.0, 0.0, 0.0)));
                    
                    // Highlight on top foliage
                    draw_circle(sx(48.0), sy(23.0), 6.0 * scale, Self::to_mq_color(foliage_highlight));
                }
                Object::Stick => {
                    // Draw stick in Super Auto Pets style (from stick.svg)
                    let scale = obj_size / 100.0;
                    let offset_x = center_x - obj_size / 2.0;
                    let offset_y = center_y - obj_size / 2.0;
                    
                    // Helper to convert SVG coords to screen coords
                    let sx = |x: f32| offset_x + x * scale;
                    let sy = |y: f32| offset_y + y * scale;
                    
                    // Stick colors
                    let stick_base = Color::rgb(0.55, 0.44, 0.28);      // #8B6F47
                    let stick_highlight = Color::rgb(0.65, 0.54, 0.37); // #A68A5E
                    let stick_shadow = Color::rgb(0.42, 0.33, 0.22);    // #6B5537
                    let black = Color::rgb(0.0, 0.0, 0.0);
                    
                    // Draw stick as angled line with thickness (25 degree rotation)
                    let angle = 25.0_f32.to_radians();
                    let stick_length = 70.0 * scale;
                    let stick_width = 10.0 * scale;
                    
                    // Calculate stick endpoints (center at 35, 50 in SVG coords)
                    let stick_center_x = sx(45.0);
                    let stick_center_y = sy(50.0);
                    
                    let half_len = stick_length / 2.0;
                    let start_x = stick_center_x - angle.cos() * half_len;
                    let start_y = stick_center_y - angle.sin() * half_len;
                    let end_x = stick_center_x + angle.cos() * half_len;
                    let end_y = stick_center_y + angle.sin() * half_len;
                    
                    // Draw as thick line segments for base, highlight, and shadow
                    // Base stick
                    draw_line(start_x, start_y, end_x, end_y, stick_width, Self::to_mq_color(stick_base));
                    
                    // Highlight (left side) - draw thinner line offset to the left
                    let offset_perp = stick_width * 0.25;
                    let perp_x = -angle.sin();
                    let perp_y = angle.cos();
                    draw_line(
                        start_x + perp_x * offset_perp, 
                        start_y + perp_y * offset_perp,
                        end_x + perp_x * offset_perp, 
                        end_y + perp_y * offset_perp,
                        stick_width * 0.35,
                        Self::to_mq_color(stick_highlight)
                    );
                    
                    // Shadow (right side)
                    draw_line(
                        start_x - perp_x * offset_perp, 
                        start_y - perp_y * offset_perp,
                        end_x - perp_x * offset_perp, 
                        end_y - perp_y * offset_perp,
                        stick_width * 0.35,
                        Self::to_mq_color(stick_shadow)
                    );
                    
                    // Draw outline by drawing thin black lines along edges
                    draw_line(start_x, start_y, end_x, end_y, stick_width + 5.0 * scale, Self::to_mq_color(black));
                    draw_line(start_x, start_y, end_x, end_y, stick_width, Self::to_mq_color(stick_base));
                    
                    // Re-draw highlights and shadows on top
                    draw_line(
                        start_x + perp_x * offset_perp, 
                        start_y + perp_y * offset_perp,
                        end_x + perp_x * offset_perp, 
                        end_y + perp_y * offset_perp,
                        stick_width * 0.35,
                        Self::to_mq_color(stick_highlight)
                    );
                    draw_line(
                        start_x - perp_x * offset_perp, 
                        start_y - perp_y * offset_perp,
                        end_x - perp_x * offset_perp, 
                        end_y - perp_y * offset_perp,
                        stick_width * 0.35,
                        Self::to_mq_color(stick_shadow)
                    );
                    
                    // Small knot details
                    draw_ellipse(sx(45.0), sy(35.0), 4.0 * scale, 3.0 * scale, 0.0, Self::to_mq_color(stick_shadow));
                    draw_ellipse_lines(sx(45.0), sy(35.0), 4.0 * scale, 3.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                    
                    draw_ellipse(sx(52.0), sy(60.0), 3.5 * scale, 3.0 * scale, 0.0, Self::to_mq_color(stick_shadow));
                    draw_ellipse_lines(sx(52.0), sy(60.0), 3.5 * scale, 3.0 * scale, 0.0, 5.0 * scale, Self::to_mq_color(black));
                }
            }
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

    fn draw_button(&mut self, x: f32, y: f32, width: f32, height: f32, text: &str, is_pressed: bool) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        // Button background color (darker when pressed/hovered)
        let bg_color = if is_pressed {
            Color::rgb(0.3, 0.5, 0.3) // Green when toggled on
        } else if is_hovered {
            Color::rgb(0.4, 0.4, 0.4) // Gray when hovered
        } else {
            Color::rgb(0.3, 0.3, 0.3) // Dark gray default
        };

        // Draw button background
        draw_rectangle(x, y, width, height, Self::to_mq_color(bg_color));
        
        // Draw button border
        let border_color = if is_hovered {
            Color::rgb(0.7, 0.7, 0.7)
        } else {
            Color::rgb(0.5, 0.5, 0.5)
        };
        draw_rectangle_lines(x, y, width, height, 2.0, Self::to_mq_color(border_color));

        // Draw button text (centered)
        let text_size = 16.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let text_height = text_size;
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height + text_height) / 2.0 - 2.0; // Slight offset for better centering
        
        draw_text(text, text_x, text_y, text_size, Self::to_mq_color(Color::rgb(1.0, 1.0, 1.0)));

        is_clicked
    }
}
