/// Simple one-v-one combat system with simultaneous attack resolution
/// Similar to Super Auto Pets combat mechanics

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combatant {
    pub health: i32,
    pub attack: i32,
}

/// Predefined combatants with different stat combinations
impl Combatant {
    /// Tank: High health, low attack - survives long but deals little damage
    pub const TANK: Combatant = Combatant { health: 20, attack: 2 };
    
    /// Glass Cannon: Low health, high attack - deals massive damage but fragile
    pub const GLASS_CANNON: Combatant = Combatant { health: 5, attack: 8 };
    
    /// Balanced Fighter: Medium health and attack - well-rounded combatant
    pub const BALANCED: Combatant = Combatant { health: 10, attack: 5 };
    
    /// Bruiser: High health, medium attack - durable and hits hard
    pub const BRUISER: Combatant = Combatant { health: 15, attack: 6 };
    
    /// Assassin: Very low health, very high attack - extreme glass cannon
    pub const ASSASSIN: Combatant = Combatant { health: 3, attack: 10 };
    
    /// Defender: Very high health, very low attack - ultimate tank
    pub const DEFENDER: Combatant = Combatant { health: 25, attack: 1 };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatState {
    pub combatant1: Combatant,
    pub combatant2: Combatant,
    pub round: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatResult {
    /// Combat continues, both combatants still alive
    Ongoing,
    /// Combatant 1 defeated combatant 2
    Combatant1Wins,
    /// Combatant 2 defeated combatant 1
    Combatant2Wins,
    /// Both combatants defeated simultaneously
    Draw,
}

impl Combatant {
    /// Create a new combatant with specified health and attack stats
    pub fn new(health: i32, attack: i32) -> Self {
        Self { health, attack }
    }

    /// Check if this combatant is defeated (health <= 0)
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }
}

impl CombatState {
    /// Create a new combat state with two combatants
    pub fn new(combatant1: Combatant, combatant2: Combatant) -> Self {
        Self {
            combatant1,
            combatant2,
            round: 0,
        }
    }

    /// Execute one round of combat where both combatants attack simultaneously
    /// Returns the combat result after this round
    pub fn execute_round(&mut self) -> CombatResult {
        self.round += 1;

        // Both attacks resolve simultaneously
        self.combatant1.health -= self.combatant2.attack;
        self.combatant2.health -= self.combatant1.attack;

        // Determine result
        self.get_result()
    }

    /// Get the current combat result without executing a round
    pub fn get_result(&self) -> CombatResult {
        let c1_defeated = self.combatant1.is_defeated();
        let c2_defeated = self.combatant2.is_defeated();

        match (c1_defeated, c2_defeated) {
            (false, false) => CombatResult::Ongoing,
            (true, false) => CombatResult::Combatant2Wins,
            (false, true) => CombatResult::Combatant1Wins,
            (true, true) => CombatResult::Draw,
        }
    }

    /// Simulate combat to completion, executing rounds until someone wins or draw
    /// Returns the final state and result
    pub fn simulate_combat(mut self) -> (Self, CombatResult) {
        loop {
            let result = self.execute_round();
            match result {
                CombatResult::Ongoing => continue,
                _ => return (self, result),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combatant_creation() {
        let c = Combatant::new(10, 5);
        assert_eq!(c.health, 10);
        assert_eq!(c.attack, 5);
        assert!(!c.is_defeated());
    }

    #[test]
    fn test_combatant_defeated() {
        let mut c = Combatant::new(10, 5);
        assert!(!c.is_defeated());
        
        c.health = 0;
        assert!(c.is_defeated());
        
        c.health = -5;
        assert!(c.is_defeated());
    }

    #[test]
    fn test_combat_state_creation() {
        let c1 = Combatant::new(10, 5);
        let c2 = Combatant::new(8, 3);
        let state = CombatState::new(c1, c2);
        
        assert_eq!(state.round, 0);
        assert_eq!(state.combatant1.health, 10);
        assert_eq!(state.combatant2.health, 8);
    }

    #[test]
    fn test_single_round() {
        let c1 = Combatant::new(10, 5);
        let c2 = Combatant::new(8, 3);
        let mut state = CombatState::new(c1, c2);
        
        let result = state.execute_round();
        
        assert_eq!(state.round, 1);
        assert_eq!(state.combatant1.health, 7); // 10 - 3
        assert_eq!(state.combatant2.health, 3); // 8 - 5
        assert_eq!(result, CombatResult::Ongoing);
    }

    #[test]
    fn test_combatant1_wins() {
        let c1 = Combatant::new(10, 5);
        let c2 = Combatant::new(3, 2);
        let mut state = CombatState::new(c1, c2);
        
        // Round 1: c1 takes 2 damage (10 -> 8), c2 takes 5 damage (3 -> -2, defeated)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Combatant1Wins);
        assert_eq!(state.combatant1.health, 8);
        assert_eq!(state.combatant2.health, -2);
    }

    #[test]
    fn test_combatant2_wins() {
        let c1 = Combatant::new(3, 2);
        let c2 = Combatant::new(10, 5);
        let mut state = CombatState::new(c1, c2);
        
        // Round 1: c1 takes 5 damage (3 -> -2, defeated), c2 takes 2 damage (10 -> 8)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Combatant2Wins);
        assert_eq!(state.combatant1.health, -2);
        assert_eq!(state.combatant2.health, 8);
    }

    #[test]
    fn test_draw() {
        let c1 = Combatant::new(5, 5);
        let c2 = Combatant::new(5, 5);
        let mut state = CombatState::new(c1, c2);
        
        // Round 1: Both take 5 damage (5 -> 0), both defeated simultaneously
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Draw);
        assert_eq!(state.combatant1.health, 0);
        assert_eq!(state.combatant2.health, 0);
    }

    #[test]
    fn test_simulate_combat() {
        let c1 = Combatant::new(10, 3);
        let c2 = Combatant::new(8, 2);
        let state = CombatState::new(c1, c2);
        
        let (final_state, result) = state.simulate_combat();
        
        // c1 deals 3 damage per round, c2 deals 2 damage per round
        // Round 1: c1=7, c2=5
        // Round 2: c1=5, c2=2
        // Round 3: c1=3, c2=-1 (c2 defeated)
        assert_eq!(result, CombatResult::Combatant1Wins);
        assert!(final_state.combatant1.health > 0);
        assert!(final_state.combatant2.health <= 0);
        assert_eq!(final_state.round, 3);
    }

    #[test]
    fn test_multiple_rounds() {
        let c1 = Combatant::new(20, 3);
        let c2 = Combatant::new(15, 2);
        let mut state = CombatState::new(c1, c2);
        
        // Round 1
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.combatant1.health, 18);
        assert_eq!(state.combatant2.health, 12);
        
        // Round 2
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.combatant1.health, 16);
        assert_eq!(state.combatant2.health, 9);
        
        // Round 3
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.combatant1.health, 14);
        assert_eq!(state.combatant2.health, 6);
    }
}
