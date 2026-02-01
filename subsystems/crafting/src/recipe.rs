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
/// 
/// This structure is recursive - it can express requirements on an item's
/// provenance chain to arbitrary depth.
/// 
/// # Example: "A heart from a wolf slain with a manasteel-bladed weapon"
/// 
/// ```ignore
/// MaterialInput {
///     required_tags: vec![MaterialTag("heart".into())],
///     provenance_reqs: Some(Box::new(ProvenanceRequirements {
///         world_object: Some(MaterialInput {
///             required_tags: vec![MaterialTag("wolf_carcass".into())],
///             provenance_reqs: Some(Box::new(ProvenanceRequirements {
///                 tool: Some(MaterialInput {
///                     required_tags: vec![MaterialTag("weapon".into())],
///                     component_reqs: vec![
///                         ComponentRequirement {
///                             slot_name: "blade".into(),
///                             required_material_tags: vec![MaterialTag("manasteel".into())],
///                         }
///                     ],
///                     ..Default::default()
///                 }),
///                 ..Default::default()
///             })),
///             ..Default::default()
///         }),
///         ..Default::default()
///     })),
///     ..Default::default()
/// }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialInput {
    /// Specific item required, OR use required_tags for any matching item
    pub item_id: Option<ItemId>,
    
    /// Required tags - material must have ALL these tags
    pub required_tags: Vec<MaterialTag>,
    
    pub quantity: u32,
    pub min_quality: Option<Quality>,
    
    /// For multi-component outputs: which component slot this input fills.
    /// If None, this input is consumed but doesn't fill a specific slot
    /// (e.g., fuel for smelting, or simple single-material items).
    pub fills_slot: Option<String>,
    
    /// Requirements on specific components of this item (for multi-part items)
    /// e.g., require the "blade" component to be made of "manasteel"
    pub component_reqs: Vec<ComponentRequirement>,
    
    /// Requirements on this item's provenance (how it was made)
    /// This enables recursive queries like "made with a tool that was made with..."
    pub provenance_reqs: Option<Box<ProvenanceRequirements>>,
}

/// Requirements on a specific component slot of a multi-part item
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentRequirement {
    /// Name of the component slot (e.g., "blade", "handle", "head")
    pub slot_name: String,
    /// Required material tags for this component
    pub required_material_tags: Vec<MaterialTag>,
}

/// Requirements on an item's provenance chain
/// 
/// Used to express constraints like "this item must have been made using X"
/// where X can itself have provenance requirements (recursive).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProvenanceRequirements {
    /// Requirements on materials that were consumed to create this item
    pub consumed_inputs: Vec<MaterialInput>,
    
    /// Requirements on the tool used to create this item
    /// (Tools are items, so this is recursive)
    pub tool: Option<MaterialInput>,
    
    /// Requirements on the world object used to create this item
    /// (Placed items are items, so this is recursive)
    pub world_object: Option<MaterialInput>,
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
