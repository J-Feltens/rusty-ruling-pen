use crate::colors::Color;
use crate::sprites::Sprite;

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
            })
        }

        Canvas {
            buffer: buffer,
            size_x: size_x,
            size_y: size_y,
        }
    }

    pub fn draw_sprite(&mut self, sprite: Sprite) {
        for y_idx in 0..sprite.size_y as usize {
            for x_idx in 0..sprite.size_x as usize {
                let x_target_in_canvas: u32 = sprite.origin.x as u32 + x_idx as u32;
                let y_target_in_canvas: u32 = sprite.origin.y as u32 + y_idx as u32;

                if 0 < x_target_in_canvas && x_target_in_canvas < self.size_x {
                    if 0 < y_target_in_canvas && y_target_in_canvas < self.size_y {
                        self.buffer
                            [(y_target_in_canvas * self.size_x + x_target_in_canvas) as usize] =
                            sprite.grid[y_idx * sprite.size_x as usize + x_idx].as_u32()
                    }
                }
            }
        }
    }
}
