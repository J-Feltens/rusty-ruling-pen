use std::process::exit;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::Rng;
use rand::rng;

use crate::canvas::HEIGHT;
use crate::canvas::WIDTH;
use crate::colors::Color;
use crate::util::{Object, Stack, Vector2d};

pub mod canvas;
pub mod colors;
pub mod util;

const PI: f64 = 3.14159265359;
const WOBBLE_FAC_1: f64 = 0.08;
const WOBBLE_FAC_2: f64 = 5.0;

fn draw_circle(buffer: &mut Vec<u32>, x: u32, y: u32, r: u32, color: u32) {
    for y_ in 0..HEIGHT {
        for x_ in 0..WIDTH {
            if (x_ as i64 - x as i64).pow(2) + (y_ as i64 - y as i64).pow(2) < r as i64 {
                buffer[y_ * WIDTH + x_] = color;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    let white: Color = Color::from_rgb(255, 255, 255);
    let black: Color = Color::from_rgb(0, 0, 0);

    // initialize 32 bit buffer as canvas
    let mut buffer: Vec<u32> = vec![white.c; canvas::WIDTH * canvas::HEIGHT];
    for y in 0..canvas::HEIGHT {
        for x in 0..canvas::WIDTH {
            buffer[y * canvas::WIDTH + x] = white.c;
        }
    }

    // create a window and show the buffer
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        canvas::WIDTH,
        canvas::HEIGHT,
        WindowOptions::default(),
    )?;

    let world_origin = Vector2d {
        x: (0.0),
        y: (300.0),
    };

    let mut stack: Stack = Stack::new();

    for i in 1..10 {
        stack.add_object(Object {
            origin: (world_origin
                - Vector2d {
                    x: (0.0),
                    y: (25.0 * i as f64),
                }),
            r: (50.0),
            color: (black.c),
        });
    }

    let stack_size = stack.len().clone();

    // main loop
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            println!("Mouse position: ({}, {})", mx, my);

            // initialize new frame buffer (very inefficient, but it will need to make due for now)
            let mut buffer: Vec<u32> = vec![white.c; canvas::WIDTH * canvas::HEIGHT];
            for y in 0..canvas::HEIGHT {
                for x in 0..canvas::WIDTH {
                    buffer[y * canvas::WIDTH + x] = white.c;
                }
            }

            let mouse_pos: Vector2d = Vector2d {
                x: (mx as f64),
                y: (my as f64),
            };

            for (i, obj) in stack.iter_mut().enumerate() {
                let mut delta_object_mouse =
                    (mouse_pos - obj.origin) * Vector2d { x: (1.0), y: (0.0) };

                // calc and apply inertia
                let mut inertia: f64 =
                    WOBBLE_FAC_1 * (stack_size as f64 - (i as f64 * WOBBLE_FAC_2).sqrt());
                if inertia < 0.001 {
                    inertia = 0.001
                }
                delta_object_mouse.scale(inertia);

                obj.translate(delta_object_mouse);
                obj.draw_on_buffer(&mut buffer);
            }

            // render new framebuffer
            window.update_with_buffer(&buffer, canvas::WIDTH, canvas::HEIGHT)?;
        } else {
            println!("No mouse detected :(");
        };
    }

    Ok(())
}
