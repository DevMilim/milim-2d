pub use sdl2::keyboard::Keycode;
use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    event::WindowEvent,
    gfx::framerate::FPSManager,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};
use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::{AssetCache, DrawCommand, EngineCommands, Vector2, WindowGraphicsAdapter};

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct Sdl2Adapter {
    pub(crate) canvas: WindowCanvas,
    sdl: Sdl,
    event_pump: EventPump,
    window_config: WindowConfig,
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

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut fps = FPSManager::new();
        fps.set_framerate(60).unwrap();

        Self {
            canvas,
            video: sdl_context.video().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            sdl: sdl_context,
            window_config,
            fps,
            draw_queue: BTreeMap::new(),
            camera_pos: Vector2::ZERO,
        }
    }
    fn clear(&mut self, color: Color) {
        self.canvas
            .set_draw_color(Color::RGBA(color.r, color.g, color.b, color.a));
        self.canvas.clear();
    }

    fn present(&mut self, assets: &mut AssetCache<Texture>) {
        self.render(assets);
        self.canvas.present();
        self.fps.delay();
    }
    fn get_fps(&self) -> f32 {
        self.fps.get_framerate() as f32
    }
    fn set_fps(&mut self, fps: u32) {
        let _ = self.fps.set_framerate(fps);
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

    fn render(&mut self, assets: &mut AssetCache<Texture>) {
        let window_size = self.get_window_size();
        let half_screen = Vector2::new(window_size.x / 2.0, window_size.y / 2.0);

        for (_z, commands) in self.draw_queue.iter_mut() {
            for cmd in commands.drain(..) {
                let mat = &cmd.material;

                let screen_x =
                    (mat.pos.x - self.camera_pos.x + half_screen.x - (mat.size.x / 2.0)) as i32;
                let screen_y =
                    (mat.pos.y - self.camera_pos.y + half_screen.y - (mat.size.y / 2.0)) as i32;

                let dst_rect = Rect::new(screen_x, screen_y, mat.size.x as u32, mat.size.y as u32);
                match cmd.cmd_type {
                    super::DrawCommandType::Sprite => {
                        if let Some(texture) = assets.get_mut(mat.image) {
                            let query = texture.query();
                            let tex_w = query.width as f32;
                            let tex_h = query.height as f32;

                            let src_rect = if mat.uv_max.x > 0.0 && mat.uv_max.y > 0.0 {
                                let src_x = (mat.uv_min.x * tex_w) as i32;
                                let src_y = (mat.uv_min.y * tex_h) as i32;
                                let src_w = ((mat.uv_max.x - mat.uv_min.x) * tex_w) as u32;
                                let src_h = ((mat.uv_max.y - mat.uv_min.y) * tex_h) as u32;
                                Some(Rect::new(src_x, src_y, src_w, src_h))
                            } else {
                                None
                            };

                            texture.set_color_mod(mat.color.r, mat.color.g, mat.color.b);
                            texture.set_alpha_mod(mat.color.a);

                            let center =
                                Point::new((mat.size.x / 2.0) as i32, (mat.size.y / 2.0) as i32);

                            self.canvas
                                .copy_ex(
                                    texture,
                                    src_rect,
                                    Some(dst_rect),
                                    mat.rotation as f64,
                                    Some(center),
                                    mat.flip_h,
                                    mat.flip_v,
                                )
                                .expect("Erro ao desenhar sprite")
                        }
                    }
                    super::DrawCommandType::Rect => {
                        self.canvas.set_draw_color(Color::RGBA(
                            mat.color.r,
                            mat.color.g,
                            mat.color.b,
                            mat.color.a,
                        ));
                        self.canvas.fill_rect(dst_rect).unwrap()
                    }
                }
            }
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
