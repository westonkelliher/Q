use std::collections::HashMap;
use crate::ids::{ItemId, ItemInstanceId};
use crate::quality::Quality;
use crate::provenance::Provenance;

/// A specific instance of an item in the game
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemInstance {
    pub id: ItemInstanceId,     // Unique runtime ID
    pub definition: ItemId,     // What item this is
    pub quality: Quality,
    
    /// For multi-component items: what material fills each slot
    /// Key is slot name, value is the component instance
    pub components: HashMap<String, ComponentInstance>,
    
    /// How this item was created
    pub provenance: Provenance,
}

/// A component instance - what material was used to fill a slot
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComponentInstance {
    pub slot_name: String,
    pub material_used: ItemId,
    pub material_quality: Quality,
}
