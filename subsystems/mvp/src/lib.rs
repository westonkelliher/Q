pub mod game;
pub mod web;

// Re-export commonly used types for convenience
pub use game::{Biome, Land, Object, Substrate, Tile, World};
pub use game::create_hardcoded_world;
pub use game::{GameState, ViewMode};
pub use web::display::{print_land, print_world};
