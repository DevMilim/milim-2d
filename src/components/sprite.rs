use crate::{Component, DrawCommand, Id, Rect, Transform2D, Vector2};

pub struct Sprite2D {
    pub texture_id: usize,
    pub source: Rect,
    pub offset: Vector2,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Sprite2D {
    pub fn new(texture_id: usize, source: Rect) -> Self {
        Self {
            texture_id,
            source,
            offset: Vector2::ZERO,
            flip_h: false,
            flip_v: false,
        }
    }
}
impl Component for Sprite2D {
    fn draw(&mut self, ctx: &mut crate::EngineContext, base: &crate::Base) {
        let mut draw_transform = Transform2D::new(
            base.transform.global_position.x + self.offset.x,
            base.transform.global_position.y + self.offset.y,
        );

        draw_transform.scale = base.transform.global_scale;

        draw_transform.rotation = base.transform.global_rotation;
    }
}
