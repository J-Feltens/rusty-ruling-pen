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

    pub range_x: (f64, f64),
    pub range_y: (f64, f64),

    pub buffer: Vec<u32>,

    up: Vector2d,
    down: Vector2d,
    left: Vector2d,
    right: Vector2d,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: &Color) -> Canvas {
        Canvas {
            size_x: size_x,
            size_y: size_y,

            range_x: (-3.0, 3.0),
            range_y: (-3.0, 3.0),
            buffer: vec![bg_color.as_u32(); size_x * size_y],

            up: Vector2d::new(0.0, 1.0),
            down: Vector2d::new(0.0, -1.0),
            left: Vector2d::new(-1.0, 0.0),
            right: Vector2d::new(1.0, 0.0),
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

    pub fn pan(&mut self, direction: Vector2d) {
        self.range_x.0 += direction.x;
        self.range_x.1 += direction.x;
        self.range_y.0 += direction.y;
        self.range_y.1 += direction.y;

        println!(
            "Current range: {} <= x <= {}, {} <= y <= {}",
            self.range_x.0, self.range_x.1, self.range_y.0, self.range_y.1
        );
    }

    pub fn zoom(&mut self, factor: f64) {
        self.range_x.0 *= factor;
        self.range_x.1 *= factor;
        self.range_y.0 *= factor;
        self.range_y.1 *= factor;
    }

    pub fn project_vec_to_canvas(&self, v: Vector2d) -> (i32, i32, bool) {
        /*
           projects a vector with f64 coords onto the discrete canvas using the range associated with the canvas

           returns:
                i32, i32        tupel of usize describing x- and y-coords and a boolean that is false if
                                target coordiantes are out of bounds
        */
        let out_of_bounds;
        if v.x >= self.range_x.0
            && v.x <= self.range_x.1
            && v.y >= self.range_y.0
            && v.y <= self.range_y.1
        {
            // println!(
            //     "Deemed {}, {} within of bounds for bounds x in ({}, {}) and y in ({}, {})",
            //     v.x, v.y, self.range_x.0, self.range_x.1, self.range_y.0, self.range_y.1
            // );
            out_of_bounds = false;
        } else {
            // println!(
            //     "Deemed {}, {} out of bounds for bounds x in ({}, {}) and y in ({}, {})",
            //     v.x, v.y, self.range_x.0, self.range_x.1, self.range_y.0, self.range_y.1
            // );
            out_of_bounds = true;
        }
        // map vector coordinates to canvas.range
        let x: i32 = map_range(
            v.x,
            self.range_x.0,
            self.range_x.1,
            0.0,
            self.size_x as f64 - 1.0,
        ) as i32;
        let y: i32 = map_range(
            v.y,
            self.range_y.0,
            self.range_y.1,
            0.0,
            self.size_y as f64 - 1.0,
        ) as i32;

        return (x, y, out_of_bounds || !self.integer_coords_in_canvas(x, y));
    }

    pub fn integer_coords_in_canvas(&self, x: i32, y: i32) -> bool {
        return x >= 0 && (x as usize) < self.size_x && y >= 0 && (y as usize) < self.size_y;
    }

    pub fn draw_line_1px(&mut self, v1: Vector2d, v2: Vector2d, color: &Color) {
        /*
           As of now, this function contains the only direct write-on-bytebuffer operation
           Update: next to canvas.draw_dot()
        */

        let v1_integer = self.project_vec_to_canvas(v1);
        let v2_integer = self.project_vec_to_canvas(v2);

        if v1_integer.2 {
            println!(
                "Line endpoint is out of bounds ({}, {})",
                v1_integer.0, v1_integer.1
            );
            return;
        }
        if v2_integer.2 {
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

    pub fn draw_line(&mut self, v1: Vector2d, v2: Vector2d, width: u32, color: &Color) {
        assert!(width >= 1);

        for i in 0..width {
            self.draw_line_1px(v1 + self.up * i as f64, v2 + self.up * i as f64, color);
            self.draw_line_1px(v1 + self.down * i as f64, v2 + self.down * i as f64, color);
            self.draw_line_1px(v1 + self.left * i as f64, v2 + self.left * i as f64, color);
            self.draw_line_1px(
                v1 + self.right * i as f64,
                v2 + self.right * i as f64,
                color,
            );
        }
    }

    pub fn draw_dot(&mut self, vector: Vector2d, color: &Color) {
        let center_in_canvas = self.project_vec_to_canvas(vector);
        if !center_in_canvas.2 {
            // center is within canvas bounds

            self.buffer[((self.size_y as i32 - 1 - center_in_canvas.1) * self.size_x as i32
                + center_in_canvas.0) as usize] = color.as_u32();
        }
    }

    pub fn calc_grid_spacing(&mut self) -> (f64, f64) {
        let dx = self.range_x.1 - self.range_x.0;
        let dy = self.range_y.1 - self.range_y.0;

        // figure out optimal spacing of grid ticks, straight up brute force approach
        let mut major_tick_spacing_x = 1.0 as f64;
        let mut major_tick_spacing_y = 1.0 as f64;
        loop {
            let tick_count_x = dx / major_tick_spacing_x;
            if tick_count_x < 2.0 {
                // less than two ticks for x axis, gotta decrease tick spacing
                major_tick_spacing_x *= 0.1;
            } else if tick_count_x > 20.0 {
                // too many damn ticks! gotta increase spacing
                major_tick_spacing_x *= 10.0;
            } else {
                break;
            }
        }
        loop {
            let tick_count_y = dy / major_tick_spacing_y;
            if tick_count_y < 2.0 {
                // less than two ticks for y ayis, gotta decrease tick spacing
                major_tick_spacing_y *= 0.1;
            } else if tick_count_y > 20.0 {
                // too many damn ticks! gotta increase spacing
                major_tick_spacing_y *= 10.0;
            } else {
                break;
            }
        }
        // println!(
        //     "Optimal major tick spacing: {}, {}",
        //     major_tick_spacing_x, major_tick_spacing_y
        // );

        return (major_tick_spacing_x, major_tick_spacing_y);
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
