use std::collections::HashMap;
use crate::ids::{ItemId, ItemInstanceId, SubmaterialId, ComponentKindId};
use crate::quality::Quality;
use crate::provenance::Provenance;

/// A specific instance of an item in the game
/// Every instance is exactly ONE of: Simple, Component, or Composite
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemInstance {
    Simple(SimpleInstance),
    Component(ComponentInstance),
    Composite(CompositeInstance),
}

impl ItemInstance {
    /// Get the instance ID regardless of type
    pub fn id(&self) -> ItemInstanceId {
        match self {
            ItemInstance::Simple(s) => s.id,
            ItemInstance::Component(c) => c.id,
            ItemInstance::Composite(c) => c.id,
        }
    }

    /// Get the provenance regardless of type
    pub fn provenance(&self) -> &Provenance {
        match self {
            ItemInstance::Simple(s) => &s.provenance,
            ItemInstance::Component(c) => &c.provenance,
            ItemInstance::Composite(c) => &c.provenance,
        }
    }
}

//==============================================================================
// SIMPLE INSTANCE
//==============================================================================

/// Instance of a Simple item (raw materials, consumables, creatures, etc.)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SimpleInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,      // References ItemDefinition with ItemKind::Simple
    pub provenance: Provenance,
}

//==============================================================================
// COMPONENT INSTANCE
//==============================================================================

/// Instance of a Component - tracks which submaterial was used to make it
/// Components are parts made from submaterials, used to build composites
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentInstance {
    pub id: ItemInstanceId,
    pub component_kind: ComponentKindId,  // e.g., "handle", "binding", "scimitar_blade"
    pub submaterial: SubmaterialId,       // e.g., "oak_wood", "deer_leather", "iron_metal"
    pub provenance: Provenance,
}

//==============================================================================
// COMPOSITE INSTANCE
//==============================================================================

/// Instance of a Composite - tracks which components were used to assemble it
/// Composites are final assembled items (tools, weapons, armor)
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeInstance {
    pub id: ItemInstanceId,
    pub definition: ItemId,  // References ItemDefinition with ItemKind::Composite
    pub quality: Quality,    // TODO: Calculate from components, currently fixed to Common
    /// Map of slot name to the component that fills it
    /// Example: {"blade": ComponentInstance, "handle": ComponentInstance, "binding": ComponentInstance}
    pub components: HashMap<String, ComponentInstance>,
    pub provenance: Provenance,
}
