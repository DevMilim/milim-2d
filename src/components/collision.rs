use sdl2::pixels::Color;

use crate::{AABB, ColliderData, ColliderKey, Component, Vector2};

pub struct BoxCollider {
    pub key: u32,
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub layer: u32,
    pub mask: u32,
    pub debug: bool,
    pub is_sensor: bool,
}

impl Component for BoxCollider {
    fn update(&mut self, ctx: &mut crate::EngineContext, base: &mut crate::Base, delta: f32) {
        let aabb = AABB {
            x: base.transform.global_position.x + self.offset_x,
            y: base.transform.global_position.y + self.offset_y,
            width: self.width,
            height: self.height,
        };

        let data = ColliderData {
            aabb,
            layer: self.layer,
            mask: self.mask,
            is_sensor: self.is_sensor,
        };

        ctx.collision.update_collider(
            ColliderKey {
                key: self.key,
                id: base.id,
            },
            data,
        );
    }
    fn draw(&mut self, ctx: &mut crate::EngineContext, base: &crate::Base) {
        if self.debug {
            let color = if self.is_sensor {
                Color::RGB(0, 0, 255)
            } else {
                Color::RGB(255, 0, 0)
            };
            let draw_pos = Vector2::new(
                base.transform.global_position.x + self.offset_x + (self.width / 2.0),
                base.transform.global_position.y + self.offset_y + (self.height / 2.0),
            );
            ctx.draw_rect(draw_pos, Vector2::new(self.width, self.height), color, 0);
        }
    }
    fn destroy(&mut self, ctx: &mut crate::EngineContext, base: &crate::Base) {
        ctx.collision.remove_collider(ColliderKey {
            key: self.key,
            id: base.id,
        });
    }
}
