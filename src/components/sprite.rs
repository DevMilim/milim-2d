use sdl2::pixels::Color;

use crate::{Component, DrawCommand, DrawCommandType, DrawData, Rect, Vector2};

pub struct Sprite2D {
    pub texture_id: usize,
    pub source: Rect,
    pub scale: Vector2,
    pub offset: Vector2,
    pub flip_h: bool,
    pub flip_v: bool,
    pub z_index: i32,
    pub color: Color,
}

impl Sprite2D {
    pub fn new(texture_id: usize, source: Rect) -> Self {
        Self {
            texture_id,
            source,
            offset: Vector2::ZERO,
            flip_h: false,
            flip_v: false,
            z_index: 0,
            color: Color::WHITE,
            scale: Vector2::ONE,
        }
    }
}
impl Component for Sprite2D {
    fn draw(&mut self, ctx: &mut crate::EngineContext, base: &crate::Base) {
        let (tex_w, tex_h) = if let Some(tex) = ctx.resources.textures.get(self.texture_id) {
            let q = tex.query();
            (q.width as f32, q.height as f32)
        } else {
            return;
        };

        let uv_min = Vector2::new(self.source.x as f32 / tex_w, self.source.y as f32 / tex_h);
        let uv_max = Vector2::new(
            (self.source.x + self.source.w) as f32 / tex_w,
            (self.source.y + self.source.h) as f32 / tex_h,
        );

        let final_size = Vector2::new(
            self.source.w as f32 * base.transform.global_scale.x * self.scale.x,
            self.source.h as f32 * base.transform.global_scale.y * self.scale.y,
        );

        let material = DrawData {
            pos: base.transform.global_position + self.offset,
            size: final_size,
            uv_min,
            uv_max,
            rotation: base.transform.global_rotation,
            color: self.color,
            image: self.texture_id,
            flip_h: self.flip_h,
            flip_v: self.flip_v,
        };

        ctx.adapter.draw(
            DrawCommand {
                cmd_type: DrawCommandType::Sprite,
                depth: 0,
                material,
            },
            self.z_index,
        );
    }
}
