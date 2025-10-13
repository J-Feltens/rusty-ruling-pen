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

    let mut mini_canvas = Canvas::new(700, 700, &WHITE);
    mini_canvas.set_range(-1.0, 1.0, -1.0, 1.0);

    // master_canvas.set_viewport_offset(Vector2d::new(100.0, -100.0));

    let word_origin: Vector3d = Vector3d::new(0.0, 0.0, 0.0);

    while !window.is_key_down(Key::Enter) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            mini_canvas.reset(&WHITE);

            let keys_down: Vec<Key> = window.get_keys();

            for key in keys_down.iter() {
                if window.is_key_pressed(Key::W, minifb::KeyRepeat::Yes) {
                    mini_canvas.pan(Vector2d::down(0.01));
                }
                if window.is_key_pressed(Key::S, minifb::KeyRepeat::Yes) {
                    mini_canvas.pan(Vector2d::up(0.01));
                }
                if window.is_key_pressed(Key::A, minifb::KeyRepeat::Yes) {
                    mini_canvas.pan(Vector2d::right(0.01));
                }
                if window.is_key_pressed(Key::D, minifb::KeyRepeat::Yes) {
                    mini_canvas.pan(Vector2d::left(0.01));
                }
                if window.is_key_pressed(Key::E, minifb::KeyRepeat::Yes) {
                    mini_canvas.zoom(1.05);
                }
                if window.is_key_pressed(Key::Q, minifb::KeyRepeat::Yes) {
                    mini_canvas.zoom(0.95);
                }
            }

            // calc dotted grid
            let (major_tick_spacing_x, major_tick_spacing_y) = mini_canvas.calc_grid_spacing();
            let (minor_tick_spacing_x, minor_tick_spacing_y) =
                (major_tick_spacing_x / 10.0, major_tick_spacing_y / 10.0);

            let mut cur_x = mini_canvas.range_x.0.floor();
            let mut cur_y = mini_canvas.range_y.0.floor();

            // draw minor ticks
            while cur_x < mini_canvas.range_x.1 {
                let mut cur_y = mini_canvas.range_y.0.floor();
                while cur_y < mini_canvas.range_y.1 {
                    mini_canvas.draw_dot(Vector2d::new(cur_x, cur_y), &BLACK);
                    cur_y += minor_tick_spacing_y;
                }
                cur_x += minor_tick_spacing_x;
            }

            // draw major ticks
            let mut cur_x = mini_canvas.range_x.0.floor();
            let mut cur_y = mini_canvas.range_y.0.floor();
            while cur_x < mini_canvas.range_x.0 {
                cur_x += major_tick_spacing_x;
            }
            while cur_y < mini_canvas.range_y.0 {
                cur_y += major_tick_spacing_y;
            }

            while cur_x < mini_canvas.range_x.1 {
                mini_canvas.draw_line(
                    Vector2d::new(cur_x as f64, mini_canvas.range_y.0),
                    Vector2d::new(cur_x as f64, mini_canvas.range_y.1),
                    1,
                    &BLACK,
                );
                cur_x += major_tick_spacing_x;
            }
            while cur_y < mini_canvas.range_y.1 {
                mini_canvas.draw_line(
                    Vector2d::new(mini_canvas.range_x.0, cur_y),
                    Vector2d::new(mini_canvas.range_x.1, cur_y),
                    1,
                    &BLACK,
                );
                cur_y += major_tick_spacing_y;
            }

            master_canvas.reset(&CYAN);
            master_canvas.add_layer(mini_canvas.clone(), 50, 50);

            window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
        } else {
            println!("No mouse detected :(")
        }
    }
    Ok(())
}
