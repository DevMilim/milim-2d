use crate::{Color, Vector2};

pub struct Image {
    position: Vector2,
    width: u32,
    height: u32,
    pixels: Vec<Color>,
    z_index: u32,
}
