pub use sdl2::keyboard::Keycode;
use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    event::WindowEvent,
    gfx::framerate::FPSManager,
    pixels::Color,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::{DrawCommand, EngineCommands, Vector2, WindowGraphicsAdapter};

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Sdl2Adapter {
    canvas: WindowCanvas,
    sdl: Sdl,
    event_pump: EventPump,
    window_config: WindowConfig,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<Texture>,
    texture_path: HashMap<String, usize>,
    video: VideoSubsystem,
    fps: FPSManager,
    draw_queue: BTreeMap<i32, Vec<DrawCommand>>,
    camera_pos: Vector2,
}

impl WindowGraphicsAdapter for Sdl2Adapter {
    fn new(window_config: WindowConfig) -> Self
    where
        Self: Sized,
    {
        use sdl2::pixels::Color;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        sdl2::hint::set("SDL2_RENDER_SCALE_QUALITY", "nearest");

        let window = video_subsystem
            .window(
                window_config.title.as_str(),
                window_config.width,
                window_config.height,
            )
            .opengl()
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut fps = FPSManager::new();
        fps.set_framerate(60).unwrap();

        Self {
            texture_creator: canvas.texture_creator(),
            canvas,
            video: sdl_context.video().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            sdl: sdl_context,
            window_config,
            textures: Vec::new(),
            fps,
            draw_queue: BTreeMap::new(),
            texture_path: HashMap::new(),
            camera_pos: Vector2::ZERO,
        }
    }
    fn clear(&mut self, color: crate::Color) {
        self.canvas
            .set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.render();
        self.canvas.present();
        self.fps.delay();
    }
    fn get_fps(&self) -> f32 {
        self.fps.get_framerate() as f32
    }
    fn set_fps(&mut self, fps: u32) {
        let _ = self.fps.set_framerate(fps);
    }

    fn load_image(&mut self, path: &str) -> usize {
        use sdl2::image::LoadTexture;

        if let Some(id) = self.texture_path.get(path) {
            return *id;
        }
        let texture = self
            .texture_creator
            .load_texture(path)
            .expect("Falha ao carregar texture");
        let index = self.textures.len();

        self.textures.push(texture);
        self.texture_path.insert(path.to_string(), index);
        index
    }

    fn pool_events(&mut self, queue: &mut VecDeque<EngineCommands>) {
        use sdl2::event::Event::*;
        for event in self.event_pump.poll_iter() {
            match event {
                Quit { .. } => {
                    queue.push_back(EngineCommands::Quit);
                }
                KeyDown {
                    keycode: Some(key),
                    repeat,
                    ..
                } => {
                    if !repeat {
                        queue.push_back(EngineCommands::KeyDown(key));
                    }
                }
                KeyUp {
                    keycode: Some(key), ..
                } => queue.push_back(EngineCommands::KeyUp(key)),
                MouseMotion { x, y, .. } => {
                    queue.push_back(EngineCommands::MousePosition(x as f32, y as f32));
                }
                Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        self.window_config.width = w as u32;
                        self.window_config.height = h as u32;
                    }
                }
                _ => {}
            };
        }
    }

    fn draw(&mut self, command: super::DrawCommand, z_index: i32) {
        self.draw_queue.entry(z_index).or_default().push(command);
    }

    fn render(&mut self) {
        let window_size = self.get_window_size();
        let half_screen = Vector2::new(window_size.x / 2.0, window_size.y / 2.0);

        for (_z, commands) in self.draw_queue.iter_mut() {
            for cmd in commands.drain(..) {}
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let window = self.canvas.window_mut();
        let _ = window.set_size(width, height);
    }

    fn get_window_size(&self) -> Vector2 {
        let window_size = self.canvas.window().size();
        Vector2::new(window_size.0 as f32, window_size.1 as f32)
    }

    fn set_camera_pos(&mut self, pos: &Vector2) {
        self.camera_pos = *pos
    }
}
