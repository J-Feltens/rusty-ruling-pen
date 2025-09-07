use crate::{colors::Color, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]

pub struct Sprite {
    pub size_x: u32,
    pub size_y: u32,
    pub grid: Vec<Color>,

    pub origin: Vector2d,
}

impl Sprite {
    pub fn new(size_x: u32, size_y: u32) -> Sprite {
        let grid: Vec<Color> = vec![Color { r: 0, g: 0, b: 0 }; (size_x * size_y) as usize];
        let origin: Vector2d = Vector2d { x: (0.0), y: (0.0) };
        Sprite {
            size_x: (size_x),
            size_y: (size_y),
            grid: grid,
            origin: origin,
        }
    }
}
