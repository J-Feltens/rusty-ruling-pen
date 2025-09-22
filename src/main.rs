use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW, rgb2u32};
use crate::graphics::canvas::Canvas;
use crate::util::Vector2d;

pub mod colors;
pub mod graphics;
pub mod util;

const SIZE_X: usize = 201;
const SIZE_Y: usize = 121;

// the quick'n dirty
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X,
        SIZE_Y,
        WindowOptions {
            borderless: false,
            title: true,
            scale: minifb::Scale::X1,
            resize: false,
            scale_mode: minifb::ScaleMode::UpperLeft,
            topmost: false,
            transparency: false,
            none: false,
        },
    )?;

    let mut master_canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y, CYAN);

    let mut mini_canvas: Canvas = Canvas::new(200, 120, YELLOW);

    let mut v1: Vector2d = Vector2d::new(10.0, 10.0);
    let mut v2: Vector2d = Vector2d::new(190.0, 110.0);

    mini_canvas.draw_line(v1, v2, &MAGENTA);

    master_canvas.add_layer(mini_canvas, 0, 0);

    while !window.is_key_down(Key::Enter) {
        if 
        window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
    }
    Ok(())
}
