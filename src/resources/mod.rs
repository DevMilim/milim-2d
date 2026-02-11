use std::collections::HashMap;

use sdl2::render::Texture;

use crate::Id;

pub struct TextureResource<'r> {
    pub(crate) textures: HashMap<Id, Texture<'r>>,
}

impl<'r> TextureResource<'r> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}
