use crate::graphics::Color;

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

    pub fn set_pixel(&mut self, coords: (i32, i32), color: &Color) {
        if self.integer_coords_in_canvas(coords.0, coords.1) {
            self.buffer
                [((self.size_y as i32 - 1 - coords.1) * self.size_x as i32 + coords.0) as usize] =
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
