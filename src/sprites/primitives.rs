use std::thread::spawn;

use crate::{colors::Color, sprites::Sprite, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Circle {
    pub sprite: Sprite,
}

impl Circle {
    pub fn new(r: f64, fg_color: Color, bg_color: Color) -> Circle {
        let d = (r * 2.0).ceil() as usize;
        let mut sprite = Sprite::new(d as u32, d as u32);
        sprite.fill(bg_color);

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
}
