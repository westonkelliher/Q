use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Substrate {
    Grass,
    Dirt,
    Stone,
    Mud,
    Water,
    Brush,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    Rock,
    Tree,
    Stick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub substrate: Substrate,
    pub objects: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Biome {
    Forest,
    Meadow,
    Lake,
    Mountain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Land {
    pub tiles: [[Tile; 8]; 8],
    pub biome: Biome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    #[serde(serialize_with = "crate::io::serialize_terrain", deserialize_with = "crate::io::deserialize_terrain")]
    pub terrain: HashMap<(i32, i32), Land>,
}

impl Biome {
    pub fn to_chars(&self) -> &str {
        match self {
            Biome::Forest => " Y",
            Biome::Meadow => " .",
            Biome::Lake => "~~",
            Biome::Mountain => "/\\",
        }
    }
}

impl Substrate {
    pub fn to_char(&self) -> char {
        match self {
            Substrate::Grass => 'g',
            Substrate::Dirt => 'd',
            Substrate::Stone => 's',
            Substrate::Mud => 'm',
            Substrate::Water => 'w',
            Substrate::Brush => 'b',
        }
    }
}

impl Object {
    pub fn to_char(&self) -> char {
        match self {
            Object::Rock => 'R',
            Object::Tree => 'T',
            Object::Stick => 'S',
        }
    }
}
