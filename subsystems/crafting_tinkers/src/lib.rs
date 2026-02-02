pub mod ids;
pub mod quality;
pub mod world_object;
pub mod materials;
pub mod item_def;
pub mod recipe;
pub mod instance;
pub mod provenance;
pub mod registry;
pub mod content;
pub mod cli;

// Re-export commonly used types
pub use ids::{
    CraftingStationId, ItemId, ItemInstanceId, RecipeId, ResourceNodeId,
    WorldObjectInstanceId, WorldObjectTag, MaterialId, SubmaterialId, ComponentKindId,
};
pub use materials::{Material, Submaterial, ComponentKind};
pub use instance::{ComponentInstance, ItemInstance, SimpleInstance, CompositeInstance};
pub use item_def::{ItemDefinition, ItemKind, CompositeDef, CompositeSlot, CompositeCategory, ToolType};
pub use provenance::{ConsumedInput, Provenance};
pub use quality::Quality;
pub use recipe::{
    SimpleRecipe, ComponentRecipe, CompositeRecipe, SimpleInput,
    ToolRequirement, WorldObjectRequirement,
};
pub use registry::Registry;
pub use world_object::{WorldObjectKind, WorldObjectInstance};
