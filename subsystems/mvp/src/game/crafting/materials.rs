use super::ids::{MaterialId, SubmaterialId, ComponentKindId};

/// Broad material category (e.g., leather, wood, metal)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub description: String,
}

/// Specific variant of a material - these correspond to actual Simple items
/// (e.g., deer_leather, oak_wood, iron_metal)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Submaterial {
    pub id: SubmaterialId,
    pub material: MaterialId,  // parent category
    pub name: String,
    pub description: String,
    // Future: properties, stat modifiers, etc.
}

/// Defines a type of component that can be crafted from submaterials
/// (e.g., handle, binding, scimitar_blade)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentKind {
    pub id: ComponentKindId,
    pub name: String,
    pub description: String,
    /// Which materials (broad categories) this component can be made from (OR logic)
    pub accepted_materials: Vec<MaterialId>,
    /// What this component can substitute for in non-crafting scenarios
    /// e.g., knife_blade with makeshift_tags: ["knife"] can act as a makeshift knife
    pub makeshift_tags: Vec<String>,
}
