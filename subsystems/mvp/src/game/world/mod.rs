pub mod types;
pub mod world;

// Re-export commonly used types for convenience
pub use types::{Biome, Land, Substrate, Tile, World, Enemy};
pub use world::create_hardcoded_world;
