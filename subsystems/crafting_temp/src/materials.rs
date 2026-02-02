use crate::ids::{MaterialId, SubmaterialId, ComponentKindId};

/// Broad material category (e.g., leather, wood, metal)
///
/// Materials are not items themselves - they are categories that submaterials belong to.
/// Items are made *of* submaterials, and submaterials belong to a material category.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub description: String,
}

/// Specific material variant - these correspond to actual Simple items
///
/// Examples: deer_leather, oak_wood, iron_metal
/// Each submaterial belongs to a parent material category.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Submaterial {
    pub id: SubmaterialId,
    /// Parent material category (e.g., "leather", "wood", "metal")
    pub material: MaterialId,
    pub name: String,
    pub description: String,
    // Future: properties, stat modifiers, etc.
}

/// Defines a type of component
///
/// Components are parts made from submaterials, used exclusively as inputs to composites.
/// Examples: handle, binding, scimitar_blade, pickaxe_head
///
/// Each component kind specifies which material categories it accepts.
/// When crafting a component, the submaterial's parent material must be in the accepted list.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ComponentKind {
    pub id: ComponentKindId,
    pub name: String,
    pub description: String,
    /// Which material categories this component accepts (OR logic)
    /// e.g., ["wood", "bone"] means the component can be made from any wood or bone submaterial
    pub accepted_materials: Vec<MaterialId>,
    /// Tags indicating what this component can substitute for in non-crafting scenarios
    /// e.g., ["knife"] means a knife_blade component can act as a makeshift knife
    /// This does NOT affect crafting recipes, only gameplay substitution
    pub makeshift_tags: Vec<String>,
}
