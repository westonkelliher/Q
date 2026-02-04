use super::world::types::{World, Substrate, Biome};
use super::character::Character;
use super::combat::CombatResult;
use super::crafting::{CraftingRegistry, ItemInstanceId};

/// Information about a tile
#[derive(Debug, Clone)]
pub struct TileInfo {
    pub substrate: Substrate,
    pub objects: Vec<ItemInstanceId>,
    pub biome: Biome,
}

/// View mode enum for tracking which view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentMode {
    /// Terrain view: Shows biome overview (one tile per land)
    Terrain,
    /// Combat view: Combat sequence before entering a land
    Combat,
    /// Land view: Shows detailed 8x8 tile grid of selected land
    Land,
}

/// Game state that tracks the current world and player position
pub struct GameState {
    pub world: World,
    pub current_mode: CurrentMode,
    pub character: Character,
    /// Combat round counter (0 when not in combat, increments during combat)
    pub combat_round: u32,
    /// Crafting registry containing all items, recipes, and instances
    pub crafting_registry: CraftingRegistry,
}

impl GameState {
    /// Create a new game state with the given world
    pub fn new(world: World, crafting_registry: CraftingRegistry) -> Self {
        let mut character = Character::new();
        character.set_land_position(0, 0);
        character.set_tile_position(None);

        Self {
            world,
            current_mode: CurrentMode::Terrain,
            character,
            combat_round: 0,
            crafting_registry,
        }
    }

    /// Get current land coordinates (from character)
    pub fn current_land(&self) -> (i32, i32) {
        self.character.get_land_position()
    }

    /// Get current tile coordinates within the current land (from character)
    /// Returns None if in terrain view
    pub fn current_tile(&self) -> Option<(usize, usize)> {
        self.character.get_tile_position()
    }

    /// Get the center biome of the current land
    pub fn current_biome(&self) -> Option<&Biome> {
        let (x, y) = self.current_land();
        self.world.terrain.get(&(x, y)).map(|land| &land.center)
    }

    /// Get the biome for a specific tile within a land
    /// Based on tile position: corners, edges, or center
    fn get_tile_biome(land: &super::world::types::Land, tile_x: usize, tile_y: usize) -> &super::world::types::Biome {
        let is_top = tile_y == 0;
        let is_bottom = tile_y == 7;
        let is_left = tile_x == 0;
        let is_right = tile_x == 7;
        
        match (is_left, is_right, is_top, is_bottom) {
            // Corners
            (true, false, true, false) => &land.top_left,
            (false, true, true, false) => &land.top_right,
            (true, false, false, true) => &land.bottom_left,
            (false, true, false, true) => &land.bottom_right,
            // Edges
            (_, _, true, false) => &land.top,
            (_, _, false, true) => &land.bottom,
            (true, false, _, _) => &land.left,
            (false, true, _, _) => &land.right,
            // Center
            _ => &land.center,
        }
    }

    /// Get current tile information (substrate, objects, biome)
    /// Returns None if in terrain view or if land doesn't exist
    pub fn current_tile_info(&self) -> Option<TileInfo> {
        if self.current_mode != CurrentMode::Land {
            return None;
        }

        let (land_x, land_y) = self.current_land();
        let (tile_x, tile_y) = self.current_tile()?;
        
        let land = self.world.terrain.get(&(land_x, land_y))?;
        let tile = &land.tiles[tile_y][tile_x];
        let biome = Self::get_tile_biome(land, tile_x, tile_y);
        
        Some(TileInfo {
            substrate: tile.substrate.clone(),
            objects: tile.objects.clone(),
            biome: biome.clone(),
        })
    }

    /// Move between lands (terrain view)
    /// Clamps coordinates to 0-4 range
    pub fn move_terrain(&mut self, dx: i32, dy: i32) {
        if self.current_mode != CurrentMode::Terrain {
            return;
        }

        let (current_x, current_y) = self.character.get_land_position();
        let new_x = (current_x + dx).max(0).min(4);
        let new_y = (current_y + dy).max(0).min(4);
        
        // Update character position (source of truth)
        self.character.set_land_position(new_x, new_y);
        self.character.set_tile_position(None);
    }

    /// Move within the current land (land view)
    /// Clamps coordinates to 0-7 range
    pub fn move_land(&mut self, dx: i32, dy: i32) {
        if self.current_mode != CurrentMode::Land {
            return;
        }

        let (current_x, current_y) = self.character.get_tile_position().unwrap_or((4, 4));
        let new_x = ((current_x as i32) + dx).max(0).min(7) as usize;
        let new_y = ((current_y as i32) + dy).max(0).min(7) as usize;
        
        // Update character position (source of truth)
        self.character.set_tile_position(Some((new_x, new_y)));
    }

    /// Enter land view for the currently selected land
    /// If the land has an enemy, enters combat mode instead
    pub fn enter_land(&mut self) {
        if self.current_mode != CurrentMode::Terrain {
            return;
        }

        let (land_x, land_y) = self.character.get_land_position();
        
        // Check if land has an enemy
        if let Some(land) = self.world.terrain.get(&(land_x, land_y)) {
            if let Some(enemy) = &land.enemy {
                // Check if enemy is already defeated
                if enemy.is_defeated() {
                    // Enemy defeated, proceed to land view
                    self.enter_land_view_internal(land_x, land_y);
                } else {
                    // Start combat
                    self.start_combat(land_x, land_y, enemy.clone());
                }
            } else {
                // No enemy, proceed to land view
                self.enter_land_view_internal(land_x, land_y);
            }
        }
    }

    /// Internal helper to enter land view (after combat or if no enemy)
    fn enter_land_view_internal(&mut self, _land_x: i32, _land_y: i32) {
        // Update character to have a tile position (default to center)
        self.character.set_tile_position(Some((4, 4)));
        
        self.current_mode = CurrentMode::Land;
    }

    /// Start combat with an enemy
    fn start_combat(&mut self, land_x: i32, land_y: i32, mut enemy: super::world::types::Enemy) {
        // Restore enemy to full health when starting combat
        enemy.restore_health();
        
        // Update enemy in world to full health
        if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
            if let Some(ref mut world_enemy) = land.enemy {
                world_enemy.restore_health();
            }
        }
        
        // Enter combat mode and reset round counter
        self.current_mode = CurrentMode::Combat;
        self.combat_round = 0;
    }

    /// Execute a combat round (attack)
    /// Returns the combat result
    pub fn combat_attack(&mut self) -> CombatResult {
        if self.current_mode != CurrentMode::Combat {
            return CombatResult::Ongoing;
        }
        
        // Increment round counter
        self.combat_round += 1;
        
        let (land_x, land_y) = self.character.get_land_position();
        
        // Get enemy (must exist if we're in combat)
        let enemy = self.world.terrain.get_mut(&(land_x, land_y))
            .and_then(|land| land.enemy.as_mut())
            .expect("Enemy must exist in combat mode");
        
        // Store attack values before mutations
        let player_attack = self.character.get_attack();
        let enemy_attack = enemy.attack;
        
        // Execute simultaneous attacks
        self.character.health -= enemy_attack;
        enemy.health -= player_attack;
        
        // Ensure health doesn't go below 0
        self.character.health = self.character.health.max(0);
        enemy.health = enemy.health.max(0);
        
        // Determine result
        let player_defeated = self.character.health <= 0;
        let enemy_defeated = enemy.health <= 0;
        
        let result = match (player_defeated, enemy_defeated) {
            (false, false) => CombatResult::Ongoing,
            (true, false) => CombatResult::EnemyWins,
            (false, true) => CombatResult::PlayerWins,
            (true, true) => CombatResult::Draw,
        };
        
        // Handle combat conclusion
        match result {
            CombatResult::PlayerWins => {
                // Combat won - reset round counter and enter land view
                self.combat_round = 0;
                self.enter_land_view_internal(land_x, land_y);
            }
            CombatResult::EnemyWins | CombatResult::Draw => {
                // Player defeated - restore both to their starting states
                let (land_x, land_y) = self.character.get_land_position();
                
                // Restore enemy health in world (so they're full health next time)
                if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
                    if let Some(ref mut enemy) = land.enemy {
                        enemy.restore_health();
                    }
                }
                
                // Restore character to half health
                let half_health = self.character.get_max_health() / 2;
                self.character.health = half_health;
                
                // Exit combat and return to terrain view
                self.combat_round = 0;
                self.current_mode = CurrentMode::Terrain;
            }
            CombatResult::Ongoing => {
                // Combat continues
            }
        }
        
        result
    }

    /// Flee from combat (restore enemy health and return to terrain view)
    /// Character health persists (not restored)
    pub fn combat_flee(&mut self) {
        // Restore enemy health in world (so they're full health next time)
        let (land_x, land_y) = self.character.get_land_position();
        if let Some(land) = self.world.terrain.get_mut(&(land_x, land_y)) {
            if let Some(ref mut enemy) = land.enemy {
                enemy.restore_health();
            }
        }
        
        // Exit combat and return to terrain view
        // Character health is NOT restored - it persists
        self.combat_round = 0;
        self.current_mode = CurrentMode::Terrain;
    }

    /// Exit land view and return to terrain view
    pub fn exit_land(&mut self) {
        if self.current_mode != CurrentMode::Land {
            return;
        }
        
        // Update character to remove tile position
        self.character.set_tile_position(None);
        
        self.current_mode = CurrentMode::Terrain;
    }

    /// Check if a land exists at the given coordinates
    pub fn land_exists(&self, x: i32, y: i32) -> bool {
        self.world.terrain.contains_key(&(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::world::create_hardcoded_world;
    use crate::game::crafting::CraftingRegistry;
    
    fn create_test_state() -> GameState {
        let mut crafting_registry = CraftingRegistry::new();
        crate::game::crafting::content::register_sample_content(&mut crafting_registry);
        let world = create_hardcoded_world(&mut crafting_registry);
        GameState::new(world, crafting_registry)
    }

    #[test]
    fn test_terrain_movement() {
        let mut state = create_test_state();
        
        // Move right
        state.move_terrain(1, 0);
        assert_eq!(state.current_land(), (1, 0));
        
        // Move down
        state.move_terrain(0, 1);
        assert_eq!(state.current_land(), (1, 1));
        
        // Move left
        state.move_terrain(-1, 0);
        assert_eq!(state.current_land(), (0, 1));
        
        // Move up
        state.move_terrain(0, -1);
        assert_eq!(state.current_land(), (0, 0));
    }

    #[test]
    fn test_terrain_coordinate_clamping() {
        let mut state = create_test_state();
        
        // Try to move beyond bounds
        state.move_terrain(10, 10);
        assert_eq!(state.current_land(), (4, 4));
        
        // Try to move to negative coordinates
        state.move_terrain(-10, -10);
        assert_eq!(state.current_land(), (0, 0));
    }

    #[test]
    fn test_terrain_movement_only_in_terrain_view() {
        let mut state = create_test_state();
        
        // Enter land view
        state.enter_land();
        assert_eq!(state.current_mode, CurrentMode::Land);
        
        // Try to move terrain (should not work)
        let original_land = state.current_land();
        state.move_terrain(1, 0);
        assert_eq!(state.current_land(), original_land);
    }

    #[test]
    fn test_enter_land_view() {
        let mut state = create_test_state();
        
        state.move_terrain(2, 2);
        assert_eq!(state.current_mode, CurrentMode::Terrain);
        
        state.enter_land();
        assert_eq!(state.current_mode, CurrentMode::Land);
        assert_eq!(state.current_land(), (2, 2));
        assert!(state.current_tile().is_some());
    }

    #[test]
    fn test_land_movement() {
        let mut state = create_test_state();
        
        state.enter_land();
        let initial_tile = state.current_tile().unwrap();
        
        // Move right
        state.move_land(1, 0);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0 + 1);
        assert_eq!(tile.1, initial_tile.1);
        
        // Move down
        state.move_land(0, 1);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0 + 1);
        assert_eq!(tile.1, initial_tile.1 + 1);
        
        // Move left
        state.move_land(-1, 0);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile.0, initial_tile.0);
        assert_eq!(tile.1, initial_tile.1 + 1);
        
        // Move up
        state.move_land(0, -1);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, initial_tile);
    }

    #[test]
    fn test_land_coordinate_clamping() {
        let mut state = create_test_state();
        
        state.enter_land();
        // Initial position is (4, 4) - center of the land
        
        // Try to move beyond bounds from (4, 4)
        state.move_land(10, 10);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, (7, 7)); // Clamped to max
        
        // Try to move to negative coordinates from (7, 7)
        state.move_land(-10, -10);
        let tile = state.current_tile().unwrap();
        assert_eq!(tile, (0, 0)); // Clamped to min
    }

    #[test]
    fn test_land_movement_only_in_land_view() {
        let mut state = create_test_state();
        
        assert_eq!(state.current_mode, CurrentMode::Terrain);
        
        // Try to move land (should not work)
        state.move_land(1, 0);
        assert_eq!(state.current_tile(), None);
    }

    #[test]
    fn test_exit_land_view() {
        let mut state = create_test_state();
        
        // Use (2,2) which has no enemy, or (0,0) which is the start
        state.move_terrain(2, 2);
        state.enter_land();
        assert_eq!(state.current_mode, CurrentMode::Land);
        
        state.exit_land();
        assert_eq!(state.current_mode, CurrentMode::Terrain);
        assert_eq!(state.current_land(), (2, 2));
        assert_eq!(state.current_tile(), None);
    }

    #[test]
    fn test_land_exists() {
        let state = create_test_state();
        
        // Test existing lands
        assert!(state.land_exists(0, 0));
        assert!(state.land_exists(2, 2));
        assert!(state.land_exists(4, 4));
        
        // Test non-existing lands
        assert!(!state.land_exists(5, 5));
        assert!(!state.land_exists(-1, -1));
    }
}
