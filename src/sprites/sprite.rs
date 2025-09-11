use crate::{canvas::Canvas, colors::Color, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]

pub struct Sprite {
    pub size_x: u32,
    pub size_y: u32,
    pub grid: Vec<Color>,

    pub origin: Vector2d,
}

impl Sprite {
    pub fn new(size_x: u32, size_y: u32) -> Sprite {
        let grid: Vec<Color> = vec![
            Color {
                r: 0,
                g: 255,
                b: 0,
                a: 0.0,
            };
            (size_x * size_y) as usize
        ];
        let origin: Vector2d = Vector2d { x: (0.0), y: (0.0) };
        Sprite {
            size_x: (size_x),
            size_y: (size_y),
            grid: grid,
            origin: origin,
        }
    }

    pub fn fill(&mut self, color: &Color) {
        for i in 0..self.grid.len() {
            self.grid[i] = color.clone();
        }
    }

    pub fn translate(&mut self, v: Vector2d) {
        self.origin += v;
    }

    pub fn is_on_canvas(&self, canvas: &Canvas) -> bool {
        return 0.0 <= self.origin.x
            && self.origin.x < canvas.size_x as f64
            && 0.0 <= self.origin.y
            && self.origin.y < canvas.size_y as f64;
    }

    pub fn distance_to_sprite(&self, sprite: &Sprite) -> f64 {
        let distance: f64 = (self.origin - sprite.origin).length();
        return distance;
    }
}
