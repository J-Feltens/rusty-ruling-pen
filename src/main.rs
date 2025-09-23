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

const SIZE_X: usize = 1000;
const SIZE_Y: usize = 800;

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

    let mut master_canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y, &CYAN);

    let mut mini_canvas: Canvas = Canvas::new(200, 120, &YELLOW);
    master_canvas.add_layer(mini_canvas.clone(), 0, 0);

    let mut v1: Vector2d = Vector2d::new(10.0, 10.0);
    let mut v2: Vector2d = Vector2d::new(190.0, 110.0);

    master_canvas.draw_line_1px(v1, v2, &MAGENTA);

    while !window.is_key_down(Key::Enter) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            v2.set_v(mouse_x as f64, mouse_y as f64);

            master_canvas.reset(&CYAN);
            master_canvas.draw_line(v1, v2, 3, &MAGENTA);

            master_canvas.add_layer(mini_canvas.clone(), 0, 0);
            window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
        } else {
            println!("No mouse detected :(")
        }
    }
    Ok(())
}
