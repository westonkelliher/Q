pub mod world;
pub mod game_state;
pub mod character;
pub mod combat;
pub mod commands;
pub mod crafting;

// Re-export commonly used types for convenience
pub use world::{Biome, Land, Substrate, Tile, World};
pub use world::types::Enemy;
pub use world::create_hardcoded_world;
pub use game_state::{GameState, CurrentMode};
pub use character::Character;
pub use combat::CombatResult;
pub use commands::execute_command;
pub use crafting::{CraftingRegistry, ItemInstance, ItemInstanceId, ItemDefinition, ItemId, Quality};
