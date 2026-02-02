use crate::ids::{ItemId, SubmaterialId, ComponentKindId};

/// Defines what an item IS - its template/blueprint
/// Every item is exactly ONE of: Simple, Component, or Composite
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
}

/// The kind of item - mutually exclusive categories
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemKind {
    /// Raw materials, consumables, creatures, resource nodes
    /// If this is a submaterial item, include the submaterial ID
    /// Examples:
    /// - Simple { submaterial: Some("deer_leather") } - a leather material
    /// - Simple { submaterial: Some("oak_wood") } - a wood material
    /// - Simple { submaterial: None } - cooked_meat, wolf, copper_boulder
    Simple {
        submaterial: Option<SubmaterialId>,
    },

    /// Parts made from submaterials, used exclusively to build composites
    /// Examples: handle, binding, scimitar_blade, pickaxe_head
    /// Each component instance will track which specific submaterial was used
    Component {
        component_kind: ComponentKindId,
    },

    /// Final assembled items (tools, weapons, armor)
    /// Built from multiple components
    /// Examples: scimitar (blade + handle + binding), pickaxe (head + handle)
    Composite(CompositeDef),
}

/// Definition of a composite item's structure
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeDef {
    /// Slots that must be filled with components
    /// Example for scimitar: [blade: scimitar_blade, handle: handle, binding: binding]
    pub slots: Vec<CompositeSlot>,

    /// Category of composite item
    pub category: CompositeCategory,

    /// If this is a tool, what tool type is it?
    pub tool_type: Option<ToolType>,
}

/// A slot in a composite item that must be filled with a specific component kind
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeSlot {
    pub name: String,                       // "blade", "handle", "binding", "head", "pommel", etc.
    pub component_kind: ComponentKindId,    // which component type fits here
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
