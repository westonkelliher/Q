use std::collections::HashMap;
use crate::ids::{ItemId, ItemInstanceId, RecipeId};
use crate::instance::ItemInstance;
use crate::item_def::ItemDefinition;
use crate::recipe::Recipe;

/// Central registry for item definitions, recipes, and item instances
pub struct Registry {
    item_definitions: HashMap<ItemId, ItemDefinition>,
    recipes: HashMap<RecipeId, Recipe>,
    instances: HashMap<ItemInstanceId, ItemInstance>,
    next_instance_id: u64,
}

impl Registry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            item_definitions: HashMap::new(),
            recipes: HashMap::new(),
            instances: HashMap::new(),
            next_instance_id: 0,
        }
    }

    /// Register an item definition
    pub fn register_item(&mut self, item: ItemDefinition) {
        self.item_definitions.insert(item.id.clone(), item);
    }

    /// Register a recipe
    pub fn register_recipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register an item instance
    pub fn register_instance(&mut self, instance: ItemInstance) {
        self.instances.insert(instance.id, instance);
    }

    /// Get an item definition by ID
    pub fn get_item(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.item_definitions.get(id)
    }

    /// Get a recipe by ID
    pub fn get_recipe(&self, id: &RecipeId) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    /// Get an item instance by ID
    pub fn get_instance(&self, id: ItemInstanceId) -> Option<&ItemInstance> {
        self.instances.get(&id)
    }

    /// Generate a new unique instance ID
    pub fn next_instance_id(&mut self) -> ItemInstanceId {
        let id = ItemInstanceId(self.next_instance_id);
        self.next_instance_id += 1;
        id
    }

    /// Get all item definitions
    pub fn all_items(&self) -> impl Iterator<Item = &ItemDefinition> {
        self.item_definitions.values()
    }

    /// Get all recipes
    pub fn all_recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.values()
    }

    /// Get all item instances
    pub fn all_instances(&self) -> impl Iterator<Item = &ItemInstance> {
        self.instances.values()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
