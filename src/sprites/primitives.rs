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
                let dist: f64 =
                    (((x - sprite.size_x / 2).pow(2) + (y - sprite.size_y / 2).pow(2)) as f64);
                let r_square = r.powi(2);
                if r < dist {
                    sprite.grid[(y * sprite.size_y + x) as usize] = color.clone();
                }
            }
        }
        Circle { sprite: (sprite) }
    }
}
