use crate::{Component, Vector2};

pub struct Camera2D {
    pub active: bool,
    pub lerp_speed: f32,
    pub offset: Vector2,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            active: true,
            lerp_speed: 1.0,
            offset: Vector2::ZERO,
        }
    }
}

impl Component for Camera2D {
    fn late_update(&mut self, ctx: &mut crate::EngineContext, base: &mut crate::Base, delta: f32) {
        if self.active {
            ctx.camera_pos.x += (base.transform.position.x - ctx.camera_pos.x) * self.lerp_speed;
            ctx.camera_pos.y += (base.transform.position.y - ctx.camera_pos.y) * self.lerp_speed;
        }
    }
}
