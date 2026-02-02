use crate::ids::{ItemId, RecipeId, ComponentKindId, WorldObjectTag};
use crate::item_def::ToolType;
use crate::quality::Quality;
use crate::world_object::WorldObjectKind;

/// Recipe to create a Simple item (mining, harvesting, smelting, etc.)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SimpleRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub output_quantity: u32,
    pub inputs: Vec<SimpleInput>,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

/// Input for a simple recipe
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SimpleInput {
    pub item_id: ItemId,
    pub quantity: u32,
}

/// Recipe to craft a Component from a submaterial
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ComponentKindId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Input is implicit: one submaterial item whose material is in ComponentKind.accepted_materials
}

/// Recipe to assemble a Composite from components
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Inputs are implicit: whatever ComponentKinds the composite's slots require
}

/// A world object requirement for a recipe
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorldObjectRequirement {
    /// Specific world object kind required, OR use required_tags for any matching
    pub kind: Option<WorldObjectKind>,
    
    /// Required tags - world object must have ALL these tags
    /// e.g., ["high_heat"] matches forge, kiln, bonfire
    pub required_tags: Vec<WorldObjectTag>,
}

/// Requirement for a tool in a recipe
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToolRequirement {
    pub tool_type: ToolType,
    pub min_quality: Quality,
}
