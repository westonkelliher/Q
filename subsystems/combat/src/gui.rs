use combat::{Combatant, CombatState, CombatResult};
use macroquad::prelude::*;

/// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Combat System - Super Auto Pets Style".to_owned(),
        window_width: 1200,
        window_height: 800,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PredefinedCombatant {
    Tank,
    GlassCannon,
    Balanced,
    Bruiser,
    Assassin,
    Defender,
}

impl PredefinedCombatant {
    fn to_combatant(&self) -> Combatant {
        match self {
            PredefinedCombatant::Tank => Combatant::TANK,
            PredefinedCombatant::GlassCannon => Combatant::GLASS_CANNON,
            PredefinedCombatant::Balanced => Combatant::BALANCED,
            PredefinedCombatant::Bruiser => Combatant::BRUISER,
            PredefinedCombatant::Assassin => Combatant::ASSASSIN,
            PredefinedCombatant::Defender => Combatant::DEFENDER,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            PredefinedCombatant::Tank => "Tank",
            PredefinedCombatant::GlassCannon => "Glass Cannon",
            PredefinedCombatant::Balanced => "Balanced",
            PredefinedCombatant::Bruiser => "Bruiser",
            PredefinedCombatant::Assassin => "Assassin",
            PredefinedCombatant::Defender => "Defender",
        }
    }

    fn color(&self) -> Color {
        match self {
            PredefinedCombatant::Tank => Color::new(0.4, 0.6, 0.9, 1.0),      // Blue
            PredefinedCombatant::GlassCannon => Color::new(0.9, 0.3, 0.3, 1.0), // Red
            PredefinedCombatant::Balanced => Color::new(0.5, 0.8, 0.5, 1.0),   // Green
            PredefinedCombatant::Bruiser => Color::new(0.8, 0.5, 0.2, 1.0),     // Orange
            PredefinedCombatant::Assassin => Color::new(0.6, 0.2, 0.6, 1.0),    // Purple
            PredefinedCombatant::Defender => Color::new(0.3, 0.7, 0.8, 1.0),    // Cyan
        }
    }
}

struct CombatGUI {
    // Combatant 1
    side1_custom: bool,
    side1_health: String,
    side1_attack: String,
    side1_preset: PredefinedCombatant,
    
    // Combatant 2
    side2_custom: bool,
    side2_health: String,
    side2_attack: String,
    side2_preset: PredefinedCombatant,
    
    // Combat state
    combat_state: Option<CombatState>,
    combat_history: Vec<(u32, i32, i32, i32, i32, CombatResult)>,
    current_round: usize,
    auto_play: bool,
    auto_play_timer: f32,
    
    // UI state
    selected_input: Option<InputField>,
    
    // Animation
    attack_animation_timer: f32,
    last_attack_round: u32,
}

#[derive(Clone, Copy, PartialEq)]
enum InputField {
    Side1Health,
    Side1Attack,
    Side2Health,
    Side2Attack,
}

impl CombatGUI {
    fn new() -> Self {
        Self {
            side1_custom: false,
            side1_health: "10".to_string(),
            side1_attack: "5".to_string(),
            side1_preset: PredefinedCombatant::Balanced,
            
            side2_custom: false,
            side2_health: "10".to_string(),
            side2_attack: "5".to_string(),
            side2_preset: PredefinedCombatant::Balanced,
            
            combat_state: None,
            combat_history: Vec::new(),
            current_round: 0,
            auto_play: false,
            auto_play_timer: 0.0,
            
            selected_input: None,
            attack_animation_timer: 0.0,
            last_attack_round: 0,
        }
    }

    fn get_combatant1(&self) -> Combatant {
        if self.side1_custom {
            let health = self.side1_health.parse().unwrap_or(10);
            let attack = self.side1_attack.parse().unwrap_or(5);
            Combatant::new(health.max(1), attack.max(1))
        } else {
            self.side1_preset.to_combatant()
        }
    }

    fn get_combatant2(&self) -> Combatant {
        if self.side2_custom {
            let health = self.side2_health.parse().unwrap_or(10);
            let attack = self.side2_attack.parse().unwrap_or(5);
            Combatant::new(health.max(1), attack.max(1))
        } else {
            self.side2_preset.to_combatant()
        }
    }

    fn get_pet_color1(&self) -> Color {
        if self.side1_custom {
            Color::new(0.7, 0.7, 0.7, 1.0) // Gray for custom
        } else {
            self.side1_preset.color()
        }
    }

    fn get_pet_color2(&self) -> Color {
        if self.side2_custom {
            Color::new(0.7, 0.7, 0.7, 1.0) // Gray for custom
        } else {
            self.side2_preset.color()
        }
    }

    fn start_combat(&mut self) {
        let c1 = self.get_combatant1();
        let c2 = self.get_combatant2();
        self.combat_state = Some(CombatState::new(c1, c2));
        self.combat_history.clear();
        self.current_round = 0;
        self.auto_play = false;
        self.attack_animation_timer = 0.0;
        self.last_attack_round = 0;
    }

    fn execute_round(&mut self) {
        if let Some(ref mut state) = self.combat_state {
            let round_before = state.round;
            let health1_before = state.combatant1.health;
            let health2_before = state.combatant2.health;
            
            let result = state.execute_round();
            
            self.combat_history.push((
                round_before + 1,
                health1_before,
                health2_before,
                state.combatant1.health,
                state.combatant2.health,
                result,
            ));
            self.current_round = self.combat_history.len();
            self.last_attack_round = state.round;
            self.attack_animation_timer = 0.3; // 0.3 seconds animation
        }
    }

    fn reset_combat(&mut self) {
        self.combat_state = None;
        self.combat_history.clear();
        self.current_round = 0;
        self.auto_play = false;
        self.attack_animation_timer = 0.0;
        self.last_attack_round = 0;
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

    fn draw_pet(&self, x: f32, y: f32, size: f32, color: Color, health: i32, max_health: i32, attack: i32, is_defeated: bool) {
        let pet_size = size * 0.8;
        let center_x = x + size / 2.0;
        let center_y = y + size / 2.0;
        
        // Pet body (rounded circle)
        let body_color = if is_defeated {
            Color::new(color.r * 0.3, color.g * 0.3, color.b * 0.3, 1.0)
        } else {
            color
        };
        
        // Shadow
        draw_circle(center_x + 3.0, center_y + 3.0, pet_size * 0.5, Color::new(0.0, 0.0, 0.0, 0.2));
        
        // Main body
        draw_circle(center_x, center_y, pet_size * 0.5, body_color);
        draw_circle_lines(center_x, center_y, pet_size * 0.5, 3.0, Color::new(0.0, 0.0, 0.0, 0.3));
        
        // Highlight
        draw_circle(center_x - pet_size * 0.15, center_y - pet_size * 0.15, pet_size * 0.2, Color::new(1.0, 1.0, 1.0, 0.3));
        
        if !is_defeated {
            // Eyes
            let eye_size = pet_size * 0.15;
            draw_circle(center_x - pet_size * 0.15, center_y - pet_size * 0.1, eye_size, WHITE);
            draw_circle(center_x + pet_size * 0.15, center_y - pet_size * 0.1, eye_size, WHITE);
            draw_circle(center_x - pet_size * 0.15, center_y - pet_size * 0.1, eye_size * 0.5, Color::new(0.0, 0.0, 0.0, 1.0));
            draw_circle(center_x + pet_size * 0.15, center_y - pet_size * 0.1, eye_size * 0.5, Color::new(0.0, 0.0, 0.0, 1.0));
            
            // Smile (drawn as a simple arc using lines)
            let smile_radius = pet_size * 0.2;
            let smile_center_y = center_y + pet_size * 0.1;
            let segments = 8;
            for i in 0..segments {
                let angle1 = (i as f32 / segments as f32) * std::f32::consts::PI;
                let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI;
                let x1 = center_x + angle1.cos() * smile_radius;
                let y1 = smile_center_y + angle1.sin() * smile_radius;
                let x2 = center_x + angle2.cos() * smile_radius;
                let y2 = smile_center_y + angle2.sin() * smile_radius;
                draw_line(x1, y1, x2, y2, 3.0, Color::new(0.0, 0.0, 0.0, 0.5));
            }
        } else {
            // X eyes for defeated
            let eye_x1 = center_x - pet_size * 0.15;
            let eye_y1 = center_y - pet_size * 0.1;
            let eye_x2 = center_x + pet_size * 0.15;
            let eye_y2 = center_y - pet_size * 0.1;
            let eye_size = pet_size * 0.1;
            draw_line(eye_x1 - eye_size, eye_y1 - eye_size, eye_x1 + eye_size, eye_y1 + eye_size, 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_line(eye_x1 - eye_size, eye_y1 + eye_size, eye_x1 + eye_size, eye_y1 - eye_size, 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_line(eye_x2 - eye_size, eye_y2 - eye_size, eye_x2 + eye_size, eye_y2 + eye_size, 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_line(eye_x2 - eye_size, eye_y2 + eye_size, eye_x2 + eye_size, eye_y2 - eye_size, 2.0, Color::new(0.0, 0.0, 0.0, 0.8));
        }
        
        // Health bar below pet
        let bar_width = size * 0.9;
        let bar_height = 12.0;
        let bar_x = center_x - bar_width / 2.0;
        let bar_y = y + size - 20.0;
        
        // Background
        self.draw_rounded_rect(bar_x, bar_y, bar_width, bar_height, 3.0, Color::new(0.2, 0.2, 0.2, 1.0));
        
        // Health fill
        let health_ratio = (health.max(0) as f32 / max_health.max(1) as f32).min(1.0);
        let health_color = if health_ratio > 0.6 {
            Color::new(0.2, 0.8, 0.3, 1.0) // Green
        } else if health_ratio > 0.3 {
            Color::new(0.9, 0.7, 0.2, 1.0) // Yellow
        } else {
            Color::new(0.9, 0.2, 0.2, 1.0) // Red
        };
        
        if health_ratio > 0.0 {
            self.draw_rounded_rect(bar_x + 2.0, bar_y + 2.0, (bar_width - 4.0) * health_ratio, bar_height - 4.0, 2.0, health_color);
        }
        
        // Health text
        let health_text = format!("{}", health.max(0));
        let text_size = 10.0;
        let text_width = measure_text(&health_text, None, text_size as u16, 1.0).width;
        draw_text(&health_text, bar_x + (bar_width - text_width) / 2.0, bar_y + bar_height - 2.0, text_size, WHITE);
        
        // Attack indicator (top right)
        let attack_x = x + size - 25.0;
        let attack_y = y + 5.0;
        self.draw_rounded_rect(attack_x, attack_y, 20.0, 20.0, 4.0, Color::new(0.9, 0.3, 0.2, 1.0));
        let attack_text = format!("{}", attack);
        let attack_text_size = 12.0;
        let attack_text_width = measure_text(&attack_text, None, attack_text_size as u16, 1.0).width;
        draw_text(&attack_text, attack_x + (20.0 - attack_text_width) / 2.0, attack_y + 14.0, attack_text_size, WHITE);
    }

    fn draw_button(&self, x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        let bg_color = if is_hovered {
            Color::new(
                (color.r * 1.2).min(1.0),
                (color.g * 1.2).min(1.0),
                (color.b * 1.2).min(1.0),
                1.0
            )
        } else {
            color
        };

        self.draw_rounded_rect(x, y, width, height, 8.0, bg_color);
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.0, 0.0, 0.0, 0.3));

        let text_size = 18.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height + text_size) / 2.0 - 2.0;
        
        draw_text(text, text_x, text_y, text_size, WHITE);

        is_clicked
    }

    fn draw_preset_button(&self, x: f32, y: f32, width: f32, height: f32, preset: PredefinedCombatant, is_selected: bool) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        let bg_color = if is_selected {
            preset.color()
        } else if is_hovered {
            Color::new(preset.color().r * 0.7, preset.color().g * 0.7, preset.color().b * 0.7, 1.0)
        } else {
            Color::new(preset.color().r * 0.5, preset.color().g * 0.5, preset.color().b * 0.5, 1.0)
        };

        self.draw_rounded_rect(x, y, width, height, 6.0, bg_color);
        if is_selected {
            draw_rectangle_lines(x, y, width, height, 3.0, Color::new(1.0, 1.0, 1.0, 0.8));
        } else {
            draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.0, 0.0, 0.0, 0.3));
        }

        let text_size = 14.0;
        let text_width = measure_text(preset.name(), None, text_size as u16, 1.0).width;
        draw_text(preset.name(), x + (width - text_width) / 2.0, y + (height + text_size) / 2.0 - 2.0, text_size, WHITE);

        is_clicked
    }

    fn draw_text_input(&mut self, x: f32, y: f32, width: f32, height: f32, text: String, field: InputField) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        if is_clicked {
            self.selected_input = Some(field);
        }

        let is_selected = self.selected_input == Some(field);
        let bg_color = if is_selected {
            Color::new(1.0, 1.0, 0.9, 1.0) // Light yellow when selected
        } else if is_hovered {
            Color::new(0.9, 0.9, 0.9, 1.0)
        } else {
            Color::new(0.95, 0.95, 0.95, 1.0)
        };

        self.draw_rounded_rect(x, y, width, height, 6.0, bg_color);
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));

        let text_size = 16.0;
        let display_text = if is_selected && (get_time() * 2.0) as i32 % 2 == 0 {
            format!("{}|", text)
        } else {
            text
        };
        let text_x = x + 8.0;
        let text_y = y + (height + text_size) / 2.0 - 2.0;
        
        draw_text(&display_text, text_x, text_y, text_size, Color::new(0.1, 0.1, 0.1, 1.0));

        is_clicked
    }

    fn handle_text_input(&mut self) {
        if let Some(field) = self.selected_input {
            if is_key_pressed(KeyCode::Backspace) {
                match field {
                    InputField::Side1Health => {
                        self.side1_health.pop();
                        if self.side1_health.is_empty() {
                            self.side1_health = "0".to_string();
                        }
                    }
                    InputField::Side1Attack => {
                        self.side1_attack.pop();
                        if self.side1_attack.is_empty() {
                            self.side1_attack = "0".to_string();
                        }
                    }
                    InputField::Side2Health => {
                        self.side2_health.pop();
                        if self.side2_health.is_empty() {
                            self.side2_health = "0".to_string();
                        }
                    }
                    InputField::Side2Attack => {
                        self.side2_attack.pop();
                        if self.side2_attack.is_empty() {
                            self.side2_attack = "0".to_string();
                        }
                    }
                }
            }

            let number_keys = [
                (KeyCode::Key0, '0'),
                (KeyCode::Key1, '1'),
                (KeyCode::Key2, '2'),
                (KeyCode::Key3, '3'),
                (KeyCode::Key4, '4'),
                (KeyCode::Key5, '5'),
                (KeyCode::Key6, '6'),
                (KeyCode::Key7, '7'),
                (KeyCode::Key8, '8'),
                (KeyCode::Key9, '9'),
            ];

            for (key_code, digit) in number_keys.iter() {
                if is_key_pressed(*key_code) {
                    match field {
                        InputField::Side1Health => {
                            if self.side1_health == "0" {
                                self.side1_health = digit.to_string();
                            } else {
                                self.side1_health.push(*digit);
                            }
                        }
                        InputField::Side1Attack => {
                            if self.side1_attack == "0" {
                                self.side1_attack = digit.to_string();
                            } else {
                                self.side1_attack.push(*digit);
                            }
                        }
                        InputField::Side2Health => {
                            if self.side2_health == "0" {
                                self.side2_health = digit.to_string();
                            } else {
                                self.side2_health.push(*digit);
                            }
                        }
                        InputField::Side2Attack => {
                            if self.side2_attack == "0" {
                                self.side2_attack = digit.to_string();
                            } else {
                                self.side2_attack.push(*digit);
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self, delta: f32) {
        self.handle_text_input();

        if self.attack_animation_timer > 0.0 {
            self.attack_animation_timer -= delta;
        }

        if self.auto_play {
            self.auto_play_timer += delta;
            if self.auto_play_timer >= 1.0 {
                self.auto_play_timer = 0.0;
                if let Some(ref state) = self.combat_state {
                    if state.get_result() == CombatResult::Ongoing {
                        self.execute_round();
                    } else {
                        self.auto_play = false;
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        // Super Auto Pets style background - light, cheerful
        clear_background(Color::new(0.95, 0.97, 1.0, 1.0));

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Draw combat arena in center
        let arena_width = screen_width * 0.7;
        let arena_height = screen_height * 0.6;
        let arena_x = (screen_width - arena_width) / 2.0;
        let arena_y = screen_height * 0.1;

        // Arena background
        self.draw_rounded_rect(arena_x, arena_y, arena_width, arena_height, 20.0, Color::new(0.98, 0.99, 1.0, 1.0));
        draw_rectangle_lines(arena_x, arena_y, arena_width, arena_height, 3.0, Color::new(0.8, 0.8, 0.9, 1.0));

        // Pet positions
        let pet_size = 150.0;
        let pet1_x = arena_x + arena_width * 0.2 - pet_size / 2.0;
        let pet1_y = arena_y + arena_height / 2.0 - pet_size / 2.0;
        let pet2_x = arena_x + arena_width * 0.8 - pet_size / 2.0;
        let pet2_y = arena_y + arena_height / 2.0 - pet_size / 2.0;

        // Get combatant stats
        let c1 = self.get_combatant1();
        let c2 = self.get_combatant2();
        let (c1_health, c1_max_health, c1_attack, c1_defeated) = if let Some(ref state) = self.combat_state {
            let max = if let Some((_, h1_before, _, _, _, _)) = self.combat_history.first() {
                *h1_before
            } else {
                c1.health
            };
            (state.combatant1.health, max, state.combatant1.attack, state.combatant1.is_defeated())
        } else {
            (c1.health, c1.health, c1.attack, false)
        };

        let (c2_health, c2_max_health, c2_attack, c2_defeated) = if let Some(ref state) = self.combat_state {
            let max = if let Some((_, _, h2_before, _, _, _)) = self.combat_history.first() {
                *h2_before
            } else {
                c2.health
            };
            (state.combatant2.health, max, state.combatant2.attack, state.combatant2.is_defeated())
        } else {
            (c2.health, c2.health, c2.attack, false)
        };

        // Attack animation
        let anim_offset = if self.attack_animation_timer > 0.0 {
            (self.attack_animation_timer * 10.0).sin() * 20.0
        } else {
            0.0
        };

        // Draw pets
        self.draw_pet(pet1_x - anim_offset, pet1_y, pet_size, self.get_pet_color1(), c1_health, c1_max_health, c1_attack, c1_defeated);
        self.draw_pet(pet2_x + anim_offset, pet2_y, pet_size, self.get_pet_color2(), c2_health, c2_max_health, c2_attack, c2_defeated);

        // VS text in center
        if self.combat_state.is_none() {
            let vs_text = "VS";
            let vs_size = 48.0;
            let vs_width = measure_text(vs_text, None, vs_size as u16, 1.0).width;
            draw_text(vs_text, arena_x + arena_width / 2.0 - vs_width / 2.0, arena_y + arena_height / 2.0 + vs_size / 2.0, vs_size, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        // Control panel at bottom
        let panel_y = arena_y + arena_height + 20.0;
        let panel_height = screen_height - panel_y - 20.0;
        let panel_width = screen_width - 40.0;
        let panel_x = 20.0;

        self.draw_rounded_rect(panel_x, panel_y, panel_width, panel_height, 15.0, Color::new(0.98, 0.99, 1.0, 1.0));
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, Color::new(0.8, 0.8, 0.9, 1.0));

        // Left side - Combatant 1 controls
        let left_panel_x = panel_x + 20.0;
        let left_panel_y = panel_y + 20.0;
        let left_panel_width = (panel_width - 60.0) / 2.0;

        draw_text("Pet 1", left_panel_x, left_panel_y, 24.0, Color::new(0.2, 0.2, 0.2, 1.0));

        // Preset buttons
        let preset_y = left_panel_y + 40.0;
        draw_text("Preset:", left_panel_x, preset_y, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
        
        let preset_buttons = [
            PredefinedCombatant::Tank,
            PredefinedCombatant::GlassCannon,
            PredefinedCombatant::Balanced,
            PredefinedCombatant::Bruiser,
            PredefinedCombatant::Assassin,
            PredefinedCombatant::Defender,
        ];
        
        let button_width = (left_panel_width - 20.0) / 3.0;
        let button_height = 30.0;
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = left_panel_x + (i % 3) as f32 * (button_width + 10.0);
            let by = preset_y + 25.0 + (i / 3) as f32 * (button_height + 10.0);
            let is_selected = !self.side1_custom && self.side1_preset == *preset;
            if self.draw_preset_button(bx, by, button_width, button_height, *preset, is_selected) {
                self.side1_custom = false;
                self.side1_preset = *preset;
            }
        }

        // Custom toggle
        let custom_y = preset_y + 25.0 + 80.0;
        if self.draw_button(left_panel_x, custom_y, 140.0, 35.0, if self.side1_custom { "Custom: ON" } else { "Custom: OFF" }, Color::new(0.6, 0.6, 0.6, 1.0)) {
            self.side1_custom = !self.side1_custom;
        }

        // Custom inputs
        if self.side1_custom {
            let input_y = custom_y + 50.0;
            draw_text("Health:", left_panel_x, input_y, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(left_panel_x + 80.0, input_y - 22.0, 100.0, 35.0, self.side1_health.clone(), InputField::Side1Health);
            
            draw_text("Attack:", left_panel_x, input_y + 40.0, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(left_panel_x + 80.0, input_y + 18.0, 100.0, 35.0, self.side1_attack.clone(), InputField::Side1Attack);
        }

        // Right side - Combatant 2 controls
        let right_panel_x = left_panel_x + left_panel_width + 20.0;
        let right_panel_y = panel_y + 20.0;

        draw_text("Pet 2", right_panel_x, right_panel_y, 24.0, Color::new(0.2, 0.2, 0.2, 1.0));

        // Preset buttons for Side 2
        let preset_y = right_panel_y + 40.0;
        draw_text("Preset:", right_panel_x, preset_y, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
        
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = right_panel_x + (i % 3) as f32 * (button_width + 10.0);
            let by = preset_y + 25.0 + (i / 3) as f32 * (button_height + 10.0);
            let is_selected = !self.side2_custom && self.side2_preset == *preset;
            if self.draw_preset_button(bx, by, button_width, button_height, *preset, is_selected) {
                self.side2_custom = false;
                self.side2_preset = *preset;
            }
        }

        // Custom toggle for Side 2
        let custom_y = preset_y + 25.0 + 80.0;
        if self.draw_button(right_panel_x, custom_y, 140.0, 35.0, if self.side2_custom { "Custom: ON" } else { "Custom: OFF" }, Color::new(0.6, 0.6, 0.6, 1.0)) {
            self.side2_custom = !self.side2_custom;
        }

        // Custom inputs for Side 2
        if self.side2_custom {
            let input_y = custom_y + 50.0;
            draw_text("Health:", right_panel_x, input_y, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(right_panel_x + 80.0, input_y - 22.0, 100.0, 35.0, self.side2_health.clone(), InputField::Side2Health);
            
            draw_text("Attack:", right_panel_x, input_y + 40.0, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(right_panel_x + 80.0, input_y + 18.0, 100.0, 35.0, self.side2_attack.clone(), InputField::Side2Attack);
        }

        // Center controls
        let center_x = screen_width / 2.0;
        let controls_y = panel_y + panel_height - 80.0;
        
        if self.draw_button(center_x - 200.0, controls_y, 140.0, 45.0, "Start Fight", Color::new(0.2, 0.7, 0.3, 1.0)) {
            self.start_combat();
        }
        
        if self.draw_button(center_x - 40.0, controls_y, 140.0, 45.0, "Next Round", Color::new(0.3, 0.5, 0.8, 1.0)) {
            if let Some(ref state) = self.combat_state {
                if state.get_result() == CombatResult::Ongoing {
                    self.execute_round();
                }
            }
        }
        
        if self.draw_button(center_x + 120.0, controls_y, 140.0, 45.0, if self.auto_play { "Stop Auto" } else { "Auto Play" }, Color::new(0.8, 0.5, 0.2, 1.0)) {
            if self.combat_state.is_some() {
                self.auto_play = !self.auto_play;
                self.auto_play_timer = 0.0;
            }
        }
        
        if self.draw_button(center_x - 120.0, controls_y + 60.0, 140.0, 45.0, "Reset", Color::new(0.7, 0.3, 0.3, 1.0)) {
            self.reset_combat();
        }

        // Combat result display
        if let Some(ref state) = self.combat_state {
            let result = state.get_result();
            let result_text = match result {
                CombatResult::Ongoing => format!("Round {}", state.round),
                CombatResult::Combatant1Wins => "Pet 1 Wins!".to_string(),
                CombatResult::Combatant2Wins => "Pet 2 Wins!".to_string(),
                CombatResult::Draw => "Draw!".to_string(),
            };
            let result_color = match result {
                CombatResult::Ongoing => Color::new(0.5, 0.5, 0.5, 1.0),
                CombatResult::Combatant1Wins => Color::new(0.2, 0.8, 0.3, 1.0),
                CombatResult::Combatant2Wins => Color::new(0.8, 0.2, 0.2, 1.0),
                CombatResult::Draw => Color::new(0.8, 0.8, 0.2, 1.0),
            };
            let text_size = 32.0;
            let text_width = measure_text(&result_text, None, text_size as u16, 1.0).width;
            draw_text(&result_text, center_x - text_width / 2.0, arena_y - 40.0, text_size, result_color);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gui = CombatGUI::new();

    loop {
        let delta = get_frame_time();
        gui.update(delta);
        gui.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
