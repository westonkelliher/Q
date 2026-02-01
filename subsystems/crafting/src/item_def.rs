use crate::ids::{ItemId, MaterialTag};

/// Defines what an item IS - its template/blueprint
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    
    /// Component slots for multi-part items (Tinker's style)
    /// Empty for simple items like "iron_ore"
    pub component_slots: Vec<ComponentSlot>,
    
    /// What categories this item belongs to
    pub categories: ItemCategories,
    
    /// If this is a material, what tags does it have?
    pub material_tags: Vec<MaterialTag>,
    
    /// If this is a tool, what tool type is it?
    pub tool_type: Option<ToolType>,
    
    /// Special properties this item/material grants
    pub inherent_properties: Vec<Property>,
}

/// A component slot in a multi-part item
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentSlot {
    pub name: String,           // e.g., "blade", "handle", "pommel"
    pub required_tags: Vec<MaterialTag>, // material must have ALL these tags
    pub optional_tags: Vec<MaterialTag>, // bonus if material has these
}

/// Categories an item can belong to (non-exclusive)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemCategories {
    pub is_material: bool,
    pub is_tool: bool,
    pub is_placeable: bool,
    pub is_consumable: bool,
}

/// Type of tool
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ToolType {
    Pickaxe,
    Axe,
    Hammer,
    Knife,
    Saw,
    Needle,
    /// Extensible via string variant for LLM generation
    Custom(String),
}

/// A special property that can be granted by materials or items
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Property {
    pub id: String,
    pub name: String,
    pub description: String,
}
