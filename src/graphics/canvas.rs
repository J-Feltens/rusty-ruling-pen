use crate::{
    colors::{Color, rgb2u32},
    util::Vector2d,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    size_x: usize,
    size_y: usize,

    range_x: (f64, f64),
    range_y: (f64, f64),

    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: &Color) -> Canvas {
        Canvas {
            size_x: size_x,
            size_y: size_y,

            range_x: (-3.0, 3.0),
            range_y: (-3.0, 3.0),

            buffer: vec![bg_color.as_u32(); size_x * size_y],
        }
    }

    pub fn reset(&mut self, bg_color: &Color) {
        self.buffer = vec![bg_color.as_u32(); self.size_x * self.size_y];
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        return &self.buffer;
    }

    pub fn set_range(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        assert!(x_min <= x_max);
        assert!(y_min <= y_max);
        self.range_x = (x_min, x_max);
        self.range_y = (y_min, y_max);
    }

    pub fn draw_line_1px(&mut self, v1: Vector2d, v2: Vector2d, color: &Color) {
        // Bresenhams
        let mut x_0: i32 = v1.x as i32;
        let mut y_0: i32 = v1.y as i32;
        let mut x_1: i32 = v2.x as i32;
        let mut y_1: i32 = v2.y as i32;

        let dx: i32 = (x_1 - x_0).abs();
        let sx: i32 = if x_0 < x_1 { 1 } else { -1 };
        let dy: i32 = -(y_1 - y_0).abs();
        let sy: i32 = if y_0 < y_1 { 1 } else { -1 };
        let mut error: i32 = dx + dy;

        while true {
            self.buffer[y_0 as usize * self.size_x + x_0 as usize] = color.as_u32();
            if 2 * error >= dy {
                if x_0 == x_1 {
                    break;
                }
                error = error + dy;
                x_0 = x_0 + sx;
            }
            if 2 * error <= dx {
                if y_0 == y_1 {
                    break;
                }
                error = error + dx;
                y_0 = y_0 + sy;
            }
        }
    }

    pub fn draw_line(&mut self, v1: Vector2d, v2: Vector2d, width: u32, color: &Color) {
        assert!(width >= 1);
        for i in 0..width {
            if i == 0 {
                self.draw_line_1px(v1, v2, &color);
            } else {
                self.draw_line_1px(
                    v1 + Vector2d::new(1.0, 1.0),
                    v2 + Vector2d::new(1.0, 1.0),
                    &color,
                );
                self.draw_line_1px(
                    v1 + Vector2d::new(1.0, 1.0),
                    v2 + Vector2d::new(1.0, 1.0),
                    &color,
                );
            }
        }
    }

    pub fn draw_circle(&mut self, center: Vector2d, radius: f64, color: &Color) {
        // get bounding box
        let x_min: usize = (center.x - radius) as usize;
        let x_max: usize = (center.x + radius) as usize;
        let y_min: usize = (center.y - radius) as usize;
        let y_max: usize = (center.y + radius) as usize;

        assert!(x_max < self.size_x && y_max < self.size_y);

        for y in y_min..y_max {
            for x in x_min..x_max {
                let x_local: f64 = x as f64 - center.x;
                let y_local: f64 = y as f64 - center.y;
                if (x_local * x_local + y_local * y_local).sqrt() <= radius {
                    self.buffer[y * self.size_x + x] = color.as_u32();
                }
            }
        }
    }

    pub fn add_layer(&mut self, layer: Canvas, pos_x: u32, pos_y: u32) {
        if pos_x + layer.size_x as u32 >= self.size_x as u32
            || pos_y + layer.size_y as u32 >= self.size_y as u32
        {
            println!("Layer too large for canvas!");
        }

        for y in 0..layer.size_y {
            for x in 0..layer.size_x {
                self.buffer[(pos_y as usize + y) * self.size_x + pos_x as usize + x] =
                    layer.buffer[y * layer.size_x + x];
            }
        }
    }
}
