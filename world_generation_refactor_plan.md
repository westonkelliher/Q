# World Generation Refactor Plan

## Current Architecture Issues

The current procedural approach scatters biome rules across multiple files:
- `biome.rs` - noise thresholds for biome selection
- `substrate.rs` - substrate rules per biome  
- `objects.rs` - object rules per biome
- `macroquad.rs` - colors per biome/substrate

Adding constraints (like "trees can't grow on stone") requires threading parameters through function calls and adding conditional logic.

### Problems for Increasing Complexity

1. **Scattered Rules**: Adding a new biome requires edits to:
   - `biome.rs` (thresholds)
   - `substrate.rs` (substrate rules)
   - `objects.rs` (object rules)
   - `macroquad.rs` (colors)
   - `types.rs` (enum variant)

2. **No Cross-Cutting Constraints**: The change we're about to make (trees can't grow on stone) requires threading `substrate` through function parameters. As you add more constraints (e.g., "sticks only on dirt/grass", "rocks more common near water edges"), you'll need to keep adding parameters.

3. **Hardcoded Probabilities**: Each probability is a magic number in the code. Tuning them requires code changes and recompilation.

4. **No Rule Composition**: If you want "Forest near Mountain has different tree density than Forest near Lake", there's no way to express that without adding more conditionals.

## Proposed Data-Driven Architecture

### Core Rule Structures

```rust
// src/generation/rules.rs

use crate::types::{Biome, Substrate, Object};
use crate::render::Color;

/// Defines substrate generation rules for a biome
#[derive(Clone)]
pub struct SubstrateRule {
    pub substrate: Substrate,
    pub weight: f64,           // Relative probability
    pub noise_threshold: Option<f64>,  // If using noise-based selection
}

/// Defines object spawning rules with substrate constraints
#[derive(Clone)]
pub struct ObjectRule {
    pub object: Object,
    pub weight: f64,                      // Relative probability when object is placed
    pub allowed_substrates: Vec<Substrate>,  // Empty = all substrates allowed
    pub forbidden_substrates: Vec<Substrate>, // Takes precedence over allowed
}

/// Complete rule set for a biome
#[derive(Clone)]
pub struct BiomeRules {
    pub biome: Biome,
    
    // Visual
    pub color: Color,
    
    // Substrate generation
    pub substrates: Vec<SubstrateRule>,
    
    // Object generation  
    pub object_placement_rate: f64,       // Base probability (e.g., 0.075 for 7.5%)
    pub objects: Vec<ObjectRule>,
    
    // Future extensibility
    pub tags: Vec<String>,                // e.g., ["wet", "cold", "rocky"]
}
```

### Rule Registry

```rust
// src/generation/rules_registry.rs

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BIOME_RULES: HashMap<Biome, BiomeRules> = {
        let mut m = HashMap::new();
        m.insert(Biome::Lake, lake_rules());
        m.insert(Biome::Meadow, meadow_rules());
        m.insert(Biome::Forest, forest_rules());
        m.insert(Biome::Mountain, mountain_rules());
        m
    };
}

fn mountain_rules() -> BiomeRules {
    BiomeRules {
        biome: Biome::Mountain,
        color: Color::rgb(0.8, 0.8, 0.85),
        
        substrates: vec![
            SubstrateRule { 
                substrate: Substrate::Stone, 
                weight: 0.8,
                noise_threshold: Some(0.6),  // noise < 0.6 = Stone
            },
            SubstrateRule { 
                substrate: Substrate::Dirt, 
                weight: 0.2,
                noise_threshold: None,       // else Dirt
            },
        ],
        
        object_placement_rate: 0.075,
        objects: vec![
            ObjectRule {
                object: Object::Rock,
                weight: 0.85,
                allowed_substrates: vec![],  // All substrates OK
                forbidden_substrates: vec![],
            },
            ObjectRule {
                object: Object::Tree,
                weight: 0.15,
                allowed_substrates: vec![Substrate::Dirt],  // Only on dirt!
                forbidden_substrates: vec![Substrate::Stone],
            },
        ],
        
        tags: vec!["rocky".into(), "elevated".into()],
    }
}

fn forest_rules() -> BiomeRules {
    BiomeRules {
        biome: Biome::Forest,
        color: Color::rgb(0.1, 0.5, 0.1),
        
        substrates: vec![
            SubstrateRule { substrate: Substrate::Dirt, weight: 0.15, noise_threshold: Some(-0.4) },
            SubstrateRule { substrate: Substrate::Grass, weight: 0.70, noise_threshold: Some(0.5) },
            SubstrateRule { substrate: Substrate::Brush, weight: 0.15, noise_threshold: None },
        ],
        
        object_placement_rate: 0.10,  // Forests have more objects!
        objects: vec![
            ObjectRule {
                object: Object::Tree,
                weight: 0.70,  // 70% trees
                allowed_substrates: vec![Substrate::Grass, Substrate::Dirt, Substrate::Brush],
                forbidden_substrates: vec![],
            },
            ObjectRule {
                object: Object::Rock,
                weight: 0.20,
                allowed_substrates: vec![],
                forbidden_substrates: vec![],
            },
            ObjectRule {
                object: Object::Stick,
                weight: 0.10,
                allowed_substrates: vec![Substrate::Grass, Substrate::Dirt],
                forbidden_substrates: vec![Substrate::Water],
            },
        ],
        
        tags: vec!["wooded".into()],
    }
}

fn meadow_rules() -> BiomeRules {
    BiomeRules {
        biome: Biome::Meadow,
        color: Color::rgb(0.7, 0.9, 0.4),
        
        substrates: vec![
            SubstrateRule { 
                substrate: Substrate::Dirt, 
                weight: 0.3,
                noise_threshold: Some(-0.3),  // noise < -0.3 = Dirt
            },
            SubstrateRule { 
                substrate: Substrate::Grass, 
                weight: 0.7,
                noise_threshold: None,  // else Grass
            },
        ],
        
        object_placement_rate: 0.075,
        objects: vec![
            ObjectRule {
                object: Object::Rock,
                weight: 0.8,
                allowed_substrates: vec![],
                forbidden_substrates: vec![],
            },
            ObjectRule {
                object: Object::Stick,
                weight: 0.2,
                allowed_substrates: vec![Substrate::Grass, Substrate::Dirt],
                forbidden_substrates: vec![],
            },
        ],
        
        tags: vec!["grassy".into()],
    }
}

fn lake_rules() -> BiomeRules {
    BiomeRules {
        biome: Biome::Lake,
        color: Color::rgb(0.2, 0.5, 0.9),
        
        substrates: vec![
            SubstrateRule { 
                substrate: Substrate::Water, 
                weight: 1.0,
                noise_threshold: None,  // Always water
            },
        ],
        
        object_placement_rate: 0.075,
        objects: vec![
            ObjectRule {
                object: Object::Rock,
                weight: 1.0,  // Always rock when object placed
                allowed_substrates: vec![],
                forbidden_substrates: vec![],
            },
        ],
        
        tags: vec!["wet".into(), "aquatic".into()],
    }
}
```

### Generic Generation Functions

```rust
// src/generation/substrate.rs (refactored)

use crate::generation::rules::{BiomeRules, SubstrateRule};

pub fn generate_substrate(rules: &BiomeRules, noise: f64) -> Substrate {
    // Use noise thresholds if defined, otherwise use weights
    for rule in &rules.substrates {
        if let Some(threshold) = rule.noise_threshold {
            if noise < threshold {
                return rule.substrate.clone();
            }
        }
    }
    // Fallback to last substrate (should always have one without threshold)
    rules.substrates.last()
        .map(|r| r.substrate.clone())
        .unwrap_or(Substrate::Dirt)
}

// src/generation/objects.rs (refactored)

use crate::generation::rules::{BiomeRules, ObjectRule};
use crate::types::{Object, Substrate};

pub fn generate_object(
    rules: &BiomeRules,
    substrate: &Substrate,
    random_value: f64,
    object_type_value: f64,
) -> Option<Object> {
    // Check if object should be placed
    if random_value >= rules.object_placement_rate {
        return None;
    }
    
    // Filter objects by substrate constraints
    let valid_objects: Vec<&ObjectRule> = rules.objects.iter()
        .filter(|rule| {
            let allowed = rule.allowed_substrates.is_empty() 
                || rule.allowed_substrates.contains(substrate);
            let forbidden = rule.forbidden_substrates.contains(substrate);
            allowed && !forbidden
        })
        .collect();
    
    if valid_objects.is_empty() {
        return None;
    }
    
    // Normalize weights and select
    let total_weight: f64 = valid_objects.iter().map(|r| r.weight).sum();
    let mut cumulative = 0.0;
    
    for rule in valid_objects {
        cumulative += rule.weight / total_weight;
        if object_type_value < cumulative {
            return Some(rule.object.clone());
        }
    }
    
    None
}
```

### Updated Generation Flow

```rust
// src/generation/mod.rs (refactored)

pub fn generate_land_terrain(
    land_x: i32,
    land_y: i32,
    biomes: &LandBiomes,
    seed: u64,
    substrate_perlin: &Perlin,
) -> [[Tile; 8]; 8] {
    use crate::generation::rules_registry::BIOME_RULES;
    
    let mut tiles = std::array::from_fn(|tile_y| {
        std::array::from_fn(|tile_x| {
            let biome = get_tile_biome(biomes, tile_x, tile_y);
            let rules = BIOME_RULES.get(biome).expect("All biomes must have rules");
            
            // Global tile coordinates for substrate (cross-boundary continuity)
            let global_x = land_x * 8 + tile_x as i32;
            let global_y = land_y * 8 + tile_y as i32;
            
            // Generate substrate
            let substrate_noise = substrate::get_substrate_noise(
                biome, global_x, global_y, substrate_perlin, seed
            );
            let substrate = substrate::generate_substrate(rules, substrate_noise);
            
            // Generate objects (now with substrate-aware filtering)
            let random_value = objects::tile_random_value(seed, land_x, land_y, tile_x, tile_y);
            let object_type_value = objects::tile_random_value(
                seed.wrapping_add(1), land_x, land_y, tile_x, tile_y
            );
            
            let objects = match objects::generate_object(
                rules, &substrate, random_value, object_type_value
            ) {
                Some(obj) => vec![obj],
                None => Vec::new(),
            };
            
            Tile { substrate, objects }
        })
    });
    
    // Second pass: Add sticks deterministically near trees
    objects::add_sticks_near_trees(&mut tiles, seed, land_x, land_y);
    
    tiles
}
```

### Color Integration

```rust
// src/render/macroquad.rs (refactored)

use crate::generation::rules_registry::BIOME_RULES;

impl MacroquadRenderer {
    fn substrate_color(substrate: &Substrate) -> Color {
        match substrate {
            Substrate::Grass => {
                // Use meadow biome color for grass
                BIOME_RULES.get(&Biome::Meadow)
                    .map(|r| r.color)
                    .unwrap_or(Color::rgb(0.7, 0.9, 0.4))
            },
            Substrate::Dirt => Color::rgb(0.6, 0.4, 0.2),
            Substrate::Stone => Color::rgb(0.7, 0.7, 0.7),
            Substrate::Mud => Color::rgb(0.4, 0.3, 0.2),
            Substrate::Water => Color::rgb(0.2, 0.4, 0.9),
            Substrate::Brush => Color::rgb(0.6, 0.8, 0.3),
        }
    }
    
    fn biome_color(biome: &Biome) -> Color {
        BIOME_RULES.get(biome)
            .map(|r| r.color)
            .unwrap_or_else(|| {
                // Fallback colors if rules not found
                match biome {
                    Biome::Forest => Color::rgb(0.1, 0.5, 0.1),
                    Biome::Meadow => Color::rgb(0.7, 0.9, 0.4),
                    Biome::Lake => Color::rgb(0.2, 0.5, 0.9),
                    Biome::Mountain => Color::rgb(0.8, 0.8, 0.85),
                }
            })
    }
}
```

## Benefits of This Approach

1. **Single Source of Truth**: All rules for a biome in one place
2. **Declarative Constraints**: Substrate restrictions are data, not conditionals
3. **Easy Tuning**: Change probabilities without touching logic
4. **Extensible**: Add new fields (elevation, moisture, temperature) without refactoring
5. **Testable**: Rules can be validated/tested independently
6. **Future: Config Files**: Rules could be loaded from TOML/JSON for modding

## Migration Path

1. Create `rules.rs` with data structures
2. Create `rules_registry.rs` with current rules as data
3. Refactor `substrate.rs` to use rules
4. Refactor `objects.rs` to use rules
5. Refactor `macroquad.rs` to pull colors from rules
6. Remove hardcoded match statements
7. Update tests

## Future Enhancements

### Biome Transitions
Rules for edge tiles between biomes:
```rust
pub struct TransitionRule {
    pub from_biome: Biome,
    pub to_biome: Biome,
    pub modifier: RuleModifier,  // e.g., reduce tree density by 50%
}
```

### Neighbor Awareness
Objects influenced by adjacent tiles:
```rust
pub struct ObjectRule {
    // ... existing fields ...
    pub neighbor_bonus: Vec<(Biome, f64)>,  // e.g., trees more common near water
    pub neighbor_penalty: Vec<(Biome, f64)>, // e.g., rocks less common near forests
}
```

### Layered Rules
Base rules + modifiers:
```rust
pub struct RuleModifier {
    pub condition: ModifierCondition,  // e.g., "near water", "elevation > 0.5"
    pub effect: ModifierEffect,        // e.g., multiply tree weight by 1.5
}
```

### Config Loading
External rule files for easy tweaking:
```toml
# biomes.toml
[mountain]
color = [0.8, 0.8, 0.85]
object_placement_rate = 0.075

[mountain.objects.rock]
weight = 0.85
allowed_substrates = []

[mountain.objects.tree]
weight = 0.15
allowed_substrates = ["dirt"]
forbidden_substrates = ["stone"]
```

## Trade-offs

### Pros
- Much more maintainable for complex rules
- Easy to add new biomes
- Rules can be data-driven/config-driven
- Clear separation of concerns

### Cons
- More upfront complexity
- Requires refactoring existing code
- Slightly more indirection (HashMap lookup vs direct match)
- Need to ensure all biomes have rules defined

## Recommendation

**For current complexity**: The existing procedural approach is fine. The refactor adds overhead without immediate benefit.

**For future growth**: If you plan to add:
- More than 6-8 biomes
- Complex constraint types (elevation, moisture, neighbor effects)
- Modding support
- Frequent rule tuning

Then the refactor is worth doing. Otherwise, the current structure is sufficient.
