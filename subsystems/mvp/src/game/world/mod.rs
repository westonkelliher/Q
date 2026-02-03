pub mod types;
pub mod camera;
pub mod terrain_view;
pub mod land_view;
pub mod world;

// Re-export commonly used types for convenience
pub use types::{Biome, Land, Object, Substrate, Tile, World, Enemy};
pub use world::create_hardcoded_world;
