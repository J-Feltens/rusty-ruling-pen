use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::process::exit;

use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::vectors::Vector2d;

pub mod colors;
pub mod util;
pub mod vectors;

const SIZE_X: u32 = 64;
const SIZE_Y: u32 = 64;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X as usize,
        SIZE_Y as usize,
        WindowOptions::default(),
    )?;

    let mut buffer: Vec<u32> = vec![0; (SIZE_X * SIZE_Y) as usize];
    for i in 0..(SIZE_X * SIZE_Y) as usize {
        buffer[i] = Color::as_u32(&Color {
            r: (255),
            g: (255),
            b: (255),
            a: (1.0),
        })
    }

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        window.update_with_buffer(&buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
