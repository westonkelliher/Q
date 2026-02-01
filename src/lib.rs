pub mod types;
pub mod generation;
pub mod io;
pub mod display;
pub mod render;
pub mod terrain_view;
pub mod land_view;

#[cfg(test)]
mod tests;

// Re-export commonly used types for convenience
pub use types::{Biome, Land, Object, Substrate, Tile, World};
pub use generation::{
    determine_biome, generate_land_terrain, generate_world, initialize_world,
    LandBiomes, get_tile_biome,
};
pub use io::{load_world, save_world};
pub use display::{print_land, print_world};
pub use terrain_view::{TerrainCamera, render as render_terrain_view, handle_input as handle_terrain_input};
pub use land_view::{LandCamera, render as render_land_view, handle_input as handle_land_input};
