use crate::game::crafting::{CraftingRegistry, ItemInstanceId, ItemInstance};
use crate::game::game_state::GameState;

/// Convert an item instance ID to its display name
pub fn get_item_name(registry: &CraftingRegistry, instance_id: ItemInstanceId) -> Option<String> {
    registry.get_instance(instance_id)
        .and_then(|instance| {
            match instance {
                ItemInstance::Simple(s) => {
                    registry.get_item(&s.definition)
                        .map(|def| def.name.clone())
                }
                ItemInstance::Component(c) => {
                    registry.get_component_kind(&c.component_kind)
                        .map(|ck| ck.name.clone())
                }
                ItemInstance::Composite(c) => {
                    registry.get_item(&c.definition)
                        .map(|def| def.name.clone())
                }
            }
        })
}

/// Serialize a list of instance IDs to their display names
pub fn serialize_item_list(registry: &CraftingRegistry, instance_ids: &[ItemInstanceId]) -> Vec<String> {
    instance_ids.iter()
        .filter_map(|id| get_item_name(registry, *id))
        .collect()
}

/// Serialize the character's inventory to display names
pub fn serialize_inventory(state: &GameState) -> Vec<String> {
    serialize_item_list(&state.crafting_registry, &state.character.get_inventory().items)
}

/// Serialize the character's equipped item to its display name
pub fn serialize_equipped(state: &GameState) -> Option<String> {
    state.character.get_equipped()
        .and_then(|equipped_id| get_item_name(&state.crafting_registry, equipped_id))
}
