pub use sdl2::keyboard::Keycode;
use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    gfx::framerate::FPSManager,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::{
    collections::{HashMap, HashSet},
    mem::transmute,
};

use crate::{DrawCommands, EngineCommands, Id, Vector2, WindowGraphicsAdapter};

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Sdl2Adapter<'a> {
    canvas: WindowCanvas,
    sdl: Sdl,
    event_pump: EventPump,
    window_config: WindowConfig,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<Id, Texture<'a>>,
    video: VideoSubsystem,
    fps: FPSManager,
    draw_queue: Vec<(i32, DrawCommands)>,
}

impl<'a> WindowGraphicsAdapter for Sdl2Adapter<'a> {
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
            textures: HashMap::new(),
            fps,
            draw_queue: Vec::new(),
        }
    }
    fn clear(&mut self, color: crate::Color) {
        self.canvas
            .set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        self.canvas.clear();
    }

    fn preset(&mut self) {
        self.flush_draw_queue();
        self.canvas.present();
        self.fps.delay();
    }
    fn get_fps(&self) -> f32 {
        self.fps.get_framerate() as f32
    }
    fn set_fps(&mut self, fps: u32) {
        let _ = self.fps.set_framerate(fps);
    }

    fn load_image(&mut self, path: &str) -> Id {
        use sdl2::image::LoadTexture;
        let texture = self
            .texture_creator
            .load_texture(path)
            .expect("Falha ao carregar texture");

        let unsafe_texture = unsafe { transmute(texture) };
        let id = Id::new();
        self.textures.insert(id, unsafe_texture);
        id
    }

    fn pool_events(&mut self) -> Vec<crate::EngineCommands> {
        use sdl2::event::Event::*;
        self.event_pump
            .poll_iter()
            .filter_map(|event| match event {
                Quit { .. } => Some(EngineCommands::Quit),
                KeyDown {
                    keycode: Some(key), ..
                } => Some(EngineCommands::KeyDown(key)),
                KeyUp {
                    keycode: Some(key), ..
                } => Some(EngineCommands::KeyUp(key)),
                MouseMotion { x, y, .. } => Some(EngineCommands::MousePosition(x as f32, y as f32)),
                _ => None,
            })
            .collect()
    }

    fn draw(&mut self, command: super::DrawCommands, z_index: i32) {
        self.draw_queue.push((z_index, command));
    }

    fn flush_draw_queue(&mut self) {
        self.draw_queue.sort_by(|a, b| a.0.cmp(&b.0));

        while let Some(cmd) = self.draw_queue.pop() {
            match cmd.1 {
                DrawCommands::DrawImage {
                    name,
                    x,
                    y,
                    scale,
                    image_x,
                    image_y,
                    image_width,
                    image_height,
                    angle,
                    flip_h,
                    flip_v,
                } => {
                    if let Some(texture) = self.textures.get(&name) {
                        let src = Rect::new(
                            image_x as i32,
                            image_y as i32,
                            image_width as u32,
                            image_height as u32,
                        );
                        let dst = Rect::new(
                            x as i32,
                            y as i32,
                            (image_width * scale) as u32,
                            (image_height * scale) as u32,
                        );
                        let _ = self.canvas.copy_ex(
                            texture,
                            Some(src),
                            Some(dst),
                            angle.into(),
                            None,
                            flip_h,
                            flip_v,
                        );
                    }
                }
                DrawCommands::DrawRect { rect, color } => {
                    self.canvas
                        .set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
                    let _ = self.canvas.fill_rect(Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.width as u32,
                        rect.height as u32,
                    ));
                }
            }
        }
    }
}
pub struct InputState {
    key_pressed: HashSet<Keycode>,
    key_just_pressed: HashSet<Keycode>,
    mouse_position: Vector2,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_pressed: HashSet::new(),
            key_just_pressed: HashSet::new(),
            mouse_position: Vector2::ZERO,
        }
    }
    pub(crate) fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = Vector2::new(x, y)
    }
    pub fn mouse_position(&self) -> Vector2 {
        self.mouse_position
    }
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.key_pressed.contains(&key)
    }
    pub fn is_key_just_pressed(&self, key: Keycode) -> bool {
        self.key_just_pressed.contains(&key)
    }
    pub fn clear_frame_data(&mut self) {
        self.key_just_pressed.clear();
    }
    pub fn update_key(&mut self, key: Keycode, pressed: bool) {
        if pressed {
            if !self.key_pressed.contains(&key) {
                self.key_just_pressed.insert(key);
            }
            self.key_pressed.insert(key);
        } else {
            self.key_pressed.remove(&key);
        }
    }
}
