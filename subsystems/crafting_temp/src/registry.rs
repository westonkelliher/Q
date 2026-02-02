use std::collections::HashMap;
use crate::ids::{ItemId, ItemInstanceId, RecipeId, MaterialId, SubmaterialId, ComponentKindId};
use crate::instance::ItemInstance;
use crate::item_def::ItemDefinition;
use crate::materials::{Material, Submaterial, ComponentKind};
use crate::recipe::{SimpleRecipe, ComponentRecipe, CompositeRecipe};

/// Central registry for materials, items, recipes, and item instances
pub struct Registry {
    // Material system
    materials: HashMap<MaterialId, Material>,
    submaterials: HashMap<SubmaterialId, Submaterial>,
    component_kinds: HashMap<ComponentKindId, ComponentKind>,

    // Items and recipes
    item_definitions: HashMap<ItemId, ItemDefinition>,
    simple_recipes: HashMap<RecipeId, SimpleRecipe>,
    component_recipes: HashMap<RecipeId, ComponentRecipe>,
    composite_recipes: HashMap<RecipeId, CompositeRecipe>,

    // Runtime instances
    instances: HashMap<ItemInstanceId, ItemInstance>,
    next_instance_id: u64,
}

impl Registry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            submaterials: HashMap::new(),
            component_kinds: HashMap::new(),
            item_definitions: HashMap::new(),
            simple_recipes: HashMap::new(),
            component_recipes: HashMap::new(),
            composite_recipes: HashMap::new(),
            instances: HashMap::new(),
            next_instance_id: 0,
        }
    }

    //==========================================================================
    // MATERIAL SYSTEM REGISTRATION
    //==========================================================================

    /// Register a material (broad category like "leather", "wood", "metal")
    pub fn register_material(&mut self, material: Material) {
        self.materials.insert(material.id.clone(), material);
    }

    /// Register a submaterial (specific variant like "deer_leather", "oak_wood")
    pub fn register_submaterial(&mut self, submaterial: Submaterial) {
        self.submaterials.insert(submaterial.id.clone(), submaterial);
    }

    /// Register a component kind (component type like "handle", "binding")
    pub fn register_component_kind(&mut self, component_kind: ComponentKind) {
        self.component_kinds.insert(component_kind.id.clone(), component_kind);
    }

    //==========================================================================
    // ITEM AND RECIPE REGISTRATION
    //==========================================================================

    /// Register an item definition
    pub fn register_item(&mut self, item: ItemDefinition) {
        self.item_definitions.insert(item.id.clone(), item);
    }

    /// Register a simple recipe (Simple item → Simple item)
    pub fn register_simple_recipe(&mut self, recipe: SimpleRecipe) {
        self.simple_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register a component recipe (Submaterial → Component)
    pub fn register_component_recipe(&mut self, recipe: ComponentRecipe) {
        self.component_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register a composite recipe (Components → Composite)
    pub fn register_composite_recipe(&mut self, recipe: CompositeRecipe) {
        self.composite_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register an item instance
    pub fn register_instance(&mut self, instance: ItemInstance) {
        self.instances.insert(instance.id(), instance);
    }

    //==========================================================================
    // GETTERS
    //==========================================================================

    pub fn get_material(&self, id: &MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    pub fn get_submaterial(&self, id: &SubmaterialId) -> Option<&Submaterial> {
        self.submaterials.get(id)
    }

    pub fn get_component_kind(&self, id: &ComponentKindId) -> Option<&ComponentKind> {
        self.component_kinds.get(id)
    }

    pub fn get_item(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.item_definitions.get(id)
    }

    pub fn get_simple_recipe(&self, id: &RecipeId) -> Option<&SimpleRecipe> {
        self.simple_recipes.get(id)
    }

    pub fn get_component_recipe(&self, id: &RecipeId) -> Option<&ComponentRecipe> {
        self.component_recipes.get(id)
    }

    pub fn get_composite_recipe(&self, id: &RecipeId) -> Option<&CompositeRecipe> {
        self.composite_recipes.get(id)
    }

    pub fn get_instance(&self, id: ItemInstanceId) -> Option<&ItemInstance> {
        self.instances.get(&id)
    }

    //==========================================================================
    // ITERATORS
    //==========================================================================

    pub fn all_materials(&self) -> impl Iterator<Item = &Material> {
        self.materials.values()
    }

    pub fn all_submaterials(&self) -> impl Iterator<Item = &Submaterial> {
        self.submaterials.values()
    }

    pub fn all_component_kinds(&self) -> impl Iterator<Item = &ComponentKind> {
        self.component_kinds.values()
    }

    pub fn all_items(&self) -> impl Iterator<Item = &ItemDefinition> {
        self.item_definitions.values()
    }

    pub fn all_simple_recipes(&self) -> impl Iterator<Item = &SimpleRecipe> {
        self.simple_recipes.values()
    }

    pub fn all_component_recipes(&self) -> impl Iterator<Item = &ComponentRecipe> {
        self.component_recipes.values()
    }

    pub fn all_composite_recipes(&self) -> impl Iterator<Item = &CompositeRecipe> {
        self.composite_recipes.values()
    }

    pub fn all_instances(&self) -> impl Iterator<Item = &ItemInstance> {
        self.instances.values()
    }

    //==========================================================================
    // INSTANCE ID GENERATION
    //==========================================================================

    /// Generate a new unique instance ID
    pub fn next_instance_id(&mut self) -> ItemInstanceId {
        let id = ItemInstanceId(self.next_instance_id);
        self.next_instance_id += 1;
        id
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
