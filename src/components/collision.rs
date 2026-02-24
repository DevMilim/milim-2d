use crate::{AABB, ColliderData, ColliderKey, Color, Component, Rect};

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
            x: base.transform.position.x + self.offset_x,
            y: base.transform.position.y + self.offset_y,
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
        let color = if self.is_sensor {
            Color::rgb(0, 0, 255)
        } else {
            Color::rgb(255, 0, 0)
        };
    }
    fn destroy(&mut self, ctx: &mut crate::EngineContext, base: &crate::Base) {
        ctx.collision.remove_collider(ColliderKey {
            key: self.key,
            id: base.id,
        });
    }
}
