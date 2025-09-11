use std::thread::spawn;

use crate::{colors::Color, sprites::Sprite, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Circle {
    pub sprite: Sprite,
}

impl Circle {
    pub fn new(r: f64, fg_color: &Color) -> Circle {
        let d = (r * 2.0).ceil() as usize;
        let mut sprite = Sprite::new(d as u32, d as u32);

        let cx = (d as f64 - 1.0) / 2.0;
        let cy = (d as f64 - 1.0) / 2.0;
        let r2 = r * r;

        for y in 0..d {
            let dy = y as f64 - cy;
            let row = y * d;
            for x in 0..d {
                let dx = x as f64 - cx;
                if dx * dx + dy * dy <= r2 {
                    sprite.grid[row + x] = fg_color.clone();
                }
            }
        }
        Circle { sprite: (sprite) }
    }

    pub fn set_origin(&mut self, origin: Vector2d) {
        self.sprite.origin = origin;
    }

    pub fn set_origin_xy(&mut self, x: f64, y: f64) {
        self.sprite.origin.x = x;
        self.sprite.origin.y = y;
    }

    pub fn translate(&mut self, vec: Vector2d) {
        self.sprite.translate(vec);
    }

    pub fn translate_xy(&mut self, x: f64, y: f64) {
        self.sprite.translate_xy(x, y);
    }
}
