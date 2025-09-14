use crate::{
    colors::{BLACK, Color},
    sprites::Sprite,
    util::Vector2d,
};
use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Circle {
    pub sprite: Sprite,
    pub color: Color,
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
        sprite.recalc_pixel_idxs();
        Circle {
            sprite: sprite,
            color: fg_color.clone(),
        }
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

    pub fn get_origin(&self) -> Vector2d {
        return self.sprite.origin;
    }

    pub fn get_center(&self) -> Vector2d {
        return self.sprite.origin
            + Vector2d {
                x: self.sprite.size_x as f64 / 2.0,
                y: self.sprite.size_y as f64 / 2.0,
            };
    }
}

#[derive(Debug)]
pub struct Frame {
    pub sprite: Sprite,
    pub thickness: u32,
    pub color: Color,
}

impl Frame {
    pub fn new(size_x: u32, size_y: u32, thickness: u32, color: Color) -> Frame {
        let mut sprite: Sprite = Sprite::new(size_x, size_y);

        for y in 0..size_y {
            for x in 0..size_x {
                if x < 0 + thickness
                    || x > size_x - thickness
                    || y < 0 + thickness
                    || y > size_y - thickness
                {
                    sprite.grid[(y * size_x + x) as usize] = color.clone();
                }
            }
        }

        Frame {
            sprite: sprite,
            thickness: thickness,
            color: color,
        }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = color.clone();
        for y in 0..self.sprite.size_y {
            for x in 0..self.sprite.size_x {
                if self.sprite.grid[(y * self.sprite.size_x + x) as usize].a == 1.0 {
                    self.sprite.grid[(y * self.sprite.size_x + x) as usize] = color.clone();
                }
            }
        }
    }
}

pub struct LetterCircle {
    pub ch: char,
    pub circle: Circle,
    pub color: Color,
}

impl LetterCircle {
    pub fn new(ch: char, r: f64, color: Color) -> LetterCircle {
        let mut circle: Circle = Circle::new(r, &color);
        if circle.sprite.size_x != 60 || circle.sprite.size_y != 60 {
            panic!("Only spritesize 60x60 is currently supported :(");
        }

        let ch = ch.to_ascii_uppercase();
        if ch as u32 > (65 + 26) {
            panic!("Illegal character '{}'", ch);
        }

        let img = image::open(format!("letters/{}.png", ch)).unwrap();
        let x_size: u32 = img.dimensions().0;
        let y_size: u32 = img.dimensions().1;

        for x in 0..x_size {
            for y in 0..y_size {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
                let alpha: f64 = pixel[3] as f64;
                if alpha > 0.0 {
                    circle.sprite.grid[(y * x_size + x) as usize].r = pixel[0];
                    circle.sprite.grid[(y * x_size + x) as usize].g = pixel[1];
                    circle.sprite.grid[(y * x_size + x) as usize].b = pixel[2];
                    circle.sprite.grid[(y * x_size + x) as usize].a = 1.0;
                }
            }
        }

        LetterCircle {
            ch: (ch),
            circle: (circle),
            color: (color),
        }
    }
}
