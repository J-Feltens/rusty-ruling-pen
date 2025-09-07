use crate::util::Vector2d;

#[derive(Clone, Debug, PartialEq, Default)]

pub struct Sprite {
    pub size_x: u8,
    pub size_y: u8,
    // pub grid: Vec<Vector2d>,
}

impl Sprite {
    pub fn new(size_x: u8, size_y: u8) -> Sprite {
        Sprite {
            size_x: (size_x),
            size_y: (size_y),
            // grid: (vec![white.c; canvas::WIDTH * canvas::HEIGHT]),
        }
    }
}
