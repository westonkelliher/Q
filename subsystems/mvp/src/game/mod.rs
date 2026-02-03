pub mod world;
pub mod game_state;
pub mod character;

// Re-export commonly used types for convenience
pub use world::{Biome, Land, Object, Substrate, Tile, World};
pub use world::create_hardcoded_world;
pub use game_state::{GameState, ViewMode};
pub use character::Character;
