use crate::ids::{ItemId, RecipeId, ComponentKindId, WorldObjectTag};
use crate::item_def::ToolType;
use crate::quality::Quality;
use crate::world_object::WorldObjectKind;

//==============================================================================
// SIMPLE RECIPES
//==============================================================================

/// Recipe to create a Simple item (mining, harvesting, smelting, cooking)
/// Inputs are other Simple items, output is a Simple item
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SimpleRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,
    pub output_quantity: u32,
    pub inputs: Vec<SimpleInput>,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
}

/// A simple input requirement - specific item ID and quantity
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SimpleInput {
    pub item_id: ItemId,
    pub quantity: u32,
}

//==============================================================================
// COMPONENT RECIPES
//==============================================================================

/// Recipe to craft a Component from a submaterial
/// Input is implicit: one submaterial item whose material is in ComponentKind.accepted_materials
/// Output is a Component instance that tracks the submaterial used
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComponentRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ComponentKindId,
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Note: Input validation happens at craft time:
    // - Must be a Simple item with a submaterial
    // - Submaterial's parent material must be in ComponentKind.accepted_materials
}

//==============================================================================
// COMPOSITE RECIPES
//==============================================================================

/// Recipe to assemble a Composite from components
/// Inputs are implicit: whatever ComponentKinds the composite's slots require
/// System validates that provided components match the required slots
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompositeRecipe {
    pub id: RecipeId,
    pub name: String,
    pub output: ItemId,  // Must be a Composite ItemDefinition
    pub tool: Option<ToolRequirement>,
    pub world_object: Option<WorldObjectRequirement>,
    // Note: Input validation happens at craft time:
    // - Must provide one component for each slot in the Composite definition
    // - Each component's ComponentKind must match the slot's required ComponentKind
}

//==============================================================================
// SHARED REQUIREMENT TYPES
//==============================================================================

/// Requirement for a tool in a recipe
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToolRequirement {
    pub tool_type: ToolType,
    pub min_quality: Quality,
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
