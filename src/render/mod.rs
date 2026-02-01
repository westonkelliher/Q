use crate::types::{Biome, Object, Substrate};

/// Error type for rendering operations
#[derive(Debug)]
pub enum RenderError {
    InitializationFailed(String),
    RenderingFailed(String),
    Other(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::InitializationFailed(msg) => write!(f, "Initialization failed: {}", msg),
            RenderError::RenderingFailed(msg) => write!(f, "Rendering failed: {}", msg),
            RenderError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

/// Input key representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Escape,
    Q,
    W,
    A,
    S,
    D,
    Z,
    X,
    Other(char),
}

/// Trait for abstracting graphics rendering operations
/// This allows swapping between different rendering backends (macroquad, Bevy, etc.)
pub trait Renderer {
    /// Initialize the renderer (create window, set up graphics context, etc.)
    fn init(&mut self) -> Result<(), RenderError>;

    /// Clear the screen with the given color
    fn clear(&mut self, color: Color);

    /// Draw a tile at the given position with the specified size
    /// The tile should show the substrate as the base and objects on top
    fn draw_tile(&mut self, x: f32, y: f32, size: f32, substrate: &Substrate, objects: &[Object]);

    /// Draw a biome overview tile at the given position with the specified size
    fn draw_biome_overview(&mut self, x: f32, y: f32, size: f32, biome: &Biome);

    /// Draw a biome overview tile with colored borders based on edge/corner biomes
    /// The center area uses the center biome, borders use edge biomes, corners use corner biomes
    fn draw_biome_overview_with_borders(
        &mut self,
        x: f32,
        y: f32,
        size: f32,
        center: &Biome,
        top: &Biome,
        bottom: &Biome,
        left: &Biome,
        right: &Biome,
        top_left: &Biome,
        top_right: &Biome,
        bottom_left: &Biome,
        bottom_right: &Biome,
        border_width: f32,
    );

    /// Draw a selection indicator (highlight) at the given position and size
    fn draw_selection_indicator(&mut self, x: f32, y: f32, size: f32);

    /// Draw a grid overlay
    /// x, y: top-left corner of the grid
    /// width, height: total size of the grid area
    /// rows, cols: number of grid cells
    fn draw_grid(&mut self, x: f32, y: f32, width: f32, height: f32, rows: usize, cols: usize);

    /// Present the rendered frame to the screen
    fn present(&mut self) -> Result<(), RenderError>;

    /// Check if the window should close
    fn should_close(&self) -> bool;

    /// Get the current mouse position in world coordinates (if available)
    fn get_mouse_pos(&self) -> Option<(f32, f32)>;

    /// Get all currently pressed keys
    fn get_keys_pressed(&self) -> Vec<Key>;

    /// Get the window dimensions
    fn window_size(&self) -> (f32, f32);
}

// Re-export implementations
pub mod macroquad;

// Future Bevy implementation
// #[cfg(feature = "graphics-bevy")]
// pub mod bevy;
