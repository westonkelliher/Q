pub mod world;
pub mod game_state;
pub mod character;
pub mod combat;

// Re-export commonly used types for convenience
pub use world::{Biome, Land, Object, Substrate, Tile, World};
pub use world::types::Enemy;
pub use world::create_hardcoded_world;
pub use game_state::{GameState, ViewMode, DisplayOverlay};
pub use character::{Character, CharacterEmoji};
pub use combat::{Combatant, CombatState, CombatResult};
