use std::{f64::consts::PI, io::stdout, vec};

use crate::{
    colors::{Color, rgb2u32},
    util::{CoordSystem, Line3d, Vector2d, Vector3d},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub size_x: usize,
    pub size_y: usize,

    pub range_x: (f64, f64),
    pub range_y: (f64, f64),

    pub viewport_offset: Vector2d,

    pub buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: &Color) -> Canvas {
        Canvas {
            size_x: size_x,
            size_y: size_y,

            range_x: (-3.0, 3.0),
            range_y: (-3.0, 3.0),

            viewport_offset: Vector2d::new(0.0, 0.0),

            buffer: vec![bg_color.as_u32(); size_x * size_y],
        }
    }

    pub fn reset(&mut self, bg_color: &Color) {
        self.buffer = vec![bg_color.as_u32(); self.size_x * self.size_y];
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        return &self.buffer;
    }

    pub fn set_viewport_offset(&mut self, offset: Vector2d) {
        self.viewport_offset = offset;
    }

    pub fn set_range(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        assert!(x_min <= x_max);
        assert!(y_min <= y_max);
        self.range_x = (x_min, x_max);
        self.range_y = (y_min, y_max);
    }

    pub fn vec_in_canvas(&self, v: Vector2d) -> bool {
        return v.x >= 0.0
            && (v.x as usize) < self.size_x
            && v.y >= 0.0
            && (v.y as usize) < self.size_y;
    }

    pub fn coords_integer_in_canvas(&self, x: i32, y: i32) -> bool {
        return x >= 0 && (x as usize) < self.size_x && y >= 0 && (y as usize) < self.size_y;
    }

    pub fn draw_line_1px(&mut self, v1: Vector2d, v2: Vector2d, color: &Color) {
        /*
           As of now, this function contains the only direct write-on-bytebuffer operation
        */

        // check if line is within canvas bounds
        let v1_in_canvas: bool = self.vec_in_canvas(v1);
        let v2_in_canvas: bool = self.vec_in_canvas(v2);
        let mut have_to_bound_check: bool = true;

        if !v1_in_canvas && !v2_in_canvas {
            // return;
        } else if v1_in_canvas && v2_in_canvas {
            have_to_bound_check = false;
        }

        // Bresenhams
        let mut x_0: i32 = v1.x as i32;
        let mut y_0: i32 = v1.y as i32;
        let x_1: i32 = v2.x as i32;
        let y_1: i32 = v2.y as i32;

        let dx: i32 = (x_1 - x_0).abs();
        let sx: i32 = if x_0 < x_1 { 1 } else { -1 };
        let dy: i32 = -(y_1 - y_0).abs();
        let sy: i32 = if y_0 < y_1 { 1 } else { -1 };
        let mut error: i32 = dx + dy;

        let buffer_len = self.get_buffer().len();
        loop {
            let target_in_buffer = ((self.size_y as f64 - 1.0 - y_0 as f64
                + self.viewport_offset.y)
                * self.size_x as f64
                + x_0 as f64
                + self.viewport_offset.x as f64) as usize;
            // println!(
            //     "drawing pixel at ({x_0}, {y_0}) => buffer[{target_in_buffer}], buffer length: {}",
            //     self.buffer.len()
            // );
            // if have_to_bound_check {
            //     if self.coords_integer_in_canvas(x_0, y_0) {
            //         self.buffer[target_in_buffer] = color.as_u32();
            //     } else {
            //         println!("Skippin pixel out of bounds");
            //     }
            // } else {
            //     self.buffer[target_in_buffer] = color.as_u32();
            // }

            self.buffer[target_in_buffer % buffer_len] = color.as_u32();

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
        self.draw_line_1px(v1, v2, color);
    }

    pub fn draw_3d_line_by_endpoints(
        &mut self,
        v1: Vector3d,
        v2: Vector3d,
        width: u32,
        color: &Color,
    ) {
        let t1: Vector2d = Vector2d::new(1.0, 0.0);
        let t2: Vector2d = Vector2d::new(0.0, 1.0);
        let t3: Vector2d = Vector2d::new(0.0, 0.0);
        // calc t3 (could be described as "direction into screen")
        // let t3 =

        let v1_projected = t1 * v1.x + t2 * v1.y + t3 * v1.z;
        let v2_projected = t1 * v2.x + t2 * v2.y + t3 * v2.z;

        self.draw_line(v1_projected, v2_projected, width, color);
    }

    pub fn draw_3d_line(&mut self, line: Line3d, width: u32) {
        let t1: Vector2d = Vector2d::new(1.0, 0.0);
        let t2: Vector2d = Vector2d::new(0.0, 1.0);
        let t3: Vector2d = Vector2d::new(0.0, 0.0);

        let v1_projected = t1 * line.v1.x + t2 * line.v1.y + t3 * line.v1.z;
        let v2_projected = t1 * line.v2.x + t2 * line.v2.y + t3 * line.v2.z;

        self.draw_line(v1_projected, v2_projected, width, &line.color);
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
