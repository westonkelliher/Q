/// 8x8 subsystem GUI application

use macroquad::prelude::*;
use eight_by_eight::{Color as GridColor, Grid8x8};

/// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "8x8 Grid Viewer".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

struct GridGUI {
    grid: Grid8x8,
    hovered_tile: Option<(usize, usize)>,
}

impl GridGUI {
    fn new(grid: Grid8x8) -> Self {
        Self {
            grid,
            hovered_tile: None,
        }
    }

    fn update(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        
        // Calculate grid position
        let screen_width = screen_width();
        let screen_height = screen_height();
        let padding = 50.0;
        let grid_size = (screen_width.min(screen_height) - padding * 2.0).min(600.0);
        let tile_size = grid_size / 8.0;
        let grid_start_x = (screen_width - grid_size) / 2.0;
        let grid_start_y = (screen_height - grid_size) / 2.0;
        
        // Check which tile is hovered
        self.hovered_tile = None;
        if mouse_x >= grid_start_x && mouse_x < grid_start_x + grid_size &&
           mouse_y >= grid_start_y && mouse_y < grid_start_y + grid_size {
            let x = ((mouse_x - grid_start_x) / tile_size) as usize;
            let y = ((mouse_y - grid_start_y) / tile_size) as usize;
            if x < 8 && y < 8 {
                self.hovered_tile = Some((x, y));
            }
        }
    }

    fn draw(&self) {
        // Light background similar to combat subsystem
        clear_background(Color::new(0.95, 0.97, 1.0, 1.0));

        let screen_width = screen_width();
        let screen_height = screen_height();
        let padding = 50.0;
        let grid_size = (screen_width.min(screen_height) - padding * 2.0).min(600.0);
        let tile_size = grid_size / 8.0;
        let grid_start_x = (screen_width - grid_size) / 2.0;
        let grid_start_y = (screen_height - grid_size) / 2.0;

        // Draw title
        let title_text = "8x8 Grid Viewer";
        let title_size = 32.0;
        let title_width = measure_text(title_text, None, title_size as u16, 1.0).width;
        draw_text(
            title_text,
            (screen_width - title_width) / 2.0,
            30.0,
            title_size,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );

        // Draw grid background
        self.draw_rounded_rect(
            grid_start_x - 5.0,
            grid_start_y - 5.0,
            grid_size + 10.0,
            grid_size + 10.0,
            10.0,
            Color::new(0.98, 0.99, 1.0, 1.0),
        );
        draw_rectangle_lines(
            grid_start_x - 5.0,
            grid_start_y - 5.0,
            grid_size + 10.0,
            grid_size + 10.0,
            2.0,
            Color::new(0.8, 0.8, 0.9, 1.0),
        );

        // Draw tiles
        for y in 0..8 {
            for x in 0..8 {
                if let Some(tile) = self.grid.get(x, y) {
                    let tile_x = grid_start_x + x as f32 * tile_size;
                    let tile_y = grid_start_y + y as f32 * tile_size;

                    // Convert our Color to macroquad Color
                    let tile_color = self.color_to_macroquad(tile.color);
                    
                    // Highlight hovered tile
                    let is_hovered = self.hovered_tile == Some((x, y));
                    let draw_color = if is_hovered {
                        Color::new(
                            (tile_color.r * 1.2).min(1.0),
                            (tile_color.g * 1.2).min(1.0),
                            (tile_color.b * 1.2).min(1.0),
                            tile_color.a,
                        )
                    } else {
                        tile_color
                    };

                    // Draw tile background
                    draw_rectangle(tile_x, tile_y, tile_size, tile_size, draw_color);
                    
                    // Draw border
                    let border_color = if is_hovered {
                        Color::new(0.0, 0.0, 0.0, 0.5)
                    } else {
                        Color::new(0.0, 0.0, 0.0, 0.2)
                    };
                    draw_rectangle_lines(tile_x, tile_y, tile_size, tile_size, 1.0, border_color);

                    // Draw strings on the tile
                    if !tile.strings.is_empty() {
                        let text_color = if self.is_dark_color(tile.color) {
                            WHITE
                        } else {
                            BLACK
                        };

                        // Display strings, one per line
                        let mut y_offset = 5.0;
                        let font_size = (tile_size * 0.15).max(8.0);
                        for string in tile.strings.iter().take(3) {
                            // Limit to 3 strings to avoid overcrowding
                            let text = if string.len() > 10 {
                                format!("{}...", &string[..10])
                            } else {
                                string.clone()
                            };
                            
                            draw_text(
                                &text,
                                tile_x + 3.0,
                                tile_y + y_offset,
                                font_size,
                                text_color,
                            );
                            
                            y_offset += font_size + 2.0;
                        }
                        
                        // Show count if there are more strings
                        if tile.strings.len() > 3 {
                            draw_text(
                                &format!("+{} more", tile.strings.len() - 3),
                                tile_x + 3.0,
                                tile_y + y_offset,
                                font_size * 0.8,
                                text_color,
                            );
                        }
                    }
                }
            }
        }

        // Draw hover tooltip
        if let Some((x, y)) = self.hovered_tile {
            if let Some(tile) = self.grid.get(x, y) {
                let tooltip_x = grid_start_x + grid_size + 20.0;
                let tooltip_y = grid_start_y;
                let tooltip_width = 200.0;
                let tooltip_padding = 10.0;

                // Tooltip background
                self.draw_rounded_rect(
                    tooltip_x,
                    tooltip_y,
                    tooltip_width,
                    150.0,
                    8.0,
                    Color::new(1.0, 1.0, 1.0, 0.95),
                );
                draw_rectangle_lines(
                    tooltip_x,
                    tooltip_y,
                    tooltip_width,
                    150.0,
                    2.0,
                    Color::new(0.0, 0.0, 0.0, 0.3),
                );

                // Tooltip content
                let mut text_y = tooltip_y + tooltip_padding + 20.0;
                draw_text(
                    &format!("Tile ({}, {})", x, y),
                    tooltip_x + tooltip_padding,
                    text_y,
                    16.0,
                    BLACK,
                );
                text_y += 25.0;
                
                draw_text(
                    &format!("Color: R={:.2}, G={:.2}, B={:.2}, A={:.2}",
                        tile.color.r, tile.color.g, tile.color.b, tile.color.a),
                    tooltip_x + tooltip_padding,
                    text_y,
                    12.0,
                    Color::new(0.3, 0.3, 0.3, 1.0),
                );
                text_y += 20.0;

                if !tile.strings.is_empty() {
                    draw_text(
                        "Strings:",
                        tooltip_x + tooltip_padding,
                        text_y,
                        12.0,
                        Color::new(0.3, 0.3, 0.3, 1.0),
                    );
                    text_y += 18.0;
                    for string in &tile.strings {
                        draw_text(
                            &format!("  • {}", string),
                            tooltip_x + tooltip_padding,
                            text_y,
                            11.0,
                            Color::new(0.4, 0.4, 0.4, 1.0),
                        );
                        text_y += 15.0;
                    }
                }
            }
        }

        // Draw grid info at bottom
        let info_text = format!("Grid size: {}x{}", self.grid.width(), self.grid.height());
        let info_size = 14.0;
        let info_width = measure_text(&info_text, None, info_size as u16, 1.0).width;
        draw_text(
            &info_text,
            (screen_width - info_width) / 2.0,
            screen_height - 30.0,
            info_size,
            Color::new(0.4, 0.4, 0.4, 1.0),
        );
    }

    fn draw_rounded_rect(&self, x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
        // Draw rounded rectangle using multiple rectangles and circles
        // Top and bottom rectangles
        draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
        // Left and right rectangles
        draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);
        // Four corner circles
        draw_circle(x + radius, y + radius, radius, color);
        draw_circle(x + width - radius, y + radius, radius, color);
        draw_circle(x + radius, y + height - radius, radius, color);
        draw_circle(x + width - radius, y + height - radius, radius, color);
    }

    /// Convert our Color to macroquad Color
    fn color_to_macroquad(&self, color: GridColor) -> Color {
        Color::new(color.r, color.g, color.b, color.a)
    }

    /// Determine if a color is dark (for choosing text color)
    fn is_dark_color(&self, color: GridColor) -> bool {
        // Calculate luminance
        let luminance = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
        luminance < 0.5
    }
}

fn create_example_grid() -> Grid8x8 {
    let default_color = GridColor::rgb(0.5, 0.5, 0.5);
    let mut grid = Grid8x8::new(default_color);
    
    // Example: Set some tiles with different colors
    grid.set_color(0, 0, GridColor::rgb(1.0, 0.0, 0.0)); // Red
    grid.set_color(7, 7, GridColor::rgb(0.0, 1.0, 0.0)); // Green
    grid.set_color(3, 4, GridColor::rgb(0.0, 0.0, 1.0)); // Blue
    grid.set_color(1, 1, GridColor::rgb(1.0, 1.0, 0.0)); // Yellow
    grid.set_color(6, 2, GridColor::rgb(1.0, 0.0, 1.0)); // Magenta
    grid.set_color(2, 6, GridColor::rgb(0.0, 1.0, 1.0)); // Cyan
    
    // Example: Add some strings to tiles
    grid.add_string(0, 0, "top-left".to_string());
    grid.add_string(7, 7, "bottom-right".to_string());
    grid.add_string(3, 4, "center".to_string());
    grid.add_string(3, 4, "blue tile".to_string());
    grid.add_string(1, 1, "yellow".to_string());
    grid.add_string(6, 2, "magenta".to_string());
    grid.add_string(2, 6, "cyan".to_string());
    
    grid
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid = create_example_grid();
    let mut gui = GridGUI::new(grid);

    loop {
        gui.update();
        gui.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
