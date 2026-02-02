//! 8x8 subsystem - Base for various systems that involve the land view
//!
//! Provides an 8x8 grid of tiles, where each tile has a color and a vector of strings.

use std::ops::{Index, IndexMut};

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color with RGBA values (0.0-1.0)
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color with RGB values (alpha defaults to 1.0)
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }
}

/// A single tile in the 8x8 grid
#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    /// The color of the tile
    pub color: Color,
    /// A vector of strings associated with this tile
    pub strings: Vec<String>,
}

impl Tile {
    /// Create a new tile with a color and empty strings vector
    pub fn new(color: Color) -> Self {
        Self {
            color,
            strings: Vec::new(),
        }
    }

    /// Create a new tile with a color and initial strings
    pub fn with_strings(color: Color, strings: Vec<String>) -> Self {
        Self { color, strings }
    }
}

/// An 8x8 grid of tiles
#[derive(Debug, Clone, PartialEq)]
pub struct Grid8x8 {
    tiles: [[Tile; 8]; 8],
}

impl Grid8x8 {
    /// Create a new 8x8 grid with all tiles initialized to the default color
    pub fn new(default_color: Color) -> Self {
        Self {
            tiles: std::array::from_fn(|_| std::array::from_fn(|_| Tile::new(default_color))),
        }
    }

    /// Get a reference to the tile at the given coordinates
    /// Returns None if coordinates are out of bounds
    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < 8 && y < 8 {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    /// Get a mutable reference to the tile at the given coordinates
    /// Returns None if coordinates are out of bounds
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < 8 && y < 8 {
            Some(&mut self.tiles[y][x])
        } else {
            None
        }
    }

    /// Set the tile at the given coordinates
    /// Returns true if successful, false if coordinates are out of bounds
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) -> bool {
        if x < 8 && y < 8 {
            self.tiles[y][x] = tile;
            true
        } else {
            false
        }
    }

    /// Set the color of a tile at the given coordinates
    /// Returns true if successful, false if coordinates are out of bounds
    pub fn set_color(&mut self, x: usize, y: usize, color: Color) -> bool {
        if let Some(tile) = self.get_mut(x, y) {
            tile.color = color;
            true
        } else {
            false
        }
    }

    /// Add a string to the tile at the given coordinates
    /// Returns true if successful, false if coordinates are out of bounds
    pub fn add_string(&mut self, x: usize, y: usize, string: String) -> bool {
        if let Some(tile) = self.get_mut(x, y) {
            tile.strings.push(string);
            true
        } else {
            false
        }
    }

    /// Clear all strings from a tile at the given coordinates
    /// Returns true if successful, false if coordinates are out of bounds
    pub fn clear_strings(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_mut(x, y) {
            tile.strings.clear();
            true
        } else {
            false
        }
    }

    /// Get the width of the grid (always 8)
    pub fn width(&self) -> usize {
        8
    }

    /// Get the height of the grid (always 8)
    pub fn height(&self) -> usize {
        8
    }
}

impl Index<(usize, usize)> for Grid8x8 {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < 8 && y < 8, "Index out of bounds: ({}, {})", x, y);
        &self.tiles[y][x]
    }
}

impl IndexMut<(usize, usize)> for Grid8x8 {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < 8 && y < 8, "Index out of bounds: ({}, {})", x, y);
        &mut self.tiles[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(1.0, 0.5, 0.0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_tile_creation() {
        let color = Color::rgb(0.5, 0.5, 0.5);
        let tile = Tile::new(color);
        assert_eq!(tile.color, color);
        assert!(tile.strings.is_empty());
    }

    #[test]
    fn test_tile_with_strings() {
        let color = Color::rgb(1.0, 0.0, 0.0);
        let strings = vec!["hello".to_string(), "world".to_string()];
        let tile = Tile::with_strings(color, strings.clone());
        assert_eq!(tile.color, color);
        assert_eq!(tile.strings, strings);
    }

    #[test]
    fn test_grid_creation() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let grid = Grid8x8::new(default_color);
        assert_eq!(grid.width(), 8);
        assert_eq!(grid.height(), 8);
    }

    #[test]
    fn test_grid_get() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let grid = Grid8x8::new(default_color);
        
        let tile = grid.get(0, 0).unwrap();
        assert_eq!(tile.color, default_color);
        
        assert!(grid.get(8, 0).is_none());
        assert!(grid.get(0, 8).is_none());
    }

    #[test]
    fn test_grid_set() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let mut grid = Grid8x8::new(default_color);
        
        let new_color = Color::rgb(1.0, 0.0, 0.0);
        let new_tile = Tile::new(new_color);
        
        assert!(grid.set(3, 4, new_tile.clone()));
        assert_eq!(grid.get(3, 4).unwrap(), &new_tile);
        
        assert!(!grid.set(8, 0, new_tile));
    }

    #[test]
    fn test_grid_index() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let mut grid = Grid8x8::new(default_color);
        
        let new_color = Color::rgb(0.0, 1.0, 0.0);
        grid[(2, 3)].color = new_color;
        
        assert_eq!(grid[(2, 3)].color, new_color);
    }

    #[test]
    fn test_grid_set_color() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let mut grid = Grid8x8::new(default_color);
        
        let new_color = Color::rgb(0.0, 0.0, 1.0);
        assert!(grid.set_color(5, 6, new_color));
        assert_eq!(grid.get(5, 6).unwrap().color, new_color);
        
        assert!(!grid.set_color(8, 0, new_color));
    }

    #[test]
    fn test_grid_add_string() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let mut grid = Grid8x8::new(default_color);
        
        assert!(grid.add_string(1, 2, "test".to_string()));
        assert_eq!(grid.get(1, 2).unwrap().strings.len(), 1);
        assert_eq!(grid.get(1, 2).unwrap().strings[0], "test");
        
        assert!(grid.add_string(1, 2, "another".to_string()));
        assert_eq!(grid.get(1, 2).unwrap().strings.len(), 2);
        
        assert!(!grid.add_string(8, 0, "fail".to_string()));
    }

    #[test]
    fn test_grid_clear_strings() {
        let default_color = Color::rgb(0.0, 0.0, 0.0);
        let mut grid = Grid8x8::new(default_color);
        
        grid.add_string(0, 0, "one".to_string());
        grid.add_string(0, 0, "two".to_string());
        assert_eq!(grid.get(0, 0).unwrap().strings.len(), 2);
        
        assert!(grid.clear_strings(0, 0));
        assert_eq!(grid.get(0, 0).unwrap().strings.len(), 0);
        
        assert!(!grid.clear_strings(8, 0));
    }
}
