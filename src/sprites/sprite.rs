use crate::{colors::Color, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]

pub struct Sprite {
    pub size_x: u32,
    pub size_y: u32,
    pub grid: Vec<Color>,
}

impl Sprite {
    pub fn new(size_x: u32, size_y: u32) -> Sprite {
        let grid: Vec<Color> = vec![Color { r: 0, g: 0, b: 0 }; (size_x * size_y) as usize];
        Sprite {
            size_x: (size_x),
            size_y: (size_y),
            grid,
        }
    }
}
