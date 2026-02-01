use macroquad::prelude::*;

/// Load a PNG from embedded bytes
pub fn load_png_from_bytes(png_data: &[u8]) -> Result<Texture2D, String> {
    let img = image::load_from_memory(png_data)
        .map_err(|e| format!("Failed to decode PNG: {}", e))?;
    
    let rgba = img.to_rgba8();
    let width = rgba.width() as u16;
    let height = rgba.height() as u16;
    
    let mac_image = Image {
        bytes: rgba.into_raw(),
        width,
        height,
    };
    
    let texture = Texture2D::from_image(&mac_image);
    texture.set_filter(FilterMode::Linear);
    
    Ok(texture)
}

/// Get embedded PNG data for an object by name
pub fn get_object_png(name: &str) -> Option<&'static [u8]> {
    match name {
        "rock" => Some(include_bytes!("../../assets/boulder.png")),
        "tree" => Some(include_bytes!("../../assets/tree.png")),
        "stick" => Some(include_bytes!("../../assets/stick.png")),
        _ => None,
    }
}
