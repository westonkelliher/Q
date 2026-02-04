use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game::crafting::ItemInstanceId;

/// Simple enemy stats (copied from combat module)
/// Stored separately to avoid circular dependencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enemy {
    pub health: i32,
    pub attack: i32,
    pub max_health: i32, // Store max health for restoration when fleeing
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Substrate {
    Grass,
    Dirt,
    Stone,
    Mud,
    Water,
    Brush,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub substrate: Substrate,
    pub objects: Vec<ItemInstanceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Biome {
    Forest,
    Meadow,
    Lake,
    Mountain,
    Plains,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Land {
    pub tiles: [[Tile; 8]; 8],
    // 9 biomes in a 3x3 pattern (named explicitly)
    pub center: Biome,       // main biome, determines most tiles
    pub top: Biome,          // top edge (6 tiles)
    pub bottom: Biome,       // bottom edge (6 tiles)
    pub left: Biome,         // left edge (6 tiles)
    pub right: Biome,        // right edge (6 tiles)
    pub top_left: Biome,     // corner (1 tile)
    pub top_right: Biome,    // corner (1 tile)
    pub bottom_left: Biome,  // corner (1 tile)
    pub bottom_right: Biome, // corner (1 tile)
    /// Optional enemy that blocks this land (must be defeated to enter)
    pub enemy: Option<Enemy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub terrain: HashMap<(i32, i32), Land>,
    pub seed: u64,
}

impl Biome {
    pub fn to_char(&self) -> &str {
        match self {
            Biome::Forest => "🟩",  // Green square
            Biome::Meadow => "🟨",  // Yellow square
            Biome::Lake => "🟦",   // Blue square
            Biome::Mountain => "⬜", // White square
            Biome::Plains => "🟫",  // Brown square
        }
    }

    /// Get RGB color values (0.0-1.0) matching the main Q game
    pub fn to_color(&self) -> (f32, f32, f32) {
        match self {
            Biome::Forest => (0.1, 0.5, 0.1),         // Dark green
            Biome::Meadow => (0.7, 0.9, 0.4),         // Light green/yellow
            Biome::Lake => (0.2, 0.5, 0.9),           // Blue
            Biome::Mountain => (0.8, 0.8, 0.85),      // Gray/white
            Biome::Plains => (0.6, 0.5, 0.35),        // Brown/tan (dirt-colored)
        }
    }
}

impl Substrate {
    pub fn to_char(&self) -> char {
        match self {
            Substrate::Grass => '🟢',  // Green circle
            Substrate::Dirt => '🟤',  // Brown circle
            Substrate::Stone => '⚪',  // White circle
            Substrate::Mud => '🟫',   // Brown square
            Substrate::Water => '🔵', // Blue circle
            Substrate::Brush => '🟡', // Yellow circle
        }
    }

    /// Get RGB color values (0.0-1.0) matching the main Q game
    pub fn to_color(&self) -> (f32, f32, f32) {
        match self {
            Substrate::Grass => (0.7, 0.9, 0.4),      // Light green/yellow (same as meadow)
            Substrate::Dirt => (0.6, 0.4, 0.2),       // Brown
            Substrate::Stone => (0.7, 0.7, 0.7),      // Gray
            Substrate::Mud => (0.4, 0.3, 0.2),        // Dark brown
            Substrate::Water => (0.2, 0.4, 0.9),      // Blue
            Substrate::Brush => (0.2, 0.6, 0.15),    // Dark green, similar to forest
        }
    }
}

impl Enemy {
    /// Create a new enemy with specified stats
    pub fn new(health: i32, attack: i32) -> Self {
        Self {
            health,
            attack,
            max_health: health,
        }
    }

    /// Check if enemy is defeated
    pub fn is_defeated(&self) -> bool {
        self.health <= 0
    }

    /// Restore enemy to full health (used when fleeing)
    pub fn restore_health(&mut self) {
        self.health = self.max_health;
    }
}
