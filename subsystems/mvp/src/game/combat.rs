/// Simple one-v-one combat system with simultaneous attack resolution
/// Copied from ../combat/ subsystem

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
    pub player: Combatant,
    pub enemy: Combatant,
    pub round: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatResult {
    /// Combat continues, both combatants still alive
    Ongoing,
    /// Player defeated the enemy
    PlayerWins,
    /// Enemy defeated the player
    EnemyWins,
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
    /// Create a new combat state with player and enemy combatants
    pub fn new(player: Combatant, enemy: Combatant) -> Self {
        Self {
            player,
            enemy,
            round: 0,
        }
    }

    /// Execute one round of combat where both combatants attack simultaneously
    /// Returns the combat result after this round
    pub fn execute_round(&mut self) -> CombatResult {
        self.round += 1;

        // Both attacks resolve simultaneously
        self.player.health -= self.enemy.attack;
        self.enemy.health -= self.player.attack;

        // Ensure health doesn't go below 0
        self.player.health = self.player.health.max(0);
        self.enemy.health = self.enemy.health.max(0);

        // Determine result
        self.get_result()
    }

    /// Get the current combat result without executing a round
    pub fn get_result(&self) -> CombatResult {
        let player_defeated = self.player.is_defeated();
        let enemy_defeated = self.enemy.is_defeated();

        match (player_defeated, enemy_defeated) {
            (false, false) => CombatResult::Ongoing,
            (true, false) => CombatResult::EnemyWins,
            (false, true) => CombatResult::PlayerWins,
            (true, true) => CombatResult::Draw,
        }
    }

    /// Restore both combatants to full health (used when fleeing)
    pub fn restore_health(&mut self, player_max_health: i32, enemy_max_health: i32) {
        self.player.health = player_max_health;
        self.enemy.health = enemy_max_health;
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
        let player = Combatant::new(10, 5);
        let enemy = Combatant::new(8, 3);
        let state = CombatState::new(player, enemy);
        
        assert_eq!(state.round, 0);
        assert_eq!(state.player.health, 10);
        assert_eq!(state.enemy.health, 8);
    }

    #[test]
    fn test_single_round() {
        let player = Combatant::new(10, 5);
        let enemy = Combatant::new(8, 3);
        let mut state = CombatState::new(player, enemy);
        
        let result = state.execute_round();
        
        assert_eq!(state.round, 1);
        assert_eq!(state.player.health, 7); // 10 - 3
        assert_eq!(state.enemy.health, 3); // 8 - 5
        assert_eq!(result, CombatResult::Ongoing);
    }

    #[test]
    fn test_player_wins() {
        let player = Combatant::new(10, 5);
        let enemy = Combatant::new(3, 2);
        let mut state = CombatState::new(player, enemy);
        
        // Round 1: player takes 2 damage (10 -> 8), enemy takes 5 damage (3 -> -2, defeated)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::PlayerWins);
        assert_eq!(state.player.health, 8);
        assert_eq!(state.enemy.health, 0);
    }

    #[test]
    fn test_enemy_wins() {
        let player = Combatant::new(3, 2);
        let enemy = Combatant::new(10, 5);
        let mut state = CombatState::new(player, enemy);
        
        // Round 1: player takes 5 damage (3 -> -2, defeated), enemy takes 2 damage (10 -> 8)
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::EnemyWins);
        assert_eq!(state.player.health, 0);
        assert_eq!(state.enemy.health, 8);
    }

    #[test]
    fn test_draw() {
        let player = Combatant::new(5, 5);
        let enemy = Combatant::new(5, 5);
        let mut state = CombatState::new(player, enemy);
        
        // Round 1: Both take 5 damage (5 -> 0), both defeated simultaneously
        let result = state.execute_round();
        
        assert_eq!(result, CombatResult::Draw);
        assert_eq!(state.player.health, 0);
        assert_eq!(state.enemy.health, 0);
    }

    #[test]
    fn test_restore_health() {
        let player = Combatant::new(10, 5);
        let enemy = Combatant::new(8, 3);
        let mut state = CombatState::new(player, enemy);
        
        // Execute a round to damage both
        state.execute_round();
        assert_eq!(state.player.health, 7);
        assert_eq!(state.enemy.health, 3);
        
        // Restore health
        state.restore_health(10, 8);
        assert_eq!(state.player.health, 10);
        assert_eq!(state.enemy.health, 8);
    }
}
