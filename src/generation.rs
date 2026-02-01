use noise::{NoiseFn, Perlin};
use crate::types::{Biome, Land, Object, Substrate, Tile, World};

/// Get substrate noise value for a specific biome at global tile coordinates.
/// Each biome has its own noise function (via unique offset) so substrate patterns
/// are different per biome, but globally continuous across land boundaries.
fn get_substrate_noise(biome: &Biome, global_tile_x: i32, global_tile_y: i32, perlin: &Perlin, seed: u64) -> f64 {
    // Each biome gets a unique offset derived from its type and the world seed
    // This ensures different biomes have different substrate patterns
    let biome_offset: u64 = match biome {
        Biome::Lake => 0,
        Biome::Meadow => 1,
        Biome::Forest => 2,
        Biome::Mountain => 3,
    };
    
    // Create seed-based offsets unique to this biome
    let combined_seed = seed.wrapping_add(biome_offset.wrapping_mul(7919)); // 7919 is prime
    let offset_x = ((combined_seed.wrapping_mul(1103515245).wrapping_add(12345)) % 1000000) as f64 / 1000.0;
    let offset_y = ((combined_seed.wrapping_mul(2147483647).wrapping_add(54321)) % 1000000) as f64 / 1000.0;
    
    // Scale factor for feature size - higher value = more variance per land
    // With scale=0.4, a single 8x8 land spans ~3.2 noise units, giving good substrate variation
    let scale = 0.4;
    
    let noise_x = (global_tile_x as f64) * scale + offset_x;
    let noise_y = (global_tile_y as f64) * scale + offset_y;
    
    perlin.get([noise_x, noise_y])
}

pub fn determine_biome(x: i32, y: i32, perlin: &Perlin, seed: u64) -> Biome {
    // Create a seed-based offset to ensure (0, 0) isn't always the same biome
    // Use a simple hash-like function to derive offset from seed
    let offset_x = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) % 1000000) as f64 / 1000000.0 * 1000.0;
    let offset_y = ((seed.wrapping_mul(2147483647).wrapping_add(54321)) % 1000000) as f64 / 1000000.0 * 1000.0;
    
    // Sample Perlin noise at the coordinate with seed-based offset
    // Scale coordinates for better noise distribution
    let noise_value = perlin.get([(x as f64 * 0.1) + offset_x, (y as f64 * 0.1) + offset_y]);
    
    // Perlin noise returns values roughly in range [-1.0, 1.0]
    // Map to biome based on noise value ranges
    if noise_value < -0.3 {
        Biome::Lake
    } else if noise_value < 0.0 {
        Biome::Meadow
    } else if noise_value < 0.4 {
        Biome::Forest
    } else {
        Biome::Mountain
    }
}

/// Holds the 9 biomes for a land, used during generation.
/// Biome sub-coordinate formula for land (lx, ly):
///   X coords: (2*lx - 1), (2*lx), (2*lx + 1)  => left, center, right
///   Y coords: (2*ly - 1), (2*ly), (2*ly + 1)  => top, center, bottom
pub struct LandBiomes {
    pub center: Biome,
    pub top: Biome,
    pub bottom: Biome,
    pub left: Biome,
    pub right: Biome,
    pub top_left: Biome,
    pub top_right: Biome,
    pub bottom_left: Biome,
    pub bottom_right: Biome,
}

/// Calculate the 9 biomes for a land using biome sub-coordinates.
/// 
/// FORMULA (CRITICAL):
///   For land at (land_x, land_y):
///   - biome X coords: (2*land_x - 1), (2*land_x), (2*land_x + 1)
///   - biome Y coords: (2*land_y - 1), (2*land_y), (2*land_y + 1)
/// 
/// EXAMPLE: Land (0, 0)
///   X coords: -1, 0, 1
///   Y coords: -1, 0, 1
///   top_left=(-1,-1), top=(0,-1), top_right=(1,-1)
///   left=(-1,0), center=(0,0), right=(1,0)
///   bottom_left=(-1,1), bottom=(0,1), bottom_right=(1,1)
/// 
/// EXAMPLE: Land (-4, -5)
///   X coords: -9, -8, -7
///   Y coords: -11, -10, -9
///   center biome coord = (-8, -10)
pub fn calculate_land_biomes(land_x: i32, land_y: i32, perlin: &Perlin, seed: u64) -> LandBiomes {
    // Left/Center/Right X coordinates
    let x_left   = 2 * land_x - 1;
    let x_center = 2 * land_x;
    let x_right  = 2 * land_x + 1;
    
    // Top/Center/Bottom Y coordinates
    let y_top    = 2 * land_y - 1;
    let y_center = 2 * land_y;
    let y_bottom = 2 * land_y + 1;
    
    LandBiomes {
        top_left:     determine_biome(x_left,   y_top,    perlin, seed),
        top:          determine_biome(x_center, y_top,    perlin, seed),
        top_right:    determine_biome(x_right,  y_top,    perlin, seed),
        left:         determine_biome(x_left,   y_center, perlin, seed),
        center:       determine_biome(x_center, y_center, perlin, seed),
        right:        determine_biome(x_right,  y_center, perlin, seed),
        bottom_left:  determine_biome(x_left,   y_bottom, perlin, seed),
        bottom:       determine_biome(x_center, y_bottom, perlin, seed),
        bottom_right: determine_biome(x_right,  y_bottom, perlin, seed),
    }
}

/// Get the biome for a specific tile within a land.
/// 
/// SIMPLE TILE MAPPING:
///   - 4 corners (1 tile each): (0,0), (7,0), (0,7), (7,7)
///   - 4 edges (6 tiles each):
///       top:    row 0, cols 1-6
///       bottom: row 7, cols 1-6
///       left:   col 0, rows 1-6
///       right:  col 7, rows 1-6
///   - center (36 tiles): rows 1-6, cols 1-6
/// 
/// TODO: Make this fancier with gradient blending or noise-based transitions
pub fn get_tile_biome(biomes: &LandBiomes, tile_x: usize, tile_y: usize) -> &Biome {
    let is_top_edge    = tile_y == 0;
    let is_bottom_edge = tile_y == 7;
    let is_left_edge   = tile_x == 0;
    let is_right_edge  = tile_x == 7;
    
    match (is_left_edge, is_right_edge, is_top_edge, is_bottom_edge) {
        // Corners (exactly 2 edges)
        (true,  false, true,  false) => &biomes.top_left,
        (false, true,  true,  false) => &biomes.top_right,
        (true,  false, false, true)  => &biomes.bottom_left,
        (false, true,  false, true)  => &biomes.bottom_right,
        // Edges (exactly 1 edge)
        (_,     _,     true,  false) => &biomes.top,
        (_,     _,     false, true)  => &biomes.bottom,
        (true,  false, _,     _)     => &biomes.left,
        (false, true,  _,     _)     => &biomes.right,
        // Center (no edges)
        _ => &biomes.center,
    }
}

/// Generate terrain tiles for a land based on its 9 biomes.
/// Uses simple zone-based generation: center biome for center tiles,
/// edge biomes for edge tiles, corner biomes for corner tiles.
/// 
/// The substrate_perlin should be a globally-seeded Perlin noise generator
/// to ensure substrate patterns are continuous across land boundaries.
pub fn generate_land_terrain(land_x: i32, land_y: i32, biomes: &LandBiomes, seed: u64, substrate_perlin: &Perlin) -> [[Tile; 8]; 8] {
    // Create a separate Perlin for object generation (can be land-specific since objects
    // don't need to blend across boundaries)
    let object_seed = seed
        .wrapping_add((land_x as u64).wrapping_mul(73856093))
        .wrapping_add((land_y as u64).wrapping_mul(19349663));
    let object_perlin = Perlin::new(object_seed as u32);
    
    std::array::from_fn(|tile_y| {
        std::array::from_fn(|tile_x| {
            // Get the biome for this specific tile
            let biome = get_tile_biome(biomes, tile_x, tile_y);
            
            // Calculate global tile coordinates for substrate noise
            // Land (0,0) has tiles (0,0) to (7,7)
            // Land (1,0) has tiles (8,0) to (15,7)
            // etc.
            let global_tile_x = land_x * 8 + tile_x as i32;
            let global_tile_y = land_y * 8 + tile_y as i32;
            
            // Get biome-specific substrate noise (globally continuous)
            let noise_value = get_substrate_noise(biome, global_tile_x, global_tile_y, substrate_perlin, seed);
            
            // Determine substrate based on biome
            let substrate = match biome {
                Biome::Lake => {
                    Substrate::Water
                }
                Biome::Meadow => {
                    if noise_value < -0.3 { Substrate::Dirt }
                    else { Substrate::Grass }
                }
                Biome::Forest => {
                    if noise_value < -0.4 { Substrate::Dirt }
                    else if noise_value < 0.5 { Substrate::Grass }
                    else { Substrate::Brush }
                }
                Biome::Mountain => {
                    if noise_value < 0.0 { Substrate::Stone }
                    else { Substrate::Dirt }
                }
            };
            
            // Generate objects based on biome (using land-specific noise)
            let obj_noise_x = (land_x as f64) + (tile_x as f64) * 0.125;
            let obj_noise_y = (land_y as f64) + (tile_y as f64) * 0.125;
            let obj_noise_value = object_perlin.get([obj_noise_x * 0.5, obj_noise_y * 0.5]);
            
            let mut objects = Vec::new();
            match biome {
                Biome::Lake => {
                    if obj_noise_value > 0.7 { objects.push(Object::Rock); }
                }
                Biome::Meadow => {
                    if obj_noise_value > 0.5 { objects.push(Object::Rock); }
                    if obj_noise_value > 0.8 { objects.push(Object::Stick); }
                }
                Biome::Forest => {
                    if obj_noise_value > 0.0 { objects.push(Object::Tree); }
                    if obj_noise_value > 0.6 { objects.push(Object::Rock); }
                    if obj_noise_value > 0.8 { objects.push(Object::Stick); }
                }
                Biome::Mountain => {
                    if obj_noise_value > 0.2 { objects.push(Object::Rock); }
                    if obj_noise_value > 0.6 { objects.push(Object::Rock); }
                    if obj_noise_value > 0.9 { objects.push(Object::Tree); }
                }
            }
            
            Tile { substrate, objects }
        })
    })
}

pub fn generate_world(world: &mut World, seed: u64, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Create a seeded Perlin noise generator for biome determination
    let biome_perlin = Perlin::new(seed as u32);
    
    // Create a separate Perlin for substrate generation (different seed for variety)
    // Using a transformed seed ensures substrate patterns differ from biome patterns
    let substrate_perlin = Perlin::new(seed.wrapping_add(999983) as u32); // 999983 is prime
    
    // Generate terrain for the specified range
    for x in x1..=x2 {
        for y in y1..=y2 {
            let biomes = calculate_land_biomes(x, y, &biome_perlin, seed);
            let tiles = generate_land_terrain(x, y, &biomes, seed, &substrate_perlin);
            
            let land = Land {
                tiles,
                center: biomes.center,
                top: biomes.top,
                bottom: biomes.bottom,
                left: biomes.left,
                right: biomes.right,
                top_left: biomes.top_left,
                top_right: biomes.top_right,
                bottom_left: biomes.bottom_left,
                bottom_right: biomes.bottom_right,
            };
            
            world.terrain.insert((x, y), land);
        }
    }
}

pub fn initialize_world(world: &mut World, seed: u64) {
    generate_world(world, seed, -10, -10, 10, 10);
}
