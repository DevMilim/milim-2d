use std::collections::VecDeque;

use sdl2::{pixels::Color, render::Texture};

use crate::{AssetCache, EngineCommands, Vector2, WindowConfig};

#[derive(Debug)]
pub enum DrawCommandType {
    Sprite,
    Rect,
}

pub struct DrawCommand {
    pub cmd_type: DrawCommandType,
    pub depth: i16,
    pub material: DrawData,
}

pub struct DrawData {
    pub pos: Vector2,
    pub size: Vector2,
    pub uv_min: Vector2,
    pub uv_max: Vector2,
    pub rotation: f32,
    pub color: Color,
    pub image: usize,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Default for DrawData {
    fn default() -> Self {
        Self {
            pos: Vector2::ZERO,
            size: Vector2::new(32.0, 32.0),
            uv_min: Vector2::ZERO,
            uv_max: Vector2::ZERO,
            rotation: 0.0,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            image: 0,
            flip_h: false,
            flip_v: false,
        }
    }
}

pub trait WindowGraphicsAdapter {
    fn new(window_config: WindowConfig) -> Self
    where
        Self: Sized;

    fn pool_events(&mut self, queue: &mut VecDeque<EngineCommands>);

    fn clear(&mut self, color: Color);

    fn present(&mut self, assets: &mut AssetCache<Texture>);

    fn get_fps(&self) -> f32;

    fn set_fps(&mut self, fps: u32);

    fn draw(&mut self, command: DrawCommand, z_index: i32);

    fn render(&mut self, assets: &mut AssetCache<Texture>);

    fn resize(&mut self, width: u32, height: u32);

    fn get_window_size(&self) -> Vector2;

    fn set_camera_pos(&mut self, pos: &Vector2);
}
