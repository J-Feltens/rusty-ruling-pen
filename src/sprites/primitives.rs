use crate::{colors::Color, sprites::Sprite, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Circle {
    sprite: Sprite,
}

impl Circle {
    pub fn new(r: f64, color: Color) -> Circle {
        let mut sprite: Sprite = Sprite::new((r / 2.0) as u32, (r / 2.0) as u32);
        for y in 0..sprite.size_x {
            for x in 0..sprite.size_y {
                let dist: u32 = (x - sprite.size_x / 2).pow(2) + (y - sprite.size_y / 2).pow(2);
                let r_square: u32 = r.powi(2) as u32;
                if r_square < dist {
                    sprite.grid[(y * sprite.size_y + x) as usize] = color.clone();
                }
            }
        }
        Circle { sprite: (sprite) }
    }

    pub fn draw_on_buffer(&self, buffer: &mut Vec<u32>, size_x: u32, size_y: u32) {
        // compute if entire spite is within canvas, if so ignore comparison for every pixel
        for y_idx in 0..self.sprite.size_y {
            for x_idx in 0..self.sprite.size_x {
                continue;
            }
        }
    }
}
