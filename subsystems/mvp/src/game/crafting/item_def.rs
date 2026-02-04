use super::ids::{ItemId, SubmaterialId, ComponentKindId};
use super::world_object::WorldObjectKind;

/// Stat bonuses granted by an item when equipped
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct StatBonuses {
    pub health: i32,
    pub attack: i32,
}

/// Defines what an item IS - its template/blueprint
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
    /// If this item can be placed as a world object, specifies what kind
    pub placeable: Option<WorldObjectKind>,
    /// Whether this item can be picked up from the world (false for trees, boulders, etc.)
    pub pickupable: bool,
    /// Stat bonuses granted when equipped
    pub stat_bonuses: StatBonuses,
}

/// The kind of item - mutually exclusive categories
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemKind {
    /// Raw materials, consumables, creatures, resource nodes
    /// If this is a submaterial item, include the submaterial ID
    Simple { 
        submaterial: Option<SubmaterialId> 
    },
    
    /// Parts made from submaterials, used to build composites
    Component { 
        component_kind: ComponentKindId 
    },
    
    /// Final assembled items (tools, weapons, armor)
    Composite(CompositeDef),
}

/// Definition of a composite item (assembled from components)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeDef {
    pub slots: Vec<CompositeSlot>,
    pub category: CompositeCategory,
    pub tool_type: Option<ToolType>,
}

/// A slot in a composite item that accepts a specific component kind
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeSlot {
    pub name: String,                     // e.g., "blade", "handle", "binding"
    pub component_kind: ComponentKindId,  // which component type fits here
}

/// Category of composite item
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompositeCategory {
    Tool,
    Weapon,
    Armor,
}

/// Type of tool
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ToolType {
    Pickaxe,
    Axe,
    Hatchet,
    Hammer,
    Knife,
    Saw,
    Needle,
    /// Extensible via string variant for LLM generation
    Custom(String),
}
