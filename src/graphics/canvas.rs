use std::{f64::consts::PI, io::stdout, vec};

use rand::thread_rng;

use crate::{
    colors::{Color, rgb2u32},
    util::{Vector2d, Vector3d, map_range},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub size_x: usize,
    pub size_y: usize,

    pub buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: &Color) -> Canvas {
        Canvas {
            size_x: size_x,
            size_y: size_y,

            buffer: vec![bg_color.as_u32(); size_x * size_y],
        }
    }

    pub fn reset(&mut self, bg_color: &Color) {
        self.buffer = vec![bg_color.as_u32(); self.size_x * self.size_y];
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        return &self.buffer;
    }

    pub fn integer_coords_in_canvas(&self, x: i32, y: i32) -> bool {
        return x >= 0 && (x as usize) < self.size_x && y >= 0 && (y as usize) < self.size_y;
    }

    pub fn draw_line_1px(&mut self, v1_integer: (i32, i32), v2_integer: (i32, i32), color: &Color) {
        /*
           As of now, this function contains the only direct write-on-bytebuffer operation
           Update: next to canvas.draw_dot()
        */

        if !self.integer_coords_in_canvas(v1_integer.0, v1_integer.1) {
            println!(
                "Line endpoint is out of bounds ({}, {})",
                v1_integer.0, v1_integer.1
            );
            return;
        }
        if !self.integer_coords_in_canvas(v2_integer.0, v2_integer.1) {
            println!(
                "Line endpoint is out of bounds ({}, {})",
                v2_integer.0, v2_integer.1
            );
            return;
        }

        // Bresenhams

        let mut x_0: i32 = v1_integer.0;
        let mut y_0: i32 = v1_integer.1;
        let x_1: i32 = v2_integer.0;
        let y_1: i32 = v2_integer.1;

        let dx: i32 = (x_1 - x_0).abs();
        let sx: i32 = if x_0 < x_1 { 1 } else { -1 };
        let dy: i32 = -(y_1 - y_0).abs();
        let sy: i32 = if y_0 < y_1 { 1 } else { -1 };
        let mut error: i32 = dx + dy;

        loop {
            let target_in_buffer = ((self.size_y as f64 - 1.0 - y_0 as f64) * self.size_x as f64
                + x_0 as f64) as usize;
            // println!(
            //     "drawing pixel at ({x_0}, {y_0}) => buffer[{target_in_buffer}], buffer length: {}",
            //     self.buffer.len()
            // );
            if self.integer_coords_in_canvas(x_0, y_0) {
                self.buffer[target_in_buffer] = color.as_u32();
            }

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

    pub fn draw_dot(&mut self, center: (i32, i32), color: &Color) {
        if self.integer_coords_in_canvas(center.0, center.1) {
            self.buffer
                [((self.size_y as i32 - 1 - center.1) * self.size_x as i32 + center.0) as usize] =
                color.as_u32();
        }
    }

    pub fn draw_circle(&mut self, center: (i32, i32), radius: i32, color: &Color) {
        if self.integer_coords_in_canvas(center.0, center.1) {
            // center is within canvas bounds

            let r_squared = radius * radius;
            for y in -radius..radius + 1 {
                for x in -radius..radius + 1 {
                    if y * y + x * x <= r_squared {
                        self.draw_dot((center.0 + x, center.1 + y), color);
                    }
                }
            }

            self.buffer
                [((self.size_y as i32 - 1 - center.1) * self.size_x as i32 + center.0) as usize] =
                color.as_u32();
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
