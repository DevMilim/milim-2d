use crate::{Component, EngineContext};

pub enum Shape {
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Circle {
        radius: f32,
    },
}

pub struct ColisionShape2D {
    shape: Shape,
    disabled: bool,
    layer: u8,
}

impl Component for ColisionShape2D {
    type Config = ();
    fn new(config: Self::Config) -> Self {
        Self {
            shape: Shape::Circle { radius: 1.0 },
            disabled: false,
            layer: 1,
        }
    }
}
