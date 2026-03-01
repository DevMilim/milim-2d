use std::{collections::HashMap, sync::Mutex};

use rodio::DeviceSinkBuilder;
use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct AssetCache<T> {
    assets: HashMap<usize, T>,
    path_map: HashMap<String, usize>,
    next_id: usize,
}

impl<T> AssetCache<T> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            path_map: HashMap::new(),
            next_id: 0,
        }
    }
    fn current_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }
    pub fn get_id(&self, path: &str) -> Option<usize> {
        self.path_map.get(path).copied()
    }
    pub fn get(&self, id: usize) -> Option<&T> {
        self.assets.get(&id)
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.assets.get_mut(&id)
    }
    pub fn insert(&mut self, path: &str, asset: T) -> usize {
        let id = self.current_id();
        self.assets.insert(id, asset);
        self.path_map.insert(path.to_string(), id);
        id
    }
}

pub struct Resources {
    pub texture_creator: TextureCreator<WindowContext>,
    pub textures: AssetCache<Texture>,

    sink_handle: rodio::MixerDeviceSink,
    players: Mutex<HashMap<String, rodio::Player>>,
}

impl Resources {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        let sink = DeviceSinkBuilder::open_default_sink().unwrap();
        Self {
            texture_creator,
            textures: AssetCache::new(),
            sink_handle: sink,
            players: Mutex::new(HashMap::new()),
        }
    }
    pub fn load_image(&mut self, path: &str) -> usize {
        use sdl2::image::LoadTexture;

        if let Some(id) = self.textures.get_id(path) {
            return id;
        }
        let texture = self
            .texture_creator
            .load_texture(path)
            .expect("Falha ao carregar texture");

        self.textures.insert(path, texture)
    }
}
