use eframe::epaint::TextureHandle;

pub struct Texture {
    texture_handle: TextureHandle,
    name: String,
}

impl Texture {
    pub fn new(texture_handle: TextureHandle, name: &str) -> Self {
        Self {
            texture_handle,
            name: String::from(name),
        }
    }

    pub fn get_handle(&self) -> &TextureHandle {
        return &self.texture_handle
    }
    pub fn get_name(&self) -> &str {
        return &self.name
    }
}
