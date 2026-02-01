/// String-based ID for LLM-friendly definition
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ItemId(pub String); // e.g., "scimitar", "iron_ore"

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RecipeId(pub String); // e.g., "forge_scimitar_blade"

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MaterialTag(pub String); // e.g., "metal", "wood", "magical"

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WorldObjectTag(pub String); // e.g., "high_heat", "water_source", "magical"

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ResourceNodeId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CraftingStationId(pub String);

/// Unique ID for a specific item instance
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ItemInstanceId(pub u64);

/// Unique ID for a specific world object instance (resource node or crafting station)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WorldObjectInstanceId(pub u64);
