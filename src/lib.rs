pub mod types;
pub mod generation;
pub mod io;
pub mod display;
pub mod render;
pub mod graphics;

#[cfg(test)]
mod tests;

// Re-export commonly used types for convenience
pub use types::{Biome, Land, Object, Substrate, Tile, World};
pub use generation::{determine_biome, generate_land_terrain, generate_world, initialize_world};
pub use io::{load_world, save_world};
pub use display::{print_land, print_world};
pub use graphics::{Camera, ViewMode, handle_input, render_land, render_land_view, render_terrain_view, render_tile};
