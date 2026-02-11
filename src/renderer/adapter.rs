use crate::{Color, EngineCommands, Id, WindowConfig};

pub enum DrawCommands {
    DrawImage {
        name: Id,
        x: f32,
        y: f32,
        scale: f32,

        image_x: f32,
        image_y: f32,
        image_width: f32,
        image_height: f32,

        angle: f32,
        flip_h: bool,
        flip_v: bool,
    },
    DrawRect {
        rect: crate::Rect,
        color: Color,
    },
}

pub trait WindowGraphicsAdapter {
    fn new(window_config: WindowConfig) -> Self
    where
        Self: Sized;
    fn pool_events(&mut self) -> Vec<EngineCommands>;
    fn clear(&mut self, color: Color);
    fn preset(&mut self);
    fn load_image(&mut self, path: &str) -> Id;
    fn get_fps(&self) -> f32;
    fn set_fps(&mut self, fps: u32);

    fn draw(&mut self, command: DrawCommands, z_index: i32);
    fn flush_draw_queue(&mut self);
}
