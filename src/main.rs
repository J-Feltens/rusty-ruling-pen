use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::ascii::escape_default;
use std::collections::LinkedList;
use std::process::exit;

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::EdgeTableEntry;
use crate::graphics::{BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, WHITE, YELLOW};
use crate::vectors::Vector2d;

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 64;
const SIZE_Y: usize = 64;
const SCALE: minifb::Scale = minifb::Scale::X4;

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

    // define polygon
    let p1 = Vector2d::new(1.0, 1.0);
    let p2 = Vector2d::new(4.0, 7.0);
    let p3 = Vector2d::new(4.0, 4.0);
    let p4 = Vector2d::new(6.0, 5.0);
    let p5 = Vector2d::new(7.0, 3.0);

    let points = vec![p1, p2, p3, p4, p5];

    let mut edge_table = EdgeTable::new();
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        edge_table.add_edge(EdgeTableEntry::from_points(p1, p2));
    }
    edge_table.sort();

    edge_table.print();

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
