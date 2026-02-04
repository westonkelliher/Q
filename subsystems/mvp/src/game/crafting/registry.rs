use std::collections::HashMap;
use super::ids::{ItemId, ItemInstanceId, RecipeId, MaterialId, SubmaterialId, ComponentKindId, WorldObjectInstanceId};
use super::instance::{ItemInstance, SimpleInstance, ComponentInstance, CompositeInstance};
use super::item_def::{ItemDefinition, ItemKind};
use super::materials::{Material, Submaterial, ComponentKind};
use super::recipe::{SimpleRecipe, ComponentRecipe, CompositeRecipe, WorldObjectRequirement};
use super::provenance::{Provenance, ConsumedInput};
use super::quality::Quality;
use super::world_object::WorldObjectInstance;

/// Central registry for all game content and runtime instances.
///
/// The CraftingRegistry stores:
/// - Material system (materials, submaterials, component kinds)
/// - Item definitions (simple, component, composite)
/// - Recipes (simple, component, composite)
/// - Runtime item instances with full provenance tracking
///
/// # Three-Tier Crafting System
/// The registry enforces a strict three-tier crafting flow:
/// 1. **Simple items** (submaterials) → used in ComponentRecipes
/// 2. **Components** (crafted from submaterials) → used in CompositeRecipes
/// 3. **Composites** (assembled from components) → final items
///
/// # Material Hierarchy
/// - **Materials**: Broad categories (e.g., "leather", "wood", "metal")
/// - **Submaterials**: Specific variants (e.g., "deer_leather", "oak_wood", "iron_metal")
/// - **Component Kinds**: Define what materials components can accept
///
/// # Recipe Execution
/// The registry provides validation and execution methods for all three recipe types,
/// ensuring material compatibility and slot matching.
pub struct CraftingRegistry {
    // Base content
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
    
    // World object instances
    world_objects: HashMap<WorldObjectInstanceId, WorldObjectInstance>,
    next_world_object_id: u64,
}

impl CraftingRegistry {
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
            world_objects: HashMap::new(),
            next_world_object_id: 0,
        }
    }

    /// Register a material (broad category like "leather", "wood", "metal")
    pub fn register_material(&mut self, material: Material) {
        self.materials.insert(material.id.clone(), material);
    }

    /// Register a submaterial (specific variant like "deer_leather", "oak_wood")
    pub fn register_submaterial(&mut self, submaterial: Submaterial) {
        self.submaterials.insert(submaterial.id.clone(), submaterial);
    }

    /// Register a component kind (defines what materials a component accepts)
    pub fn register_component_kind(&mut self, component_kind: ComponentKind) {
        self.component_kinds.insert(component_kind.id.clone(), component_kind);
    }

    /// Register an item definition (simple, component, or composite)
    pub fn register_item(&mut self, item: ItemDefinition) {
        self.item_definitions.insert(item.id.clone(), item);
    }

    /// Register a simple recipe (creates simple items from other simple items)
    pub fn register_simple_recipe(&mut self, recipe: SimpleRecipe) {
        self.simple_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register a component recipe (creates components from submaterials)
    pub fn register_component_recipe(&mut self, recipe: ComponentRecipe) {
        self.component_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register a composite recipe (assembles composites from components)
    pub fn register_composite_recipe(&mut self, recipe: CompositeRecipe) {
        self.composite_recipes.insert(recipe.id.clone(), recipe);
    }

    /// Register an item instance into the registry
    pub fn register_instance(&mut self, instance: ItemInstance) {
        self.instances.insert(instance.id(), instance);
    }

    /// Remove an item instance from the registry
    pub fn remove_instance(&mut self, id: ItemInstanceId) -> Option<ItemInstance> {
        self.instances.remove(&id)
    }

    /// Get a material by ID
    pub fn get_material(&self, id: &MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    /// Get a submaterial by ID
    pub fn get_submaterial(&self, id: &SubmaterialId) -> Option<&Submaterial> {
        self.submaterials.get(id)
    }

    /// Get a component kind by ID
    pub fn get_component_kind(&self, id: &ComponentKindId) -> Option<&ComponentKind> {
        self.component_kinds.get(id)
    }

    /// Get an item definition by ID
    pub fn get_item(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.item_definitions.get(id)
    }

    /// Get a simple recipe by ID
    pub fn get_simple_recipe(&self, id: &RecipeId) -> Option<&SimpleRecipe> {
        self.simple_recipes.get(id)
    }

    /// Get a component recipe by ID
    pub fn get_component_recipe(&self, id: &RecipeId) -> Option<&ComponentRecipe> {
        self.component_recipes.get(id)
    }

    /// Get a composite recipe by ID
    pub fn get_composite_recipe(&self, id: &RecipeId) -> Option<&CompositeRecipe> {
        self.composite_recipes.get(id)
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
    
    /// Create and register a simple item instance (e.g., for world drops)
    pub fn create_simple_item(&mut self, item_id: &ItemId) -> ItemInstanceId {
        let instance_id = self.next_instance_id();
        let item_instance = ItemInstance::Simple(
            SimpleInstance {
                id: instance_id,
                definition: item_id.clone(),
                provenance: Provenance {
                    recipe_id: super::ids::RecipeId("world_drop".to_string()),
                    consumed_inputs: vec![],
                    tool_used: None,
                    world_object_used: None,
                    crafted_at: 0,
                },
            }
        );
        self.register_instance(item_instance);
        instance_id
    }

    /// Iterate over all registered materials
    pub fn all_materials(&self) -> impl Iterator<Item = &Material> {
        self.materials.values()
    }

    /// Iterate over all registered submaterials
    pub fn all_submaterials(&self) -> impl Iterator<Item = &Submaterial> {
        self.submaterials.values()
    }

    /// Iterate over all registered component kinds
    pub fn all_component_kinds(&self) -> impl Iterator<Item = &ComponentKind> {
        self.component_kinds.values()
    }

    /// Iterate over all registered item definitions
    pub fn all_items(&self) -> impl Iterator<Item = &ItemDefinition> {
        self.item_definitions.values()
    }

    /// Iterate over all registered simple recipes
    pub fn all_simple_recipes(&self) -> impl Iterator<Item = &SimpleRecipe> {
        self.simple_recipes.values()
    }

    /// Iterate over all registered component recipes
    pub fn all_component_recipes(&self) -> impl Iterator<Item = &ComponentRecipe> {
        self.component_recipes.values()
    }

    /// Iterate over all registered composite recipes
    pub fn all_composite_recipes(&self) -> impl Iterator<Item = &CompositeRecipe> {
        self.composite_recipes.values()
    }

    /// Iterate over all item instances
    pub fn all_instances(&self) -> impl Iterator<Item = &ItemInstance> {
        self.instances.values()
    }

    /// Generate a new unique world object instance ID
    pub fn next_world_object_id(&mut self) -> WorldObjectInstanceId {
        let id = WorldObjectInstanceId(self.next_world_object_id);
        self.next_world_object_id += 1;
        id
    }

    /// Register a world object instance
    pub fn register_world_object(&mut self, world_object: WorldObjectInstance) {
        self.world_objects.insert(world_object.id, world_object);
    }

    /// Get a world object instance by ID
    pub fn get_world_object(&self, id: WorldObjectInstanceId) -> Option<&WorldObjectInstance> {
        self.world_objects.get(&id)
    }

    /// Iterate over all world object instances
    pub fn all_world_objects(&self) -> impl Iterator<Item = &WorldObjectInstance> {
        self.world_objects.values()
    }

    /// Validate that a world object instance meets the requirements
    pub fn validate_world_object_requirement(
        &self,
        world_object_id: WorldObjectInstanceId,
        requirement: &WorldObjectRequirement,
    ) -> Result<(), String> {
        let world_object = self.get_world_object(world_object_id)
            .ok_or_else(|| format!("World object instance {:?} not found", world_object_id))?;

        // Check if specific kind is required
        if let Some(ref required_kind) = requirement.kind {
            if &world_object.kind != required_kind {
                return Err(format!(
                    "World object kind mismatch: required {:?}, got {:?}",
                    required_kind, world_object.kind
                ));
            }
        }

        // Check if all required tags are present
        for required_tag in &requirement.required_tags {
            if !world_object.tags.contains(required_tag) {
                return Err(format!(
                    "World object missing required tag: {:?}",
                    required_tag
                ));
            }
        }

        Ok(())
    }

    // Crafting validation and execution methods

    /// Execute a SimpleRecipe to create a Simple item
    ///
    /// Validates:
    /// - All required inputs are provided with correct quantities
    /// - Input items exist in the registry
    /// - World object requirements are met (if specified)
    pub fn execute_simple_recipe(
        &mut self,
        recipe: &SimpleRecipe,
        provided_inputs: Vec<ItemInstanceId>,
        tool_used: Option<ItemInstanceId>,
        world_object_used: Option<WorldObjectInstanceId>,
    ) -> Result<ItemInstance, String> {
        // Validate world object requirement if specified
        if let Some(ref requirement) = recipe.world_object {
            if let Some(wo_id) = world_object_used {
                self.validate_world_object_requirement(wo_id, requirement)?;
            } else {
                return Err(format!("Recipe requires a world object but none was provided"));
            }
        }

        // Validate all required inputs are provided
        for required_input in &recipe.inputs {
            let mut found_quantity = 0u32;

            for &instance_id in &provided_inputs {
                let instance = self.get_instance(instance_id)
                    .ok_or_else(|| format!("Input instance {:?} not found", instance_id))?;

                // Get the item definition for this instance
                let item_def = match instance {
                    ItemInstance::Simple(si) => self.get_item(&si.definition)
                        .ok_or_else(|| format!("Item definition {:?} not found", si.definition))?,
                    _ => return Err(format!("SimpleRecipe can only accept Simple item instances as input")),
                };

                // Check if this instance matches the required input
                if item_def.id == required_input.item_id {
                    found_quantity += 1; // Assuming quantity 1 per instance for now
                }
            }

            if found_quantity < required_input.quantity {
                return Err(format!(
                    "Insufficient quantity of {:?}: need {}, have {}",
                    required_input.item_id, required_input.quantity, found_quantity
                ));
            }
        }

        // Create consumed inputs list
        let consumed_inputs: Vec<ConsumedInput> = provided_inputs
            .iter()
            .map(|&id| ConsumedInput {
                instance_id: id,
                quantity: 1, // Assuming 1 per instance
            })
            .collect();

        // Create provenance
        let provenance = Provenance {
            recipe_id: recipe.id.clone(),
            consumed_inputs,
            tool_used,
            world_object_used,
            crafted_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        // Create the SimpleInstance
        let instance_id = self.next_instance_id();
        let simple_instance = SimpleInstance {
            id: instance_id,
            definition: recipe.output.clone(),
            provenance,
        };

        Ok(ItemInstance::Simple(simple_instance))
    }

    /// Execute a ComponentRecipe to create a Component from a submaterial
    ///
    /// Validates:
    /// - Input is exactly one item instance
    /// - Input is a Simple item with a submaterial
    /// - The submaterial's parent material is in the ComponentKind's accepted_materials list
    /// - World object requirements are met (if specified)
    pub fn execute_component_recipe(
        &mut self,
        recipe: &ComponentRecipe,
        input_instance_id: ItemInstanceId,
        tool_used: Option<ItemInstanceId>,
        world_object_used: Option<WorldObjectInstanceId>,
    ) -> Result<ItemInstance, String> {
        // Validate world object requirement if specified
        if let Some(ref requirement) = recipe.world_object {
            if let Some(wo_id) = world_object_used {
                self.validate_world_object_requirement(wo_id, requirement)?;
            } else {
                return Err(format!("Recipe requires a world object but none was provided"));
            }
        }

        // Get the input instance
        let input_instance = self.get_instance(input_instance_id)
            .ok_or_else(|| format!("Input instance {:?} not found", input_instance_id))?;

        // Validate it's a Simple instance
        let simple_instance = match input_instance {
            ItemInstance::Simple(si) => si,
            _ => return Err(format!("ComponentRecipe requires a Simple item as input, but got a Component or Composite")),
        };

        // Get the item definition
        let item_def = self.get_item(&simple_instance.definition)
            .ok_or_else(|| format!("Item definition {:?} not found", simple_instance.definition))?;

        // Validate it has a submaterial
        let submaterial_id = match &item_def.kind {
            ItemKind::Simple { submaterial: Some(submat_id) } => submat_id,
            ItemKind::Simple { submaterial: None } => {
                return Err(format!("Item {:?} is not a submaterial item (no submaterial specified)", item_def.id));
            }
            _ => return Err(format!("Expected Simple item kind, but got Component or Composite")),
        };

        // Get the submaterial
        let submaterial = self.get_submaterial(submaterial_id)
            .ok_or_else(|| format!("Submaterial {:?} not found", submaterial_id))?;

        // Get the component kind
        let component_kind = self.get_component_kind(&recipe.output)
            .ok_or_else(|| format!("Component kind {:?} not found", recipe.output))?;

        // Validate the submaterial's parent material is accepted by the component kind
        if !component_kind.accepted_materials.contains(&submaterial.material) {
            return Err(format!(
                "Component kind {:?} does not accept material {:?}. Accepted materials: {:?}",
                component_kind.id, submaterial.material, component_kind.accepted_materials
            ));
        }

        // Clone necessary data before mutable borrow
        let output_component_kind = recipe.output.clone();
        let submaterial_id_cloned = submaterial_id.clone();

        // Create provenance
        let provenance = Provenance {
            recipe_id: recipe.id.clone(),
            consumed_inputs: vec![ConsumedInput {
                instance_id: input_instance_id,
                quantity: 1,
            }],
            tool_used,
            world_object_used,
            crafted_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        // Create the ComponentInstance
        let instance_id = self.next_instance_id();
        let component_instance = ComponentInstance {
            id: instance_id,
            component_kind: output_component_kind,
            submaterial: submaterial_id_cloned,
            provenance,
        };

        Ok(ItemInstance::Component(component_instance))
    }

    /// Execute a CompositeRecipe to assemble a Composite from components
    ///
    /// Validates:
    /// - Each slot in the CompositeDef is filled with exactly one component
    /// - Each provided component matches the slot's required ComponentKind
    /// - No extra components are provided
    /// - World object requirements are met (if specified)
    pub fn execute_composite_recipe(
        &mut self,
        recipe: &CompositeRecipe,
        provided_components: Vec<(String, ItemInstanceId)>, // (slot_name, instance_id)
        tool_used: Option<ItemInstanceId>,
        world_object_used: Option<WorldObjectInstanceId>,
    ) -> Result<ItemInstance, String> {
        // Validate world object requirement if specified
        if let Some(ref requirement) = recipe.world_object {
            if let Some(wo_id) = world_object_used {
                self.validate_world_object_requirement(wo_id, requirement)?;
            } else {
                return Err(format!("Recipe requires a world object but none was provided"));
            }
        }

        // Get the output item definition
        let output_def = self.get_item(&recipe.output)
            .ok_or_else(|| format!("Output item {:?} not found", recipe.output))?;

        // Extract the composite definition
        let composite_def = match &output_def.kind {
            ItemKind::Composite(def) => def,
            _ => return Err(format!("Recipe output {:?} is not a Composite item", recipe.output)),
        };

        // Validate we have exactly the right number of components
        if provided_components.len() != composite_def.slots.len() {
            return Err(format!(
                "Expected {} components but got {}",
                composite_def.slots.len(),
                provided_components.len()
            ));
        }

        // Track which slots have been filled
        let mut filled_slots: HashMap<String, ComponentInstance> = HashMap::new();
        let mut consumed_input_ids: Vec<ItemInstanceId> = Vec::new();

        // Validate each provided component
        for (slot_name, instance_id) in provided_components {
            // Find the slot definition
            let slot_def = composite_def.slots.iter()
                .find(|s| s.name == slot_name)
                .ok_or_else(|| format!("Slot {:?} not found in composite definition", slot_name))?;

            // Check if slot already filled
            if filled_slots.contains_key(&slot_name) {
                return Err(format!("Slot {:?} filled multiple times", slot_name));
            }

            // Get the component instance
            let component_instance = self.get_instance(instance_id)
                .ok_or_else(|| format!("Component instance {:?} not found", instance_id))?;

            // Validate it's a Component instance
            let component = match component_instance {
                ItemInstance::Component(ci) => ci,
                _ => return Err(format!("Slot {:?} requires a Component, but provided instance is not a Component", slot_name)),
            };

            // Validate the component kind matches the slot requirement
            if component.component_kind != slot_def.component_kind {
                return Err(format!(
                    "Slot {:?} requires component kind {:?}, but provided component is kind {:?}",
                    slot_name, slot_def.component_kind, component.component_kind
                ));
            }

            // Add to filled slots
            filled_slots.insert(slot_name.clone(), component.clone());
            consumed_input_ids.push(instance_id);
        }

        // Verify all slots are filled
        for slot in &composite_def.slots {
            if !filled_slots.contains_key(&slot.name) {
                return Err(format!("Slot {:?} not filled", slot.name));
            }
        }

        // Create provenance
        let provenance = Provenance {
            recipe_id: recipe.id.clone(),
            consumed_inputs: consumed_input_ids.iter()
                .map(|&id| ConsumedInput {
                    instance_id: id,
                    quantity: 1,
                })
                .collect(),
            tool_used,
            world_object_used,
            crafted_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        // Create the CompositeInstance
        let instance_id = self.next_instance_id();
        let composite_instance = CompositeInstance {
            id: instance_id,
            definition: recipe.output.clone(),
            quality: Quality::Common, // TODO: Implement quality calculation
            components: filled_slots,
            provenance,
        };

        Ok(ItemInstance::Composite(composite_instance))
    }
}

impl Default for CraftingRegistry {
    fn default() -> Self {
        Self::new()
    }
}
