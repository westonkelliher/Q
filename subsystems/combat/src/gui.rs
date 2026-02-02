use combat::{Combatant, CombatState, CombatResult};
use macroquad::prelude::*;

/// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Combat System GUI".to_owned(),
        window_width: 1000,
        window_height: 700,
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

    fn start_combat(&mut self) {
        let c1 = self.get_combatant1();
        let c2 = self.get_combatant2();
        self.combat_state = Some(CombatState::new(c1, c2));
        self.combat_history.clear();
        self.current_round = 0;
        self.auto_play = false;
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
        }
    }

    fn reset_combat(&mut self) {
        self.combat_state = None;
        self.combat_history.clear();
        self.current_round = 0;
        self.auto_play = false;
    }

    fn draw_button(&self, x: f32, y: f32, width: f32, height: f32, text: &str) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        let bg_color = if is_hovered {
            Color::new(0.4, 0.4, 0.4, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0)
        };

        draw_rectangle(x, y, width, height, bg_color);
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.5, 0.5, 0.5, 1.0));

        let text_size = 16.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height + text_size) / 2.0 - 2.0;
        
        draw_text(text, text_x, text_y, text_size, WHITE);

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
            Color::new(0.2, 0.2, 0.3, 1.0)
        } else if is_hovered {
            Color::new(0.25, 0.25, 0.25, 1.0)
        } else {
            Color::new(0.2, 0.2, 0.2, 1.0)
        };

        draw_rectangle(x, y, width, height, bg_color);
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.5, 0.5, 0.5, 1.0));

        let text_size = 16.0;
        let display_text = if is_selected && get_time() as i32 % 2 == 0 {
            format!("{}|", text)
        } else {
            text
        };
        let text_x = x + 5.0;
        let text_y = y + (height + text_size) / 2.0 - 2.0;
        
        draw_text(&display_text, text_x, text_y, text_size, WHITE);

        is_clicked
    }

    fn draw_health_bar(&self, x: f32, y: f32, width: f32, height: f32, current: i32, max: i32) {
        let max_health = max.max(1);
        let health_ratio = (current.max(0) as f32 / max_health as f32).min(1.0);
        
        // Background (dark red)
        draw_rectangle(x, y, width, height, Color::new(0.3, 0.1, 0.1, 1.0));
        
        // Health bar (green to red gradient)
        let health_color = if health_ratio > 0.5 {
            Color::new(0.2, 0.8, 0.2, 1.0) // Green
        } else if health_ratio > 0.25 {
            Color::new(0.8, 0.6, 0.2, 1.0) // Yellow
        } else {
            Color::new(0.8, 0.2, 0.2, 1.0) // Red
        };
        
        draw_rectangle(x, y, width * health_ratio, height, health_color);
        draw_rectangle_lines(x, y, width, height, 2.0, Color::new(0.5, 0.5, 0.5, 1.0));
        
        // Health text
        let health_text = format!("{} / {}", current.max(0), max_health);
        let text_size = 14.0;
        let text_width = measure_text(&health_text, None, text_size as u16, 1.0).width;
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height + text_size) / 2.0 - 2.0;
        draw_text(&health_text, text_x, text_y, text_size, WHITE);
    }

    fn handle_text_input(&mut self) {
        if let Some(field) = self.selected_input {
            // Handle backspace
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

            // Handle number input
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

        // Handle clicking outside to deselect (handled in draw_text_input)

        // Auto-play logic
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
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let screen_width = screen_width();
        let screen_height = screen_height();
        let padding = 20.0;
        let panel_width = (screen_width - padding * 3.0) / 2.0;

        // Left panel - Combatant 1
        let panel1_x = padding;
        let panel1_y = padding;
        draw_rectangle(panel1_x, panel1_y, panel_width, screen_height - padding * 2.0, Color::new(0.15, 0.15, 0.15, 1.0));
        draw_rectangle_lines(panel1_x, panel1_y, panel_width, screen_height - padding * 2.0, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
        
        draw_text("Combatant 1", panel1_x + 10.0, panel1_y + 30.0, 24.0, WHITE);

        // Preset selector for Side 1
        let preset_y = panel1_y + 60.0;
        draw_text("Preset:", panel1_x + 10.0, preset_y, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
        
        let preset_buttons = [
            PredefinedCombatant::Tank,
            PredefinedCombatant::GlassCannon,
            PredefinedCombatant::Balanced,
            PredefinedCombatant::Bruiser,
            PredefinedCombatant::Assassin,
            PredefinedCombatant::Defender,
        ];
        
        let button_width = (panel_width - 30.0) / 3.0;
        let button_height = 25.0;
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = panel1_x + 10.0 + (i % 3) as f32 * (button_width + 5.0);
            let by = preset_y + 25.0 + (i / 3) as f32 * (button_height + 5.0);
            let is_selected = !self.side1_custom && self.side1_preset == *preset;
            let bg_color = if is_selected {
                Color::new(0.2, 0.5, 0.2, 1.0)
            } else {
                Color::new(0.3, 0.3, 0.3, 1.0)
            };
            draw_rectangle(bx, by, button_width, button_height, bg_color);
            draw_rectangle_lines(bx, by, button_width, button_height, 1.0, Color::new(0.5, 0.5, 0.5, 1.0));
            let text_size = 12.0;
            let text_width = measure_text(preset.name(), None, text_size as u16, 1.0).width;
            draw_text(preset.name(), bx + (button_width - text_width) / 2.0, by + (button_height + text_size) / 2.0 - 2.0, text_size, WHITE);
            
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if mx >= bx && mx <= bx + button_width && my >= by && my <= by + button_height {
                    self.side1_custom = false;
                    self.side1_preset = *preset;
                }
            }
        }

        // Custom toggle for Side 1
        let custom_y = preset_y + 25.0 + 60.0;
        if self.draw_button(panel1_x + 10.0, custom_y, 120.0, 30.0, if self.side1_custom { "Custom: ON" } else { "Custom: OFF" }) {
            self.side1_custom = !self.side1_custom;
        }

        // Custom inputs for Side 1
        if self.side1_custom {
            let input_y = custom_y + 40.0;
            draw_text("Health:", panel1_x + 10.0, input_y, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
            self.draw_text_input(panel1_x + 80.0, input_y - 20.0, 100.0, 30.0, self.side1_health.clone(), InputField::Side1Health);
            
            draw_text("Attack:", panel1_x + 10.0, input_y + 30.0, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
            self.draw_text_input(panel1_x + 80.0, input_y + 10.0, 100.0, 30.0, self.side1_attack.clone(), InputField::Side1Attack);
        }

        // Display current stats
        let stats_y = if self.side1_custom { custom_y + 100.0 } else { custom_y + 40.0 };
        let c1 = self.get_combatant1();
        draw_text(&format!("HP: {}  ATK: {}", c1.health, c1.attack), panel1_x + 10.0, stats_y, 18.0, Color::new(0.9, 0.9, 0.9, 1.0));

        // Health bar for Side 1
        let health_bar_y = stats_y + 30.0;
        let (current_health, max_health) = if let Some(ref state) = self.combat_state {
            // Use initial health from history if available, otherwise estimate
            let initial_health = if let Some((_, h1_before, _, _, _, _)) = self.combat_history.first() {
                *h1_before
            } else {
                c1.health
            };
            (state.combatant1.health, initial_health.max(c1.health))
        } else {
            (c1.health, c1.health)
        };
        self.draw_health_bar(panel1_x + 10.0, health_bar_y, panel_width - 20.0, 30.0, 
            current_health, max_health);

        // Right panel - Combatant 2
        let panel2_x = panel1_x + panel_width + padding;
        let panel2_y = padding;
        draw_rectangle(panel2_x, panel2_y, panel_width, screen_height - padding * 2.0, Color::new(0.15, 0.15, 0.15, 1.0));
        draw_rectangle_lines(panel2_x, panel2_y, panel_width, screen_height - padding * 2.0, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
        
        draw_text("Combatant 2", panel2_x + 10.0, panel2_y + 30.0, 24.0, WHITE);

        // Preset selector for Side 2
        let preset_y = panel2_y + 60.0;
        draw_text("Preset:", panel2_x + 10.0, preset_y, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
        
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = panel2_x + 10.0 + (i % 3) as f32 * (button_width + 5.0);
            let by = preset_y + 25.0 + (i / 3) as f32 * (button_height + 5.0);
            let is_selected = !self.side2_custom && self.side2_preset == *preset;
            let bg_color = if is_selected {
                Color::new(0.2, 0.5, 0.2, 1.0)
            } else {
                Color::new(0.3, 0.3, 0.3, 1.0)
            };
            draw_rectangle(bx, by, button_width, button_height, bg_color);
            draw_rectangle_lines(bx, by, button_width, button_height, 1.0, Color::new(0.5, 0.5, 0.5, 1.0));
            let text_size = 12.0;
            let text_width = measure_text(preset.name(), None, text_size as u16, 1.0).width;
            draw_text(preset.name(), bx + (button_width - text_width) / 2.0, by + (button_height + text_size) / 2.0 - 2.0, text_size, WHITE);
            
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if mx >= bx && mx <= bx + button_width && my >= by && my <= by + button_height {
                    self.side2_custom = false;
                    self.side2_preset = *preset;
                }
            }
        }

        // Custom toggle for Side 2
        let custom_y = preset_y + 25.0 + 60.0;
        if self.draw_button(panel2_x + 10.0, custom_y, 120.0, 30.0, if self.side2_custom { "Custom: ON" } else { "Custom: OFF" }) {
            self.side2_custom = !self.side2_custom;
        }

        // Custom inputs for Side 2
        if self.side2_custom {
            let input_y = custom_y + 40.0;
            draw_text("Health:", panel2_x + 10.0, input_y, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
            self.draw_text_input(panel2_x + 80.0, input_y - 20.0, 100.0, 30.0, self.side2_health.clone(), InputField::Side2Health);
            
            draw_text("Attack:", panel2_x + 10.0, input_y + 30.0, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
            self.draw_text_input(panel2_x + 80.0, input_y + 10.0, 100.0, 30.0, self.side2_attack.clone(), InputField::Side2Attack);
        }

        // Display current stats
        let stats_y = if self.side2_custom { custom_y + 100.0 } else { custom_y + 40.0 };
        let c2 = self.get_combatant2();
        draw_text(&format!("HP: {}  ATK: {}", c2.health, c2.attack), panel2_x + 10.0, stats_y, 18.0, Color::new(0.9, 0.9, 0.9, 1.0));

        // Health bar for Side 2
        let health_bar_y = stats_y + 30.0;
        let (current_health, max_health) = if let Some(ref state) = self.combat_state {
            // Use initial health from history if available, otherwise estimate
            let initial_health = if let Some((_, _, h2_before, _, _, _)) = self.combat_history.first() {
                *h2_before
            } else {
                c2.health
            };
            (state.combatant2.health, initial_health.max(c2.health))
        } else {
            (c2.health, c2.health)
        };
        self.draw_health_bar(panel2_x + 10.0, health_bar_y, panel_width - 20.0, 30.0,
            current_health, max_health);

        // Control buttons in center
        let center_x = screen_width / 2.0;
        let controls_y = screen_height - 150.0;
        
        if self.draw_button(center_x - 200.0, controls_y, 120.0, 40.0, "Start Combat") {
            self.start_combat();
        }
        
        if self.draw_button(center_x - 60.0, controls_y, 120.0, 40.0, "Next Round") {
            if let Some(ref state) = self.combat_state {
                if state.get_result() == CombatResult::Ongoing {
                    self.execute_round();
                }
            }
        }
        
        if self.draw_button(center_x + 80.0, controls_y, 120.0, 40.0, if self.auto_play { "Stop Auto" } else { "Auto Play" }) {
            if self.combat_state.is_some() {
                self.auto_play = !self.auto_play;
                self.auto_play_timer = 0.0;
            }
        }
        
        if self.draw_button(center_x - 200.0, controls_y + 50.0, 120.0, 40.0, "Reset") {
            self.reset_combat();
        }

        // Combat result display
        if let Some(ref state) = self.combat_state {
            let result = state.get_result();
            let result_text = match result {
                CombatResult::Ongoing => format!("Round {}", state.round),
                CombatResult::Combatant1Wins => "Combatant 1 Wins!".to_string(),
                CombatResult::Combatant2Wins => "Combatant 2 Wins!".to_string(),
                CombatResult::Draw => "Draw!".to_string(),
            };
            let result_color = match result {
                CombatResult::Ongoing => Color::new(0.8, 0.8, 0.8, 1.0),
                CombatResult::Combatant1Wins => Color::new(0.2, 0.8, 0.2, 1.0),
                CombatResult::Combatant2Wins => Color::new(0.8, 0.2, 0.2, 1.0),
                CombatResult::Draw => Color::new(0.8, 0.8, 0.2, 1.0),
            };
            let text_width = measure_text(&result_text, None, 20, 1.0).width;
            draw_text(&result_text, center_x - text_width / 2.0, controls_y - 30.0, 20.0, result_color);
        }

        // Combat history
        let history_y = health_bar_y + 50.0;
        draw_text("Combat History:", panel1_x + 10.0, history_y, 16.0, Color::new(0.8, 0.8, 0.8, 1.0));
        
        let history_start = history_y + 25.0;
        let max_history_lines = 8;
        let history_start_idx = if self.combat_history.len() > max_history_lines {
            self.combat_history.len() - max_history_lines
        } else {
            0
        };
        
        for (i, (round, h1_before, h2_before, h1_after, h2_after, result)) in 
            self.combat_history[history_start_idx..].iter().enumerate() {
            let y = history_start + i as f32 * 20.0;
            let result_str = match result {
                CombatResult::Ongoing => "",
                CombatResult::Combatant1Wins => " → C1 Wins",
                CombatResult::Combatant2Wins => " → C2 Wins",
                CombatResult::Draw => " → Draw",
            };
            let history_text = format!("R{}: C1 {}→{}  C2 {}→{}{}", 
                round, h1_before, h1_after, h2_before, h2_after, result_str);
            draw_text(&history_text, panel1_x + 10.0, y, 12.0, Color::new(0.7, 0.7, 0.7, 1.0));
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
