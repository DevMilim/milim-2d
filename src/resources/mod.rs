use std::collections::HashMap;

use sdl2::render::Texture;

use crate::Id;

pub struct TextureResource {
    pub(crate) textures: HashMap<Id, Texture>,
}

impl TextureResource {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}
