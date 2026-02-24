use crate::{Base, Component, EngineContext, Id, Vector2};

pub struct Body2D {
    pub velocity: Vector2,
}
impl Component for Body2D {}

impl Body2D {
    pub fn move_and_slide(&mut self, ctx: &mut EngineContext, base: &mut Base, delta: f32) {
        let movement = Vector2 {
            x: self.velocity.x * delta,
            y: self.velocity.y * delta,
        };

        base.transform.position.x += movement.x;

        for (_key, data) in ctx.collision.colliders.iter_mut() {}
        for (key, data) in ctx.collision.colliders.iter_mut() {
            if key.id == base.id {
                data.aabb.x += movement.x
            }
        }
        self.resolve_axis(ctx, base.id, base, true);

        base.transform.position.y += movement.y;
        for (key, data) in ctx.collision.colliders.iter_mut() {
            if key.id == base.id {
                data.aabb.y += movement.y
            }
        }
        self.resolve_axis(ctx, base.id, base, false);
    }
    pub fn resolve_axis(
        &mut self,
        ctx: &mut EngineContext,
        my_id: Id,
        base: &mut Base,
        is_x_axis: bool,
    ) {
        let mut my_colliders = Vec::new();
        for (key, data) in &ctx.collision.colliders {
            if key.id == my_id {
                my_colliders.push(*data);
            }
        }

        for mut collider_data in my_colliders {
            if let Some(correction) = ctx.collision.get_currection(my_id, &collider_data) {
                if is_x_axis {
                    base.transform.position.x += correction.x;
                    self.velocity.x = 0.0
                } else {
                    base.transform.position.y += correction.y;
                    self.velocity.y = 0.0
                }
                collider_data.aabb.x += correction.x;
                collider_data.aabb.y += correction.y;
            }
        }
    }
}
