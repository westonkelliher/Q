use std::collections::HashMap;
use crate::ids::{ItemId, ItemInstanceId, ComponentKindId, SubmaterialId};
use crate::quality::Quality;
use crate::provenance::Provenance;

/// Instance of a Simple item (including submaterials).
///
/// Simple items include:
/// - Raw submaterial items (deer_leather, oak_wood, iron_bar)
/// - Consumables (cooked_meat, health_potion)
/// - Creatures (wolf, deer)
/// - Resource nodes (copper_boulder, oak_tree)
/// - Carcasses (wolf_carcass)
///
/// Simple items are the base tier of the crafting system and can be used as inputs
/// to ComponentRecipes.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SimpleInstance {
    /// Unique runtime ID for this instance
    pub id: ItemInstanceId,
    /// Reference to the item definition (what type of item this is)
    pub definition: ItemId,
    /// How this item was created (recipe, inputs, tools, etc.)
    pub provenance: Provenance,
}

/// Instance of a Component - tracks which submaterial was used.
///
/// Components are crafted parts made from submaterials. They are used exclusively
/// as inputs to CompositeRecipes to build final items like tools and weapons.
///
/// Each component instance tracks:
/// - The component kind (e.g., "handle", "binding", "scimitar_blade")
/// - The specific submaterial used (e.g., "deer_leather", "oak_wood")
/// - Complete provenance chain back to the source submaterial
///
/// # Example
/// A handle component crafted from oak_wood will have:
/// - component_kind: "handle"
/// - submaterial: "oak_wood"
/// - provenance: tracks the recipe and consumed oak_wood instance
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentInstance {
    /// Unique runtime ID for this instance
    pub id: ItemInstanceId,
    /// The type of component this is (e.g., "handle", "binding", "blade")
    pub component_kind: ComponentKindId,
    /// The specific submaterial used to craft this component (e.g., "deer_leather", "oak_wood")
    pub submaterial: SubmaterialId,
    /// How this component was created (recipe, input submaterial, tools, etc.)
    pub provenance: Provenance,
}

/// Instance of a Composite - tracks which components were used.
///
/// Composites are final assembled items (tools, weapons, armor) crafted by combining
/// multiple components. Each composite has slots that accept specific component kinds.
///
/// # Quality Calculation
/// Quality is currently defaulted to Common. Future implementation will calculate
/// quality based on component submaterials and crafting conditions.
///
/// # Example
/// A scimitar composite will have:
/// - definition: "scimitar"
/// - components: {
///     "blade" -> ComponentInstance(scimitar_blade from steel_metal),
///     "handle" -> ComponentInstance(handle from oak_wood),
///     "binding" -> ComponentInstance(binding from deer_leather)
///   }
/// - quality: Common (TODO: calculate from components)
/// - provenance: tracks the recipe and all consumed component instances
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompositeInstance {
    /// Unique runtime ID for this instance
    pub id: ItemInstanceId,
    /// Reference to the composite item definition (what type of composite this is)
    pub definition: ItemId,
    /// Quality of this composite (default: Common, TODO: calculate from components)
    pub quality: Quality,
    /// Map of slot names to the component instances used in each slot
    pub components: HashMap<String, ComponentInstance>,
    /// How this composite was created (recipe, component inputs, tools, etc.)
    pub provenance: Provenance,
}

/// Unified item instance that can be any of the three instance types.
///
/// The crafting system uses this enum to handle all instance types uniformly
/// while preserving type-specific information. Helper methods provide common
/// access to shared fields like id and provenance.
///
/// # Three-Tier System
/// Items are exactly ONE of:
/// 1. Simple - raw materials and standalone items
/// 2. Component - crafted parts made from submaterials
/// 3. Composite - final items assembled from components
///
/// This enforces a strict crafting flow:
/// ```text
/// Submaterial (Simple) → Component → Composite
/// ```
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemInstance {
    /// A simple item instance
    Simple(SimpleInstance),
    /// A component instance
    Component(ComponentInstance),
    /// A composite instance
    Composite(CompositeInstance),
}

impl ItemInstance {
    /// Get the unique instance ID regardless of instance type
    pub fn id(&self) -> ItemInstanceId {
        match self {
            ItemInstance::Simple(i) => i.id,
            ItemInstance::Component(i) => i.id,
            ItemInstance::Composite(i) => i.id,
        }
    }

    /// Get the provenance regardless of instance type
    pub fn provenance(&self) -> &Provenance {
        match self {
            ItemInstance::Simple(i) => &i.provenance,
            ItemInstance::Component(i) => &i.provenance,
            ItemInstance::Composite(i) => &i.provenance,
        }
    }
}
