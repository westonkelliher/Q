use crate::ids::{ItemInstanceId, RecipeId};
use crate::world_object::WorldObjectKind;

/// Tracks how an item was created - immediate inputs only
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Provenance {
    /// Recipe used to create this item
    pub recipe_id: RecipeId,
    
    /// The actual item instances consumed as inputs
    pub consumed_inputs: Vec<ConsumedInput>,
    
    /// Tool used (if any) - reference, not consumed
    pub tool_used: Option<ItemInstanceId>,
    
    /// World object used (if any)
    pub world_object_used: Option<WorldObjectKind>,
    
    /// When this was crafted (Unix timestamp)
    pub crafted_at: i64,
}

/// A consumed input reference
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConsumedInput {
    pub instance_id: ItemInstanceId,
    pub quantity: u32,
}
