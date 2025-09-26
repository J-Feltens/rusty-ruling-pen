use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::f64::consts::PI;
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW, rgb2u32};
use crate::graphics::canvas::{self, Canvas};
use crate::util::lines::{Line2d, Lines3d};
use crate::util::{CoordSystem, Line3d, Vector2d, Vector3d, coordinate_system, cube};

pub mod colors;
pub mod graphics;
pub mod util;

const SIZE_X: usize = 200;
const SIZE_Y: usize = 200;

// the quick'n dirty
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X,
        SIZE_Y,
        WindowOptions {
            borderless: false,
            title: true,
            scale: minifb::Scale::X4,
            resize: false,
            scale_mode: minifb::ScaleMode::UpperLeft,
            topmost: false,
            transparency: false,
            none: false,
        },
    )?;

    let mut master_canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y, &CYAN);
    master_canvas.set_viewport_offset(Vector2d::new(100.0, -100.0));

    let word_origin: Vector3d = Vector3d::new(0.0, 0.0, 0.0);
    let mut world_coord_system: CoordSystem = CoordSystem::new(word_origin);

    let mut lines = cube(50.0, word_origin + Vector3d::new(25.0, 25.0, 25.0), &WHITE);
    // let mut lines = cube(50.0, word_origin);

    while !window.is_key_down(Key::Enter) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            let x = 0.005;
            let y = 0.005;
            let z = 0.005;

            let keys_down: Vec<Key> = window.get_keys();

            for key in keys_down.iter() {
                for line in lines.iter_mut() {
                    line.v1 -= word_origin;
                    line.v2 -= word_origin;
                    if window.is_key_down(Key::W) {
                        line.v1.rotate_euler_xyz(x, 0.0, 0.0);
                        line.v2.rotate_euler_xyz(x, 0.0, 0.0);
                    }
                    if window.is_key_down(Key::S) {
                        line.v1.rotate_euler_xyz(-x, 0.0, 0.0);
                        line.v2.rotate_euler_xyz(-x, 0.0, 0.0);
                    }
                    if window.is_key_down(Key::A) {
                        line.v1.rotate_euler_xyz(0.0, y, 0.0);
                        line.v2.rotate_euler_xyz(0.0, y, 0.0);
                    }
                    if window.is_key_down(Key::D) {
                        line.v1.rotate_euler_xyz(0.0, -y, 0.0);
                        line.v2.rotate_euler_xyz(0.0, -y, 0.0);
                    }
                    if window.is_key_down(Key::Q) {
                        line.v1.rotate_euler_xyz(0.0, 0.0, z);
                        line.v2.rotate_euler_xyz(0.0, 0.0, z);
                    }
                    if window.is_key_down(Key::E) {
                        line.v1.rotate_euler_xyz(0.0, 0.0, -z);
                        line.v2.rotate_euler_xyz(0.0, 0.0, -z);
                    }
                    line.v1 += word_origin;
                    line.v2 += word_origin;
                }
            }

            master_canvas.reset(&CYAN);
            for line in lines.iter() {
                master_canvas.draw_3d_line(*line, 1);
            }
            for line in world_coord_system.get_lines().iter() {
                master_canvas.draw_3d_line(*line, 1);
            }

            window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
        } else {
            println!("No mouse detected :(")
        }
    }
    Ok(())
}
