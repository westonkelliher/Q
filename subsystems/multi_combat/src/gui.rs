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
            PredefinedCombatant::Tank => Color::new(0.3, 0.5, 0.8, 1.0),        // Steel blue - defensive, sturdy
            PredefinedCombatant::GlassCannon => Color::new(0.95, 0.4, 0.2, 1.0), // Bright orange-red - aggressive, dangerous
            PredefinedCombatant::Balanced => Color::new(0.4, 0.7, 0.4, 1.0),    // Forest green - nature, balanced
            PredefinedCombatant::Bruiser => Color::new(0.85, 0.5, 0.15, 1.0),   // Deep orange - strong, powerful
            PredefinedCombatant::Assassin => Color::new(0.5, 0.15, 0.6, 1.0),    // Dark purple - stealthy, deadly
            PredefinedCombatant::Defender => Color::new(0.2, 0.6, 0.85, 1.0),   // Ice blue - protective, defensive
        }
    }
}

/// Detect a combatant's class based on their stats
fn detect_combatant_class(combatant: &Combatant) -> PredefinedCombatant {
    // Match exact predefined stats first
    if combatant.health == 20 && combatant.attack == 2 {
        return PredefinedCombatant::Tank;
    }
    if combatant.health == 5 && combatant.attack == 8 {
        return PredefinedCombatant::GlassCannon;
    }
    if combatant.health == 10 && combatant.attack == 5 {
        return PredefinedCombatant::Balanced;
    }
    if combatant.health == 15 && combatant.attack == 6 {
        return PredefinedCombatant::Bruiser;
    }
    if combatant.health == 3 && combatant.attack == 10 {
        return PredefinedCombatant::Assassin;
    }
    if combatant.health == 25 && combatant.attack == 1 {
        return PredefinedCombatant::Defender;
    }
    
    // Classify by stat patterns
    let health = combatant.health;
    let attack = combatant.attack;
    
    // Defender: Very high health, very low attack
    if health >= 20 && attack <= 2 {
        return PredefinedCombatant::Defender;
    }
    
    // Tank: High health, low attack
    if health >= 15 && attack <= 3 {
        return PredefinedCombatant::Tank;
    }
    
    // Assassin: Very low health, very high attack
    if health <= 4 && attack >= 9 {
        return PredefinedCombatant::Assassin;
    }
    
    // Glass Cannon: Low health, high attack
    if health <= 6 && attack >= 7 {
        return PredefinedCombatant::GlassCannon;
    }
    
    // Bruiser: High health, medium-high attack
    if health >= 12 && attack >= 5 {
        return PredefinedCombatant::Bruiser;
    }
    
    // Default to Balanced for everything else
    PredefinedCombatant::Balanced
}

/// Get color for a combatant based on their class
fn get_combatant_color(combatant: &Combatant) -> Color {
    detect_combatant_class(combatant).color()
}

struct CombatGUI {
    // Teams
    side1_team: Vec<TeamMemberUI>,
    side2_team: Vec<TeamMemberUI>,
    
    // Current input for adding new member
    new_member_side1: TeamMemberUI,
    new_member_side2: TeamMemberUI,
    
    // Combat state
    combat_state: Option<CombatState>,
    combat_history: Vec<(u32, Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<(i32, i32)>, CombatResult)>,
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
    Side1Health(usize),
    Side1Attack(usize),
    Side1Leadership(usize),
    Side2Health(usize),
    Side2Attack(usize),
    Side2Leadership(usize),
}

#[derive(Clone)]
struct TeamMemberUI {
    preset: PredefinedCombatant,
    custom: bool,
    health: String,
    attack: String,
    leadership: String,
}

impl TeamMemberUI {
    fn new() -> Self {
        Self {
            preset: PredefinedCombatant::Balanced,
            custom: false,
            health: "10".to_string(),
            attack: "5".to_string(),
            leadership: "3".to_string(),
        }
    }

    fn to_combatant(&self) -> Combatant {
        if self.custom {
            let health = self.health.parse().unwrap_or(10);
            let attack = self.attack.parse().unwrap_or(5);
            let leadership = self.leadership.parse().unwrap_or(3);
            Combatant::new(health.max(1), attack.max(1), leadership.max(1))
        } else {
            self.preset.to_combatant()
        }
    }

    fn get_color(&self) -> Color {
        if self.custom {
            // Detect class from stats for custom combatants
            get_combatant_color(&self.to_combatant())
        } else {
            self.preset.color()
        }
    }
}

impl CombatGUI {
    fn new() -> Self {
        Self {
            side1_team: Vec::new(),
            side2_team: Vec::new(),
            new_member_side1: TeamMemberUI::new(),
            new_member_side2: TeamMemberUI::new(),
            
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

    fn get_side1_team(&self) -> Vec<Combatant> {
        self.side1_team.iter().map(|m| m.to_combatant()).collect()
    }

    fn get_side2_team(&self) -> Vec<Combatant> {
        self.side2_team.iter().map(|m| m.to_combatant()).collect()
    }

    fn add_to_side1(&mut self) -> Result<(), String> {
        let team = self.get_side1_team();
        
        // Check if we can add
        if team.is_empty() {
            // First member becomes leader
            self.side1_team.push(self.new_member_side1.clone());
            self.new_member_side1 = TeamMemberUI::new();
            return Ok(());
        }
        
        // Check leadership capacity
        let leader = &team[0];
        let max_size = (leader.leadership + 1) as usize;
        if team.len() >= max_size {
            return Err(format!("Team size would exceed leader's leadership capacity of {}", max_size));
        }
        
        self.side1_team.push(self.new_member_side1.clone());
        self.new_member_side1 = TeamMemberUI::new();
        Ok(())
    }

    fn add_to_side2(&mut self) -> Result<(), String> {
        let team = self.get_side2_team();
        
        // Check if we can add
        if team.is_empty() {
            // First member becomes leader
            self.side2_team.push(self.new_member_side2.clone());
            self.new_member_side2 = TeamMemberUI::new();
            return Ok(());
        }
        
        // Check leadership capacity
        let leader = &team[0];
        let max_size = (leader.leadership + 1) as usize;
        if team.len() >= max_size {
            return Err(format!("Team size would exceed leader's leadership capacity of {}", max_size));
        }
        
        self.side2_team.push(self.new_member_side2.clone());
        self.new_member_side2 = TeamMemberUI::new();
        Ok(())
    }

    fn remove_from_side1(&mut self) {
        if !self.side1_team.is_empty() {
            self.side1_team.pop();
        }
    }

    fn remove_from_side2(&mut self) {
        if !self.side2_team.is_empty() {
            self.side2_team.pop();
        }
    }

    fn clear_side1(&mut self) {
        self.side1_team.clear();
    }

    fn clear_side2(&mut self) {
        self.side2_team.clear();
    }

    fn start_combat(&mut self) {
        let side1 = self.get_side1_team();
        let side2 = self.get_side2_team();
        
        if side1.is_empty() || side2.is_empty() {
            return; // Can't start without teams
        }
        
        match CombatState::new(side1, side2) {
            Ok(state) => {
                self.combat_state = Some(state);
                self.combat_history.clear();
                self.current_round = 0;
                self.auto_play = false;
                self.attack_animation_timer = 0.0;
                self.last_attack_round = 0;
            }
            Err(e) => {
                // Could show error message in UI
                eprintln!("Error starting combat: {}", e);
            }
        }
    }

    fn execute_round(&mut self) {
        if let Some(ref mut state) = self.combat_state {
            let round_before = state.round;
            let side1_before: Vec<(i32, i32)> = state.side1.iter().map(|c| (c.health, c.attack)).collect();
            let side2_before: Vec<(i32, i32)> = state.side2.iter().map(|c| (c.health, c.attack)).collect();
            
            let result = state.execute_round();
            
            let side1_after: Vec<(i32, i32)> = state.side1.iter().map(|c| (c.health, c.attack)).collect();
            let side2_after: Vec<(i32, i32)> = state.side2.iter().map(|c| (c.health, c.attack)).collect();
            
            self.combat_history.push((
                round_before + 1,
                side1_before,
                side2_before,
                side1_after,
                side2_after,
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
                    InputField::Side1Health(_) => {
                        self.new_member_side1.health.pop();
                        if self.new_member_side1.health.is_empty() {
                            self.new_member_side1.health = "0".to_string();
                        }
                    }
                    InputField::Side1Attack(_) => {
                        self.new_member_side1.attack.pop();
                        if self.new_member_side1.attack.is_empty() {
                            self.new_member_side1.attack = "0".to_string();
                        }
                    }
                    InputField::Side1Leadership(_) => {
                        self.new_member_side1.leadership.pop();
                        if self.new_member_side1.leadership.is_empty() {
                            self.new_member_side1.leadership = "0".to_string();
                        }
                    }
                    InputField::Side2Health(_) => {
                        self.new_member_side2.health.pop();
                        if self.new_member_side2.health.is_empty() {
                            self.new_member_side2.health = "0".to_string();
                        }
                    }
                    InputField::Side2Attack(_) => {
                        self.new_member_side2.attack.pop();
                        if self.new_member_side2.attack.is_empty() {
                            self.new_member_side2.attack = "0".to_string();
                        }
                    }
                    InputField::Side2Leadership(_) => {
                        self.new_member_side2.leadership.pop();
                        if self.new_member_side2.leadership.is_empty() {
                            self.new_member_side2.leadership = "0".to_string();
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
                        InputField::Side1Health(_) => {
                            if self.new_member_side1.health == "0" {
                                self.new_member_side1.health = digit.to_string();
                            } else {
                                self.new_member_side1.health.push(*digit);
                            }
                        }
                        InputField::Side1Attack(_) => {
                            if self.new_member_side1.attack == "0" {
                                self.new_member_side1.attack = digit.to_string();
                            } else {
                                self.new_member_side1.attack.push(*digit);
                            }
                        }
                        InputField::Side1Leadership(_) => {
                            if self.new_member_side1.leadership == "0" {
                                self.new_member_side1.leadership = digit.to_string();
                            } else {
                                self.new_member_side1.leadership.push(*digit);
                            }
                        }
                        InputField::Side2Health(_) => {
                            if self.new_member_side2.health == "0" {
                                self.new_member_side2.health = digit.to_string();
                            } else {
                                self.new_member_side2.health.push(*digit);
                            }
                        }
                        InputField::Side2Attack(_) => {
                            if self.new_member_side2.attack == "0" {
                                self.new_member_side2.attack = digit.to_string();
                            } else {
                                self.new_member_side2.attack.push(*digit);
                            }
                        }
                        InputField::Side2Leadership(_) => {
                            if self.new_member_side2.leadership == "0" {
                                self.new_member_side2.leadership = digit.to_string();
                            } else {
                                self.new_member_side2.leadership.push(*digit);
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

        // Pet selection panel at top
        let top_panel_height = 250.0;
        let top_panel_y = 20.0;
        let top_panel_width = screen_width - 40.0;
        let top_panel_x = 20.0;

        self.draw_rounded_rect(top_panel_x, top_panel_y, top_panel_width, top_panel_height, 15.0, Color::new(0.98, 0.99, 1.0, 1.0));
        draw_rectangle_lines(top_panel_x, top_panel_y, top_panel_width, top_panel_height, 2.0, Color::new(0.8, 0.8, 0.9, 1.0));

        // Left side - Team 1 controls
        let left_panel_x = top_panel_x + 20.0;
        let left_panel_y = top_panel_y + 20.0;
        let left_panel_width = (top_panel_width - 60.0) / 2.0;

        draw_text("Team 1", left_panel_x, left_panel_y, 24.0, Color::new(0.2, 0.2, 0.2, 1.0));
        
        // Show current team
        let team_y = left_panel_y + 35.0;
        draw_text(&format!("Team: {} member(s)", self.side1_team.len()), left_panel_x, team_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
        if !self.side1_team.is_empty() {
            let leader = &self.side1_team[0];
            let max_size = leader.to_combatant().leadership + 1;
            draw_text(&format!("Max size: {} (Leader LDR: {})", max_size, leader.to_combatant().leadership), left_panel_x, team_y + 20.0, 12.0, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        // Preset buttons for new member
        let preset_y = team_y + 50.0;
        draw_text("New Member Preset:", left_panel_x, preset_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
        
        let preset_buttons = [
            PredefinedCombatant::Tank,
            PredefinedCombatant::GlassCannon,
            PredefinedCombatant::Balanced,
            PredefinedCombatant::Bruiser,
            PredefinedCombatant::Assassin,
            PredefinedCombatant::Defender,
        ];
        
        let button_spacing = 6.0;
        let button_width = (left_panel_width - 2.0 * button_spacing) / 3.0;
        let button_height = 25.0;
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = left_panel_x + (i % 3) as f32 * (button_width + button_spacing);
            let by = preset_y + 20.0 + (i / 3) as f32 * (button_height + 8.0);
            let is_selected = !self.new_member_side1.custom && self.new_member_side1.preset == *preset;
            if self.draw_preset_button(bx, by, button_width, button_height, *preset, is_selected) {
                self.new_member_side1.custom = false;
                self.new_member_side1.preset = *preset;
            }
        }

        // Custom toggle
        let custom_y = preset_y + 20.0 + 60.0;
        if self.draw_button(left_panel_x, custom_y, 120.0, 30.0, if self.new_member_side1.custom { "Custom: ON" } else { "Custom: OFF" }, Color::new(0.6, 0.6, 0.6, 1.0)) {
            self.new_member_side1.custom = !self.new_member_side1.custom;
        }

        // Custom inputs
        if self.new_member_side1.custom {
            let input_y = custom_y + 40.0;
            draw_text("H:", left_panel_x, input_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(left_panel_x + 25.0, input_y - 18.0, 60.0, 28.0, self.new_member_side1.health.clone(), InputField::Side1Health(0));
            
            draw_text("A:", left_panel_x + 95.0, input_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(left_panel_x + 120.0, input_y - 18.0, 60.0, 28.0, self.new_member_side1.attack.clone(), InputField::Side1Attack(0));
            
            draw_text("L:", left_panel_x, input_y + 35.0, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(left_panel_x + 25.0, input_y + 17.0, 60.0, 28.0, self.new_member_side1.leadership.clone(), InputField::Side1Leadership(0));
        }

        // Team management buttons
        let mgmt_y = custom_y + (if self.new_member_side1.custom { 100.0 } else { 50.0 });
        if self.draw_button(left_panel_x, mgmt_y, 100.0, 30.0, "Add", Color::new(0.2, 0.7, 0.3, 1.0)) {
            if let Err(e) = self.add_to_side1() {
                eprintln!("Error: {}", e);
            }
        }
        if self.draw_button(left_panel_x + 110.0, mgmt_y, 50.0, 30.0, "Remove", Color::new(0.7, 0.3, 0.3, 1.0)) {
            self.remove_from_side1();
        }
        if self.draw_button(left_panel_x, mgmt_y + 35.0, 160.0, 30.0, "Clear Team", Color::new(0.6, 0.3, 0.3, 1.0)) {
            self.clear_side1();
        }

        // Right side - Team 2 controls
        let right_panel_x = left_panel_x + left_panel_width + 20.0;
        let right_panel_y = top_panel_y + 20.0;

        draw_text("Team 2", right_panel_x, right_panel_y, 24.0, Color::new(0.2, 0.2, 0.2, 1.0));
        
        // Show current team
        let team_y = right_panel_y + 35.0;
        draw_text(&format!("Team: {} member(s)", self.side2_team.len()), right_panel_x, team_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
        if !self.side2_team.is_empty() {
            let leader = &self.side2_team[0];
            let max_size = leader.to_combatant().leadership + 1;
            draw_text(&format!("Max size: {} (Leader LDR: {})", max_size, leader.to_combatant().leadership), right_panel_x, team_y + 20.0, 12.0, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        // Preset buttons for new member
        let preset_y = team_y + 50.0;
        draw_text("New Member Preset:", right_panel_x, preset_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
        
        let button_spacing = 6.0;
        let button_width = (left_panel_width - 2.0 * button_spacing) / 3.0;
        for (i, preset) in preset_buttons.iter().enumerate() {
            let bx = right_panel_x + (i % 3) as f32 * (button_width + button_spacing);
            let by = preset_y + 20.0 + (i / 3) as f32 * (button_height + 8.0);
            let is_selected = !self.new_member_side2.custom && self.new_member_side2.preset == *preset;
            if self.draw_preset_button(bx, by, button_width, button_height, *preset, is_selected) {
                self.new_member_side2.custom = false;
                self.new_member_side2.preset = *preset;
            }
        }

        // Custom toggle for Side 2
        let custom_y = preset_y + 20.0 + 60.0;
        if self.draw_button(right_panel_x, custom_y, 120.0, 30.0, if self.new_member_side2.custom { "Custom: ON" } else { "Custom: OFF" }, Color::new(0.6, 0.6, 0.6, 1.0)) {
            self.new_member_side2.custom = !self.new_member_side2.custom;
        }

        // Custom inputs for Side 2
        if self.new_member_side2.custom {
            let input_y = custom_y + 40.0;
            draw_text("H:", right_panel_x, input_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(right_panel_x + 25.0, input_y - 18.0, 60.0, 28.0, self.new_member_side2.health.clone(), InputField::Side2Health(0));
            
            draw_text("A:", right_panel_x + 95.0, input_y, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(right_panel_x + 120.0, input_y - 18.0, 60.0, 28.0, self.new_member_side2.attack.clone(), InputField::Side2Attack(0));
            
            draw_text("L:", right_panel_x, input_y + 35.0, 14.0, Color::new(0.4, 0.4, 0.4, 1.0));
            self.draw_text_input(right_panel_x + 25.0, input_y + 17.0, 60.0, 28.0, self.new_member_side2.leadership.clone(), InputField::Side2Leadership(0));
        }

        // Team management buttons for Side 2
        let mgmt_y = custom_y + (if self.new_member_side2.custom { 100.0 } else { 50.0 });
        if self.draw_button(right_panel_x, mgmt_y, 100.0, 30.0, "Add", Color::new(0.2, 0.7, 0.3, 1.0)) {
            if let Err(e) = self.add_to_side2() {
                eprintln!("Error: {}", e);
            }
        }
        if self.draw_button(right_panel_x + 110.0, mgmt_y, 50.0, 30.0, "Remove", Color::new(0.7, 0.3, 0.3, 1.0)) {
            self.remove_from_side2();
        }
        if self.draw_button(right_panel_x, mgmt_y + 35.0, 160.0, 30.0, "Clear Team", Color::new(0.6, 0.3, 0.3, 1.0)) {
            self.clear_side2();
        }

        // Draw combat arena in center
        let arena_width = screen_width * 0.7;
        let arena_height = screen_height * 0.5;
        let arena_x = (screen_width - arena_width) / 2.0;
        let arena_y = top_panel_y + top_panel_height + 20.0;

        // Arena background
        self.draw_rounded_rect(arena_x, arena_y, arena_width, arena_height, 20.0, Color::new(0.98, 0.99, 1.0, 1.0));
        draw_rectangle_lines(arena_x, arena_y, arena_width, arena_height, 3.0, Color::new(0.8, 0.8, 0.9, 1.0));

        // Pet positions - arrange multiple pets per side
        let pet_size = 120.0;
        let pet_spacing = 10.0;
        let _max_pets_per_row = 5;
        
        // Get teams (either from combat state or from team setup)
        let (side1_pets, side2_pets) = if let Some(ref state) = self.combat_state {
            // Use combat state - color by class using original stats
            let s1: Vec<(Combatant, i32, Color)> = state.side1.iter().enumerate().map(|(i, c)| {
                let (max_hp, original_attack) = if let Some((_, s1_before, _, _, _, _)) = self.combat_history.first() {
                    if i < s1_before.len() { 
                        (s1_before[i].0, s1_before[i].1)
                    } else { 
                        (c.health, c.attack)
                    }
                } else {
                    (c.health, c.attack)
                };
                // Use original stats to determine color (so it doesn't change after damage)
                let original_combatant = Combatant::new(max_hp, original_attack, c.leadership);
                let base_color = get_combatant_color(&original_combatant);
                // Add slight gold tint for leader, but keep class color visible
                let color = if i == 0 {
                    Color::new(
                        (base_color.r + 0.15).min(1.0),
                        (base_color.g + 0.1).min(1.0),
                        base_color.b.max(0.2),
                        1.0
                    )
                } else {
                    base_color
                };
                (*c, max_hp, color)
            }).collect();
            let s2: Vec<(Combatant, i32, Color)> = state.side2.iter().enumerate().map(|(i, c)| {
                let (max_hp, original_attack) = if let Some((_, _, s2_before, _, _, _)) = self.combat_history.first() {
                    if i < s2_before.len() { 
                        (s2_before[i].0, s2_before[i].1)
                    } else { 
                        (c.health, c.attack)
                    }
                } else {
                    (c.health, c.attack)
                };
                // Use original stats to determine color (so it doesn't change after damage)
                let original_combatant = Combatant::new(max_hp, original_attack, c.leadership);
                let base_color = get_combatant_color(&original_combatant);
                // Add slight gold tint for leader, but keep class color visible
                let color = if i == 0 {
                    Color::new(
                        (base_color.r + 0.15).min(1.0),
                        (base_color.g + 0.1).min(1.0),
                        base_color.b.max(0.2),
                        1.0
                    )
                } else {
                    base_color
                };
                (*c, max_hp, color)
            }).collect();
            (s1, s2)
        } else {
            // Use team setup - color by class
            let s1: Vec<(Combatant, i32, Color)> = self.side1_team.iter().enumerate().map(|(i, m)| {
                let c = m.to_combatant();
                let base_color = if m.custom {
                    // For custom combatants, detect class from stats
                    get_combatant_color(&c)
                } else {
                    m.get_color()
                };
                // Add slight gold tint for leader, but keep class color visible
                let color = if i == 0 {
                    Color::new(
                        (base_color.r + 0.15).min(1.0),
                        (base_color.g + 0.1).min(1.0),
                        base_color.b.max(0.2),
                        1.0
                    )
                } else {
                    base_color
                };
                (c, c.health, color)
            }).collect();
            let s2: Vec<(Combatant, i32, Color)> = self.side2_team.iter().enumerate().map(|(i, m)| {
                let c = m.to_combatant();
                let base_color = if m.custom {
                    // For custom combatants, detect class from stats
                    get_combatant_color(&c)
                } else {
                    m.get_color()
                };
                // Add slight gold tint for leader, but keep class color visible
                let color = if i == 0 {
                    Color::new(
                        (base_color.r + 0.15).min(1.0),
                        (base_color.g + 0.1).min(1.0),
                        base_color.b.max(0.2),
                        1.0
                    )
                } else {
                    base_color
                };
                (c, c.health, color)
            }).collect();
            (s1, s2)
        };

        // Attack animation
        let anim_offset = if self.attack_animation_timer > 0.0 {
            (self.attack_animation_timer * 10.0).sin() * 15.0
        } else {
            0.0
        };

        // Draw Side 1 pets (left side, arranged vertically)
        let side1_start_x = arena_x + arena_width * 0.15;
        let side1_start_y = arena_y + arena_height * 0.1;
        for (i, (combatant, max_hp, color)) in side1_pets.iter().enumerate() {
            let pet_y = side1_start_y + (i as f32 * (pet_size + pet_spacing));
            let pet_x = side1_start_x - anim_offset;
            self.draw_pet(pet_x, pet_y, pet_size, *color, combatant.health, *max_hp, combatant.attack, combatant.is_defeated());
            
            // Draw leader indicator
            if i == 0 {
                draw_text("L", pet_x + pet_size - 15.0, pet_y + 5.0, 14.0, Color::new(1.0, 0.8, 0.0, 1.0));
            }
        }

        // Draw Side 2 pets (right side, arranged vertically)
        let side2_start_x = arena_x + arena_width * 0.85 - pet_size;
        let side2_start_y = arena_y + arena_height * 0.1;
        for (i, (combatant, max_hp, color)) in side2_pets.iter().enumerate() {
            let pet_y = side2_start_y + (i as f32 * (pet_size + pet_spacing));
            let pet_x = side2_start_x + anim_offset;
            self.draw_pet(pet_x, pet_y, pet_size, *color, combatant.health, *max_hp, combatant.attack, combatant.is_defeated());
            
            // Draw leader indicator
            if i == 0 {
                draw_text("L", pet_x + pet_size - 15.0, pet_y + 5.0, 14.0, Color::new(1.0, 0.8, 0.0, 1.0));
            }
        }

        // VS text in center
        if self.combat_state.is_none() {
            let vs_text = "VS";
            let vs_size = 48.0;
            let vs_width = measure_text(vs_text, None, vs_size as u16, 1.0).width;
            draw_text(vs_text, arena_x + arena_width / 2.0 - vs_width / 2.0, arena_y + arena_height / 2.0 + vs_size / 2.0, vs_size, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        // Center controls at bottom
        let center_x = screen_width / 2.0;
        let controls_y = arena_y + arena_height + 30.0;
        let button_width = 130.0;
        let button_height = 45.0;
        let button_spacing = 15.0;
        
        // Top row: Start Fight, Next Round, Auto Play
        let top_row_start_x = center_x - (button_width * 1.5 + button_spacing);
        
        if self.draw_button(top_row_start_x, controls_y, button_width, button_height, "Start Fight", Color::new(0.2, 0.7, 0.3, 1.0)) {
            self.start_combat();
        }
        
        if self.draw_button(top_row_start_x + button_width + button_spacing, controls_y, button_width, button_height, "Next Round", Color::new(0.3, 0.5, 0.8, 1.0)) {
            if let Some(ref state) = self.combat_state {
                if state.get_result() == CombatResult::Ongoing {
                    self.execute_round();
                }
            }
        }
        
        if self.draw_button(top_row_start_x + 2.0 * (button_width + button_spacing), controls_y, button_width, button_height, if self.auto_play { "Stop Auto" } else { "Auto Play" }, Color::new(0.8, 0.5, 0.2, 1.0)) {
            if self.combat_state.is_some() {
                self.auto_play = !self.auto_play;
                self.auto_play_timer = 0.0;
            }
        }
        
        // Bottom row: Reset button centered
        if self.draw_button(center_x - button_width / 2.0, controls_y + button_height + button_spacing, button_width, button_height, "Reset", Color::new(0.7, 0.3, 0.3, 1.0)) {
            self.reset_combat();
        }

        // Combat result display
        if let Some(ref state) = self.combat_state {
            let result = state.get_result();
            let result_text = match result {
                CombatResult::Ongoing => format!("Round {}", state.round),
                CombatResult::Side1Wins => "Team 1 Wins!".to_string(),
                CombatResult::Side2Wins => "Team 2 Wins!".to_string(),
                CombatResult::Draw => "Draw!".to_string(),
            };
            let result_color = match result {
                CombatResult::Ongoing => Color::new(0.5, 0.5, 0.5, 1.0),
                CombatResult::Side1Wins => Color::new(0.2, 0.8, 0.3, 1.0),
                CombatResult::Side2Wins => Color::new(0.8, 0.2, 0.2, 1.0),
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
