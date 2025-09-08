use crate::colors::Color;

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
}
