use super::crafting::ItemInstanceId;

/// Character inventory - simple list of items (no stacking)
#[derive(Debug, Clone)]
pub struct Inventory {
    /// Items in the inventory (no stacking, infinite capacity for now)
    pub items: Vec<ItemInstanceId>,
}

impl Inventory {
    /// Create a new empty inventory
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Add an item to the inventory
    pub fn add_item(&mut self, item: ItemInstanceId) {
        self.items.push(item);
    }

    /// Remove an item from the inventory at the given index
    /// Returns the item if it exists
    pub fn remove_item(&mut self, index: usize) -> Option<ItemInstanceId> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Get the number of items in the inventory
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Character struct representing the player character
/// Contains position and stats (health, attack)
#[derive(Debug, Clone)]
pub struct Character {
    /// Current land coordinates
    pub land_position: (i32, i32),
    /// Current tile coordinates within the land (None when in terrain view)
    pub tile_position: Option<(usize, usize)>,
    /// Current health
    pub health: i32,
    /// Maximum health
    pub max_health: i32,
    /// Attack stat
    pub attack: i32,
    /// Character inventory
    pub inventory: Inventory,
}

impl Character {
    /// Create a new character with default stats
    /// Starts at land (0, 0) with no tile position (terrain view)
    pub fn new() -> Self {
        let inventory = Inventory::new();
        
        Self {
            land_position: (0, 0),
            tile_position: None,
            health: 10,
            max_health: 10,
            attack: 5,
            inventory,
        }
    }

    /// Get current land position
    pub fn get_land_position(&self) -> (i32, i32) {
        self.land_position
    }

    /// Get current tile position
    pub fn get_tile_position(&self) -> Option<(usize, usize)> {
        self.tile_position
    }

    /// Set land position
    pub fn set_land_position(&mut self, x: i32, y: i32) {
        self.land_position = (x, y);
    }

    /// Set tile position
    pub fn set_tile_position(&mut self, pos: Option<(usize, usize)>) {
        self.tile_position = pos;
    }

    /// Get current health
    pub fn get_health(&self) -> i32 {
        self.health
    }

    /// Get max health
    pub fn get_max_health(&self) -> i32 {
        self.max_health
    }

    /// Get attack stat
    pub fn get_attack(&self) -> i32 {
        self.attack
    }

    /// Take damage (reduce health)
    /// Health cannot go below 0
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    /// Heal (restore health)
    /// Health cannot exceed max_health
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Check if character is defeated (health <= 0)
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }

    /// Get a reference to the inventory
    pub fn get_inventory(&self) -> &Inventory {
        &self.inventory
    }

    /// Get a mutable reference to the inventory
    pub fn get_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_damage() {
        let mut char = Character::new();
        assert_eq!(char.health, 10);

        char.take_damage(3);
        assert_eq!(char.health, 7);

        char.take_damage(10);
        assert_eq!(char.health, 0); // Cannot go below 0
    }

    #[test]
    fn test_character_heal() {
        let mut char = Character::new();
        char.take_damage(5);
        assert_eq!(char.health, 5);

        char.heal(3);
        assert_eq!(char.health, 8);

        char.heal(10);
        assert_eq!(char.health, 10); // Cannot exceed max_health
    }

    #[test]
    fn test_character_defeated() {
        let mut char = Character::new();
        assert!(!char.is_defeated());

        char.take_damage(10);
        assert!(char.is_defeated());

        char.take_damage(5);
        assert!(char.is_defeated()); // Still defeated
    }
}
