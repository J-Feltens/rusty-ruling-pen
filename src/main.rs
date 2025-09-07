use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::rand_core::impls::next_u32_via_fill;
use rand::{Rng, rng};
use std::process::exit;

use crate::canvas::Canvas;
use crate::colors::Color;
use crate::sprites::{Circle, Sprite};
use crate::util::{Object, Stack, Vector2d};

pub mod canvas;
pub mod colors;
pub mod sprites;
pub mod util;

const PI: f64 = 3.14159265359;
const WOBBLE_FAC_1: f64 = 0.08;
const WOBBLE_FAC_2: f64 = 5.0;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

fn draw_circle(buffer: &mut Vec<u32>, x: u32, y: u32, r: u32, color: u32) {
    for y_ in 0..SIZE_X {
        for x_ in 0..SIZE_Y {
            if (x_ as i64 - x as i64).pow(2) + (y_ as i64 - y as i64).pow(2) < r as i64 {
                buffer[(y_ * SIZE_X + x_) as usize] = color;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    let white: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let black: Color = Color { r: 0, g: 0, b: 0 };

    // initialize 32 bit buffer as canvas
    let mut canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y);

    // create a window and show the buffer
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X as usize,
        SIZE_Y as usize,
        WindowOptions::default(),
    )?;

    let world_origin = Vector2d {
        x: (0.0),
        y: (300.0),
    };

    let mut stack = Vec::<Circle>::new();

    for i in 1..10 {
        stack.push(Circle::new(50.0, black.clone()));
    }

    let stack_size = stack.len().clone();

    // main loop
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            println!("Mouse position: ({}, {})", mx, my);

            // initialize new frame buffer (very inefficient, but it will need to make due for now)
            let mut next_canvas = canvas.clone();

            let mouse_pos: Vector2d = Vector2d {
                x: (mx as f64),
                y: (my as f64),
            };

            for (i, circle) in stack.iter_mut().enumerate() {
                let mut delta_object_mouse =
                    (mouse_pos - circle.sprite.origin) * Vector2d { x: (1.0), y: (0.0) };

                // calc and apply inertia
                let mut inertia: f64 =
                    WOBBLE_FAC_1 * (stack_size as f64 - (i as f64 * WOBBLE_FAC_2).sqrt());
                if inertia < 0.001 {
                    inertia = 0.001
                }
                delta_object_mouse.scale(inertia);

                circle.sprite.origin += delta_object_mouse;
                circle.draw_on_buffer(
                    &mut next_canvas.buffer,
                    next_canvas.size_x,
                    next_canvas.size_y,
                );
            }

            // render new framebuffer
            window.update_with_buffer(&next_canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
        } else {
            println!("No mouse detected :(");
        };
    }

    Ok(())
}
