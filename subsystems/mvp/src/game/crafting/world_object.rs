use super::ids::{CraftingStationId, ResourceNodeId, WorldObjectInstanceId, WorldObjectTag};

/// World objects that can be part of a construction requirement
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WorldObjectKind {
    ResourceNode(ResourceNodeId),   // e.g., iron_ore_boulder, oak_tree
    CraftingStation(CraftingStationId), // e.g., anvil, magic_imbuing_altar
}

/// A world object instance placed in the world
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldObjectInstance {
    pub id: WorldObjectInstanceId,
    pub kind: WorldObjectKind,
    pub tags: Vec<WorldObjectTag>,
}
