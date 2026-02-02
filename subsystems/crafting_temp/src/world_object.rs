use crate::ids::{CraftingStationId, ResourceNodeId};

/// World objects that can be part of a construction requirement
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WorldObjectKind {
    ResourceNode(ResourceNodeId),   // e.g., iron_ore_boulder, oak_tree
    CraftingStation(CraftingStationId), // e.g., anvil, magic_imbuing_altar
}
