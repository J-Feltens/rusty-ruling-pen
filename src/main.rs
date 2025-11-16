use std::time::Instant;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::time;

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::{
    ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTableEntry, draw_polygon_onto_buffer,
};
use crate::graphics::{
    BLACK, BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
use crate::util::interpolate1d;
use crate::vectors::ivec2d::Polygon2d;
use crate::vectors::{IntegerVector2d, Vector2d};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 16;
const SIZE_Y: usize = 16;
const SCALE: minifb::Scale = minifb::Scale::X32;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut global_timer = Instant::now();

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

    // canvas.checker(
    //     &WHITE,
    //     &Color {
    //         r: (0),
    //         g: (0),
    //         b: (0),
    //         a: (0.1),
    //     },
    // );

    // define polygon
    let p1 = IntegerVector2d::new(2, 5, vec![1.0, 0.0, 0.0, 0.0]);
    let p2 = IntegerVector2d::new(14, 14, vec![0.0, 1.0, 0.0, 0.0]);
    let p3 = IntegerVector2d::new(8, 2, vec![0.0, 0.0, 1.0, 1.0]);

    let triangle = Polygon2d::new(vec![p1, p2, p3]);

    // finally, draw polygon
    draw_polygon_onto_buffer(&triangle.vertices, &mut canvas, false);

    println!("Rendertime: {} ms", global_timer.elapsed().as_millis());

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop

        // update minifb with new buffer
        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }

    Ok(())
}
