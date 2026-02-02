/// Multi-combatant combat system with simultaneous attack resolution
/// Similar to Super Auto Pets combat mechanics
/// Supports multiple combatants on each side, with front-to-back attacking

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combatant {
    pub health: i32,
    pub attack: i32,
    pub leadership: i32,  // Max followers this leader can have (team size = leader + leadership)
}

/// Predefined combatants with different stat combinations
impl Combatant {
    /// Tank: High health, low attack - survives long but deals little damage
    pub const TANK: Combatant = Combatant { health: 20, attack: 2, leadership: 4 };
    
    /// Glass Cannon: Low health, high attack - deals massive damage but fragile
    pub const GLASS_CANNON: Combatant = Combatant { health: 5, attack: 8, leadership: 3 };
    
    /// Balanced Fighter: Medium health and attack - well-rounded combatant
    pub const BALANCED: Combatant = Combatant { health: 10, attack: 5, leadership: 3 };
    
    /// Bruiser: High health, medium attack - durable and hits hard
    pub const BRUISER: Combatant = Combatant { health: 15, attack: 6, leadership: 4 };
    
    /// Assassin: Very low health, very high attack - extreme glass cannon
    pub const ASSASSIN: Combatant = Combatant { health: 3, attack: 10, leadership: 2 };
    
    /// Defender: Very high health, very low attack - ultimate tank
    pub const DEFENDER: Combatant = Combatant { health: 25, attack: 1, leadership: 5 };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatState {
    pub side1: Vec<Combatant>,
    pub side2: Vec<Combatant>,
    pub round: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatResult {
    /// Combat continues, both sides still have living combatants
    Ongoing,
    /// Side 1 defeated all combatants on side 2
    Side1Wins,
    /// Side 2 defeated all combatants on side 1
    Side2Wins,
    /// All combatants on both sides defeated simultaneously
    Draw,
}

impl Combatant {
    /// Create a new combatant with specified health, attack, and leadership stats
    pub fn new(health: i32, attack: i32, leadership: i32) -> Self {
        Self { health, attack, leadership }
    }

    /// Check if this combatant is defeated (health <= 0)
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }
}

impl CombatState {
    /// Create a new combat state with two teams of combatants
    /// Validates that team sizes don't exceed the leader's leadership capacity
    pub fn new(side1: Vec<Combatant>, side2: Vec<Combatant>) -> Result<Self, String> {
        // Validate team sizes
        if let Some(leader) = side1.first() {
            let max_size = (leader.leadership + 1) as usize; // leader + followers
            if side1.len() > max_size {
                return Err(format!("Side 1 team size {} exceeds leader's leadership capacity of {}", side1.len(), max_size));
            }
        }
        if let Some(leader) = side2.first() {
            let max_size = (leader.leadership + 1) as usize; // leader + followers
            if side2.len() > max_size {
                return Err(format!("Side 2 team size {} exceeds leader's leadership capacity of {}", side2.len(), max_size));
            }
        }
        
        // Ensure teams are not empty
        if side1.is_empty() {
            return Err("Side 1 cannot be empty".to_string());
        }
        if side2.is_empty() {
            return Err("Side 2 cannot be empty".to_string());
        }

        Ok(Self {
            side1,
            side2,
            round: 0,
        })
    }

    /// Get the front-most combatant for a side (index 0)
    pub fn get_front_combatant(&self, side: usize) -> Option<&Combatant> {
        match side {
            1 => self.side1.first(),
            2 => self.side2.first(),
            _ => None,
        }
    }

    /// Validate that a team size doesn't exceed the leader's leadership capacity
    pub fn validate_team_size(team: &[Combatant]) -> Result<(), String> {
        if team.is_empty() {
            return Err("Team cannot be empty".to_string());
        }
        let leader = &team[0];
        let max_size = (leader.leadership + 1) as usize; // leader + followers
        if team.len() > max_size {
            return Err(format!("Team size {} exceeds leader's leadership capacity of {}", team.len(), max_size));
        }
        Ok(())
    }

    /// Execute one round of combat where all combatants attack simultaneously
    /// Each combatant attacks the front-most enemy (index 0 of opposing side)
    /// Returns the combat result after this round
    pub fn execute_round(&mut self) -> CombatResult {
        self.round += 1;

        // Collect all damage to apply simultaneously
        let mut side1_damage = vec![0; self.side1.len()];
        let mut side2_damage = vec![0; self.side2.len()];

        // Side 1 attacks front-most enemy (side2[0])
        if !self.side2.is_empty() {
            for combatant in self.side1.iter() {
                if !combatant.is_defeated() {
                    side2_damage[0] += combatant.attack;
                }
            }
        }

        // Side 2 attacks front-most enemy (side1[0])
        if !self.side1.is_empty() {
            for combatant in self.side2.iter() {
                if !combatant.is_defeated() {
                    side1_damage[0] += combatant.attack;
                }
            }
        }

        // Apply all damage simultaneously
        for (i, damage) in side1_damage.iter().enumerate() {
            if i < self.side1.len() {
                self.side1[i].health -= damage;
            }
        }
        for (i, damage) in side2_damage.iter().enumerate() {
            if i < self.side2.len() {
                self.side2[i].health -= damage;
            }
        }

        // Remove defeated combatants and shift remaining forward
        self.side1.retain(|c| !c.is_defeated());
        self.side2.retain(|c| !c.is_defeated());

        // Determine result
        self.get_result()
    }

    /// Get the current combat result without executing a round
    pub fn get_result(&self) -> CombatResult {
        let side1_alive = self.side1.iter().any(|c| !c.is_defeated());
        let side2_alive = self.side2.iter().any(|c| !c.is_defeated());

        match (side1_alive, side2_alive) {
            (true, true) => CombatResult::Ongoing,
            (false, true) => CombatResult::Side2Wins,
            (true, false) => CombatResult::Side1Wins,
            (false, false) => CombatResult::Draw,
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
        let c = Combatant::new(10, 5, 3);
        assert_eq!(c.health, 10);
        assert_eq!(c.attack, 5);
        assert_eq!(c.leadership, 3);
        assert!(!c.is_defeated());
    }

    #[test]
    fn test_combatant_defeated() {
        let mut c = Combatant::new(10, 5, 3);
        assert!(!c.is_defeated());
        
        c.health = 0;
        assert!(c.is_defeated());
        
        c.health = -5;
        assert!(c.is_defeated());
    }

    #[test]
    fn test_combat_state_creation() {
        let c1 = Combatant::new(10, 5, 3);
        let c2 = Combatant::new(8, 3, 3);
        let state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        assert_eq!(state.round, 0);
        assert_eq!(state.side1[0].health, 10);
        assert_eq!(state.side2[0].health, 8);
    }

    #[test]
    fn test_combat_state_validation() {
        // Valid: leader with leadership 3 can have 3 followers (total 4)
        let leader = Combatant::new(10, 5, 3);
        let follower1 = Combatant::new(8, 3, 2);
        let follower2 = Combatant::new(6, 2, 2);
        let follower3 = Combatant::new(4, 1, 1);
        let team = vec![leader, follower1, follower2, follower3];
        assert!(CombatState::validate_team_size(&team).is_ok());

        // Invalid: exceeds leadership
        let leader = Combatant::new(10, 5, 2);
        let follower1 = Combatant::new(8, 3, 2);
        let follower2 = Combatant::new(6, 2, 2);
        let follower3 = Combatant::new(4, 1, 1);
        let team = vec![leader, follower1, follower2, follower3];
        assert!(CombatState::validate_team_size(&team).is_err());

        // Invalid: empty team
        let team = vec![];
        assert!(CombatState::validate_team_size(&team).is_err());
    }

    #[test]
    fn test_single_round() {
        let c1 = Combatant::new(10, 5, 3);
        let c2 = Combatant::new(8, 3, 3);
        let mut state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        let result = state.execute_round();
        
        assert_eq!(state.round, 1);
        assert_eq!(state.side1[0].health, 7); // 10 - 3
        assert_eq!(state.side2[0].health, 3); // 8 - 5
        assert_eq!(result, CombatResult::Ongoing);
    }

    #[test]
    fn test_side1_wins() {
        let c1 = Combatant::new(10, 5, 3);
        let c2 = Combatant::new(3, 2, 3);
        let mut state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        // Round 1: c1 takes 2 damage (10 -> 8), c2 takes 5 damage (3 -> -2, defeated)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Side1Wins);
        assert_eq!(state.side1[0].health, 8);
        assert!(state.side2.is_empty()); // c2 was removed
    }

    #[test]
    fn test_side2_wins() {
        let c1 = Combatant::new(3, 2, 3);
        let c2 = Combatant::new(10, 5, 3);
        let mut state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        // Round 1: c1 takes 5 damage (3 -> -2, defeated), c2 takes 2 damage (10 -> 8)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Side2Wins);
        assert!(state.side1.is_empty()); // c1 was removed
        assert_eq!(state.side2[0].health, 8);
    }

    #[test]
    fn test_draw() {
        let c1 = Combatant::new(5, 5, 3);
        let c2 = Combatant::new(5, 5, 3);
        let mut state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        // Round 1: Both take 5 damage (5 -> 0), both defeated simultaneously
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Draw);
        assert!(state.side1.is_empty());
        assert!(state.side2.is_empty());
    }

    #[test]
    fn test_simulate_combat() {
        let c1 = Combatant::new(10, 3, 3);
        let c2 = Combatant::new(8, 2, 3);
        let state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        let (final_state, result) = state.simulate_combat();
        
        // c1 deals 3 damage per round, c2 deals 2 damage per round
        // Round 1: c1=7, c2=5
        // Round 2: c1=5, c2=2
        // Round 3: c1=3, c2=-1 (c2 defeated)
        assert_eq!(result, CombatResult::Side1Wins);
        assert!(!final_state.side1.is_empty());
        assert!(final_state.side2.is_empty());
        assert_eq!(final_state.round, 3);
    }

    #[test]
    fn test_multiple_rounds() {
        let c1 = Combatant::new(20, 3, 3);
        let c2 = Combatant::new(15, 2, 3);
        let mut state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        // Round 1
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.side1[0].health, 18);
        assert_eq!(state.side2[0].health, 12);
        
        // Round 2
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.side1[0].health, 16);
        assert_eq!(state.side2[0].health, 9);
        
        // Round 3
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.side1[0].health, 14);
        assert_eq!(state.side2[0].health, 6);
    }

    #[test]
    fn test_multi_combatant_front_to_back() {
        // Side 1: Leader (10 HP, 5 ATK) + Follower (8 HP, 3 ATK)
        // Side 2: Leader (12 HP, 4 ATK)
        let leader1 = Combatant::new(10, 5, 3);
        let follower1 = Combatant::new(8, 3, 2);
        let leader2 = Combatant::new(12, 4, 3);
        
        let mut state = CombatState::new(
            vec![leader1, follower1],
            vec![leader2]
        ).unwrap();
        
        // Round 1: Both side1 combatants attack side2[0] (5+3=8 damage)
        //          Side2[0] attacks side1[0] (4 damage)
        // Result: side1[0] = 6, side1[1] = 8, side2[0] = 4
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.side1[0].health, 6);  // 10 - 4
        assert_eq!(state.side1[1].health, 8);  // unchanged
        assert_eq!(state.side2[0].health, 4);  // 12 - 8
    }

    #[test]
    fn test_formation_shifting() {
        // Side 1: Leader (3 HP, 2 ATK) - will be defeated
        // Side 2: Leader (10 HP, 5 ATK)
        let leader1 = Combatant::new(3, 2, 3);
        let follower1 = Combatant::new(8, 3, 2);
        let leader2 = Combatant::new(10, 5, 3);
        
        let mut state = CombatState::new(
            vec![leader1, follower1],
            vec![leader2]
        ).unwrap();
        
        // Round 1: side1[0] takes 5 damage (3 -> -2, defeated), side2[0] takes 5 damage (10 -> 5)
        // After removal: side1 = [follower1], side2 = [leader2]
        let result = state.execute_round();
        assert_eq!(result, CombatResult::Ongoing);
        assert_eq!(state.side1.len(), 1);
        assert_eq!(state.side1[0].health, 8); // follower1 is now front
        assert_eq!(state.side2[0].health, 5);
    }

    #[test]
    fn test_get_front_combatant() {
        let c1 = Combatant::new(10, 5, 3);
        let c2 = Combatant::new(8, 3, 3);
        let state = CombatState::new(vec![c1], vec![c2]).unwrap();
        
        assert_eq!(state.get_front_combatant(1).unwrap().health, 10);
        assert_eq!(state.get_front_combatant(2).unwrap().health, 8);
        assert!(state.get_front_combatant(3).is_none());
    }
}
