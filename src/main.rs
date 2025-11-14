use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::ascii::escape_default;
use std::collections::LinkedList;
use std::process::exit;
use std::{thread, time};

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::{
    ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTableEntry, draw_polygon_onto_buffer,
};
use crate::graphics::{
    BLACK, BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
use crate::vectors::{IntegerVector2d, Vector2d};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const SCALE: minifb::Scale = minifb::Scale::X1;
const ANIM_INTERVAL: time::Duration = time::Duration::from_millis(100);

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
            topmost: true,
            transparency: false,
            none: false,
        },
    )?;

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, &WHITE);

    // define polygon
    let scale = 30;

    let p1 = IntegerVector2d::new(2 * scale, 5 * scale, vec![1.0, 0.0, 0.0]);
    let p2 = IntegerVector2d::new(14 * scale, 14 * scale, vec![0.0, 1.0, 0.0]);
    let p3 = IntegerVector2d::new(8 * scale, 2 * scale, vec![0.0, 0.0, 1.0]);

    let mut points = vec![p1, p2, p3];

    canvas.checker(
        &WHITE,
        &Color {
            r: (200),
            g: (200),
            b: (200),
            a: (1.0),
        },
    );

    // finally, draw polygon
    draw_polygon_onto_buffer(&points, &mut canvas, &BLACK, false);

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
