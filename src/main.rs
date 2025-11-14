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
            topmost: false,
            transparency: false,
            none: false,
        },
    )?;

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, &WHITE);
    canvas.checker(
        &WHITE,
        &Color {
            r: (200),
            g: (200),
            b: (200),
            a: (1.0),
        },
    );

    // define polygon
    let scale = 2.0;

    let p1 = IntegerVector2d::from_floats(100.0 * scale, 10.0 * scale);
    let p2 = IntegerVector2d::from_floats(120.0 * scale, 72.0 * scale);
    let p3 = IntegerVector2d::from_floats(186.0 * scale, 72.0 * scale);
    let p4 = IntegerVector2d::from_floats(136.0 * scale, 112.0 * scale);
    let p5 = IntegerVector2d::from_floats(153.0 * scale, 173.0 * scale);
    let p6 = IntegerVector2d::from_floats(100.0 * scale, 138.0 * scale);
    let p7 = IntegerVector2d::from_floats(47.0 * scale, 173.0 * scale);
    let p8 = IntegerVector2d::from_floats(64.0 * scale, 112.0 * scale);
    let p9 = IntegerVector2d::from_floats(14.0 * scale, 72.0 * scale);
    let p10 = IntegerVector2d::from_floats(80.0 * scale, 72.0 * scale);

    let points = vec![p1, p2, p3, p4, p5, p6, p7, p8, p9, p10];

    // draw polygon corners
    // for p in points.iter() {
    //     canvas.set_pixel((p.x, p.y), &MAGENTA);
    // }

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        draw_polygon_onto_buffer(&points, &mut canvas, &BLACK, false);

        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
