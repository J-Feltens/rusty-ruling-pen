use core::f64;

use crate::{
    graphics::{Color, alpha_blend, rgb2u32},
    util::interpolate1d,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub size_x: usize,
    pub size_y: usize,

    pub buffer: Vec<u32>,
    pub z_buffer: Vec<f64>,
    pub bg_color: Color,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: Color) -> Canvas {
        Canvas {
            size_x: size_x,
            size_y: size_y,

            buffer: vec![bg_color.as_u32(); size_x * size_y],
            z_buffer: vec![f64::MAX; size_x * size_y],
            bg_color: bg_color,
        }
    }

    pub fn reset(&mut self) {
        self.buffer = vec![self.bg_color.as_u32(); self.size_x * self.size_y];
    }

    pub fn reset_z_buffer(&mut self) {
        for i in 0..self.z_buffer.len() {
            self.z_buffer[i] = f64::MAX;
        }
    }

    pub fn checker(&mut self, color_1: &Color, color_2: &Color) {
        let colors = vec![color_1, color_2];
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                self.set_pixel((x as i32, y as i32), colors[(x + (y % 2)) % 2]);
            }
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        return &self.buffer;
    }

    pub fn integer_coords_in_canvas(&self, x: i32, y: i32) -> bool {
        return x >= 0 && (x as usize) < self.size_x && y >= 0 && (y as usize) < self.size_y;
    }

    pub fn set_pixel(&mut self, coords: (i32, i32), color: &Color) {
        // only draw pixel if it is in buffer bounds, will pass silently
        if self.integer_coords_in_canvas(coords.0, coords.1) {
            let integer_coord_in_buffer =
                ((self.size_y as i32 - 1 - coords.1) * self.size_x as i32 + coords.0) as usize;

            let color_from = &Color::from_u32(self.buffer[integer_coord_in_buffer]);

            // alpha-blend
            self.buffer[integer_coord_in_buffer] = alpha_blend(color_from, &color).as_u32();
        } else {
            println!("Drawing outside of canvas!");
        }
    }

    pub fn set_pixel_with_z(&mut self, coords: (i32, i32), z: f64, color: &Color) {
        // only draw pixel if it is in buffer bounds, will pass silently
        if self.integer_coords_in_canvas(coords.0, coords.1) {
            let integer_coord_in_buffer =
                ((self.size_y as i32 - 1 - coords.1) * self.size_x as i32 + coords.0) as usize;

            if z < self.z_buffer[integer_coord_in_buffer] {
                self.set_pixel(coords, color);
                self.z_buffer[integer_coord_in_buffer] = z;
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
