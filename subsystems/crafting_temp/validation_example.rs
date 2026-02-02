// Example demonstrating the validation logic
// This is NOT a test file, just documentation showing how the validation works

use crafting::*;

fn example_usage() {
    let mut registry = Registry::new();

    // Example 1: ComponentRecipe validation
    // ======================================

    // Setup: Register materials, submaterials, component kinds
    registry.register_material(Material {
        id: MaterialId("leather".into()),
        name: "Leather".into(),
        description: "Animal hide material".into(),
    });

    registry.register_submaterial(Submaterial {
        id: SubmaterialId("deer_leather".into()),
        material: MaterialId("leather".into()),
        name: "Deer Leather".into(),
        description: "Soft leather from deer".into(),
    });

    registry.register_component_kind(ComponentKind {
        id: ComponentKindId("binding".into()),
        name: "Binding".into(),
        description: "Wraps around handle for grip".into(),
        accepted_materials: vec![MaterialId("leather".into()), MaterialId("fiber".into())],
        makeshift_tags: vec![],
    });

    // Register items
    registry.register_item(ItemDefinition {
        id: ItemId("deer_leather".into()),
        name: "Deer Leather".into(),
        description: "Soft leather from deer".into(),
        kind: ItemKind::Simple {
            submaterial: Some(SubmaterialId("deer_leather".into()))
        },
    });

    registry.register_item(ItemDefinition {
        id: ItemId("binding".into()),
        name: "Binding".into(),
        description: "Grip wrapping".into(),
        kind: ItemKind::Component {
            component_kind: ComponentKindId("binding".into())
        },
    });

    // Create a ComponentRecipe
    let component_recipe = ComponentRecipe {
        id: RecipeId("craft_binding".into()),
        name: "Craft Binding".into(),
        output: ComponentKindId("binding".into()),
        tool: None,
        world_object: None,
    };

    registry.register_component_recipe(component_recipe.clone());

    // Create a deer_leather instance
    let leather_instance_id = registry.next_instance_id();
    let leather_instance = ItemInstance::Simple(SimpleInstance {
        id: leather_instance_id,
        definition: ItemId("deer_leather".into()),
        provenance: Provenance {
            recipe_id: RecipeId("harvest_deer".into()),
            consumed_inputs: vec![],
            tool_used: None,
            world_object_used: None,
            crafted_at: 0,
        },
    });
    registry.register_instance(leather_instance);

    // Execute the component recipe
    let result = registry.execute_component_recipe(
        &component_recipe,
        leather_instance_id,
        None,  // no tool
        None,  // no world object
    );

    match result {
        Ok(ItemInstance::Component(component)) => {
            println!("✓ Created component:");
            println!("  - Kind: {:?}", component.component_kind);
            println!("  - Submaterial: {:?}", component.submaterial);
            println!("  - Provenance tracked: {:?}", component.provenance.recipe_id);
        }
        Err(e) => println!("✗ Validation failed: {}", e),
        _ => println!("✗ Wrong instance type created"),
    }

    // Example 2: CompositeRecipe validation
    // ======================================

    // Setup composite definition
    registry.register_component_kind(ComponentKind {
        id: ComponentKindId("scimitar_blade".into()),
        name: "Scimitar Blade".into(),
        description: "Curved blade".into(),
        accepted_materials: vec![MaterialId("metal".into())],
        makeshift_tags: vec![],
    });

    registry.register_component_kind(ComponentKind {
        id: ComponentKindId("handle".into()),
        name: "Handle".into(),
        description: "Grip".into(),
        accepted_materials: vec![MaterialId("wood".into()), MaterialId("bone".into())],
        makeshift_tags: vec![],
    });

    registry.register_item(ItemDefinition {
        id: ItemId("scimitar".into()),
        name: "Scimitar".into(),
        description: "Curved sword".into(),
        kind: ItemKind::Composite(CompositeDef {
            slots: vec![
                CompositeSlot {
                    name: "blade".into(),
                    component_kind: ComponentKindId("scimitar_blade".into()),
                },
                CompositeSlot {
                    name: "handle".into(),
                    component_kind: ComponentKindId("handle".into()),
                },
                CompositeSlot {
                    name: "binding".into(),
                    component_kind: ComponentKindId("binding".into()),
                },
            ],
            category: CompositeCategory::Weapon,
            tool_type: None,
        }),
    });

    // Create component instances (assuming blade and handle exist)
    // ... (would create blade and handle components)

    // Execute composite recipe
    let composite_recipe = CompositeRecipe {
        id: RecipeId("assemble_scimitar".into()),
        name: "Assemble Scimitar".into(),
        output: ItemId("scimitar".into()),
        tool: None,
        world_object: None,
    };

    // This would validate:
    // - All 3 slots (blade, handle, binding) are filled
    // - Each component matches its slot's required ComponentKind
    // - Creates CompositeInstance with components HashMap

    println!("\n✓ Validation logic ready for CLI integration");
}

// Expected validation behaviors:
//
// ComponentRecipe:
// ✗ Using oak_wood (wood) for binding → ALLOWED (fiber/leather accepted)
// ✗ Using iron_metal (metal) for binding → REJECTED (only fiber/leather accepted)
// ✗ Using a Component as input → REJECTED (must be Simple with submaterial)
// ✗ Using non-submaterial Simple item → REJECTED (must have submaterial)
//
// CompositeRecipe:
// ✗ Missing blade slot → REJECTED (all slots must be filled)
// ✗ Using Simple item for slot → REJECTED (must be Component)
// ✗ Using wrong ComponentKind for slot → REJECTED (must match slot requirement)
// ✗ Filling blade slot twice → REJECTED (each slot exactly once)
