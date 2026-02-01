use crate::ids::{ItemId, MaterialTag, RecipeId, WorldObjectTag};
use crate::item_def::ToolType;
use crate::quality::Quality;
use crate::world_object::WorldObjectKind;

/// A recipe is a named construction that produces an item
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: String,
    pub construction: Construction,
    pub output: RecipeOutput,
}

/// How to construct an item
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Construction {
    /// Tool required (with minimum quality)
    pub tool: Option<ToolRequirement>,
    
    /// World object required (resource node or crafting station)
    pub world_object: Option<WorldObjectRequirement>,
    
    /// Material inputs consumed
    pub material_inputs: Vec<MaterialInput>,
}

/// A world object requirement for a construction
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorldObjectRequirement {
    /// Specific world object kind required, OR use required_tags for any matching
    pub kind: Option<WorldObjectKind>,
    
    /// Required tags - world object must have ALL these tags
    /// e.g., ["high_heat"] matches forge, kiln, bonfire
    pub required_tags: Vec<WorldObjectTag>,
}

/// Requirement for a tool in a construction
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToolRequirement {
    pub tool_type: ToolType,
    pub min_quality: Quality,
}

/// A material input requirement for a construction
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialInput {
    /// Specific item required, OR use required_tags for any matching item
    pub item_id: Option<ItemId>,
    
    /// Required tags - material must have ALL these tags
    pub required_tags: Vec<MaterialTag>,
    
    pub quantity: u32,
    pub min_quality: Option<Quality>,
}

/// Output specification for a recipe
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecipeOutput {
    pub item_id: ItemId,
    pub quantity: u32,
    /// How input qualities affect output quality
    pub quality_formula: QualityFormula,
}

/// Formula for calculating output quality from input qualities
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum QualityFormula {
    /// Output = minimum quality of all inputs
    MinOfInputs,
    /// Output = average quality of all inputs (rounded down)
    AverageOfInputs,
    /// Output = weighted average based on component importance
    /// Vec of (component name or "default", weight)
    Weighted(Vec<(String, f32)>),
    /// Custom formula for complex cases (interpreted at runtime)
    Custom(String),
}
