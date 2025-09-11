use crate::colors::Color;
use crate::sprites::Sprite;
use crate::util::Vector2d;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Canvas {
    pub buffer: Vec<u32>,
    pub size_x: u32,
    pub size_y: u32,
}

impl Canvas {
    pub fn new(size_x: u32, size_y: u32) -> Canvas {
        let mut buffer: Vec<u32> = vec![0; (size_x * size_y) as usize];
        for i in 0..(size_x * size_y) as usize {
            buffer[i] = Color::as_u32(&Color {
                r: (255),
                g: (255),
                b: (255),
                a: (1.0),
            })
        }

        Canvas {
            buffer: buffer,
            size_x: size_x,
            size_y: size_y,
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        for y_idx in 0..sprite.size_y as usize {
            for x_idx in 0..sprite.size_x as usize {
                let x_target_in_canvas: f64 = sprite.origin.x + x_idx as f64;
                let y_target_in_canvas: f64 = sprite.origin.y + y_idx as f64;

                // check if the pixel of the sprite is within canvas boundries
                if 0.0 < x_target_in_canvas && x_target_in_canvas < self.size_x as f64 {
                    if 0.0 < y_target_in_canvas && y_target_in_canvas < self.size_y as f64 {
                        // very simple alpha channel only displaying alpha = 1.0 and higher
                        if sprite.grid[y_idx * sprite.size_x as usize + x_idx].a == 1.0 {
                            // assign pixel value in sprite to pixel value in canvas
                            self.buffer[(y_target_in_canvas * self.size_x as f64
                                + x_target_in_canvas)
                                as usize] =
                                sprite.grid[y_idx * sprite.size_x as usize + x_idx].as_u32()
                        } else {
                            // if alpha lower than 1.0 draw white
                            // self.buffer[(y_target_in_canvas * self.size_x as f64
                            //     + x_target_in_canvas)
                            //     as usize] = 0
                        }
                    }
                }
            }
        }
    }

    pub fn fill(&mut self, color: &Color) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = color.as_u32();
        }
    }
}
