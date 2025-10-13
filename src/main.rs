use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::f64::consts::PI;
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW, rgb2u32};
use crate::graphics::canvas::{self, Canvas};
use crate::util::lines::Line2d;
use crate::util::{Vector2d, Vector3d};

pub mod colors;
pub mod graphics;
pub mod util;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 800;
const SCALE: minifb::Scale = minifb::Scale::X1;

// the quick'n dirty
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X,
        SIZE_Y,
        WindowOptions {
            borderless: false,
            title: true,
            scale: SCALE,
            resize: false,
            scale_mode: minifb::ScaleMode::UpperLeft,
            topmost: false,
            transparency: false,
            none: false,
        },
    )?;

    let mut master_canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y, &CYAN);

    // master_canvas.set_viewport_offset(Vector2d::new(100.0, -100.0));

    let word_origin: Vector3d = Vector3d::new(0.0, 0.0, 0.0);

    while !window.is_key_down(Key::Enter) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            master_canvas.reset(&BLACK);

            master_canvas.draw_circle((400, 400), 100, &CYAN);

            window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
        } else {
            println!("No mouse detected :(")
        }
    }
    Ok(())
}
