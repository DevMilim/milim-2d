use sdl2::pixels::Color;

use crate::{Component, Vector2};

pub struct Camera2D {
    pub active: bool,
    pub lerp_speed: f32,
    pub deadzone: Vector2,
}

impl Camera2D {
    pub fn new(deadzone: Vector2) -> Self {
        Self {
            active: true,
            lerp_speed: 1.0,
            deadzone,
        }
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            active: true,
            lerp_speed: 1.0,
            deadzone: Vector2::ZERO,
        }
    }
}

impl Component for Camera2D {
    fn late_update(&mut self, ctx: &mut crate::EngineContext, base: &mut crate::Base, delta: f32) {
        if !self.active {
            return;
        }
        let mut target_pos = *ctx.camera_pos;

        let diff_x = base.transform.position.x - ctx.camera_pos.x;
        if diff_x.abs() > self.deadzone.x {
            if diff_x > 0.0 {
                target_pos.x = base.transform.position.x - self.deadzone.x;
            } else {
                target_pos.x = base.transform.position.x + self.deadzone.x;
            }
        }

        let diff_y = base.transform.position.y - ctx.camera_pos.y;
        if diff_y.abs() > self.deadzone.y {
            if diff_y > 0.0 {
                target_pos.y = base.transform.position.y - self.deadzone.y;
            } else {
                target_pos.y = base.transform.position.y + self.deadzone.y;
            }
        }
        let t = 1.0 - (-self.lerp_speed * delta).exp();
        ctx.camera_pos.x = ctx.camera_pos.x + (target_pos.x - ctx.camera_pos.x) * t;
        ctx.camera_pos.y = ctx.camera_pos.y + (target_pos.y - ctx.camera_pos.y) * t;
    }
    fn draw(&mut self, ctx: &mut crate::EngineContext, _base: &crate::Base) {
        ctx.draw_rect(*ctx.camera_pos, self.deadzone * 2.0, Color::BLUE, 10);
    }
}
