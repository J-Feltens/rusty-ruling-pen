use crate::{canvas::Canvas, colors::Color, util::Vector2d};

#[derive(Clone, Debug, PartialEq, Default)]

pub struct Sprite {
    pub size_x: u32,
    pub size_y: u32,
    pub grid: Vec<Color>,

    pub origin: Vector2d,
    // the following variable contains a tuple of x- and y-indexes which specify which pixels
    // of grid are visible (not alpha = 0.0) for more efficient rendering
    pub pixel_idxs: Vec<(usize, usize)>,
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
        let mut pixel_idxs = vec![(0 as usize, 0 as usize); (size_x * size_y) as usize];
        for y in 0..size_y as usize {
            for x in 0..size_x as usize {
                pixel_idxs[y * size_x as usize + x] = (x, y);
            }
        }
        Sprite {
            size_x: (size_x),
            size_y: (size_y),
            grid: grid,
            origin: origin,
            pixel_idxs: pixel_idxs,
        }
    }

    pub fn recalc_pixel_idxs(&mut self) {
        let mut pixel_idxs: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.size_y as usize {
            for x in 0..self.size_x as usize {
                if self.grid[y * self.size_x as usize + x].a == 1.0 {
                    pixel_idxs.push((x, y));
                }
            }
        }
        self.pixel_idxs = pixel_idxs;
    }

    pub fn fill(&mut self, color: &Color) {
        for i in 0..self.grid.len() {
            self.grid[i] = color.clone();
        }
    }

    pub fn translate(&mut self, v: Vector2d) {
        self.origin += v;
    }

    pub fn translate_xy(&mut self, x: f64, y: f64) {
        self.origin.x += x;
        self.origin.y += y;
    }

    pub fn is_on_canvas(&self, canvas: &Canvas) -> bool {
        return 0.0 <= self.origin.x
            && self.origin.x < canvas.size_x as f64
            && 0.0 <= self.origin.y
            && self.origin.y < canvas.size_y as f64;
    }

    pub fn distance_to_sprite(&self, sprite: &Sprite) -> f64 {
        let distance: f64 = ((self.origin
            + Vector2d {
                x: self.size_x as f64 / 2.0,
                y: self.size_y as f64 / 2.0,
            })
            - (sprite.origin
                + Vector2d {
                    x: sprite.size_x as f64 / 2.0,
                    y: sprite.size_y as f64 / 2.0,
                }))
        .length();
        return distance;
    }
}
