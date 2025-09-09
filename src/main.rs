use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::rand_core::impls::next_u32_via_fill;
use rand::{Rng, rng};
use std::process::exit;

use crate::canvas::Canvas;
use crate::colors::Color;
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod sprites;
pub mod util;

const PI: f64 = 3.14159265359;
const WOBBLE_FAC_1: f64 = 1.0;
const WOBBLE_FAC_2: f64 = 1.0;

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

    let transparent: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0.0,
    };
    let black: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    let white: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };
    let magenta: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 1.0,
    };
    let cyan: Color = Color {
        r: 0,
        g: 255,
        b: 255,
        a: 1.0,
    };
    let yellow: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 1.0,
    };

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
        x: (500.0),
        y: (600.0),
    };

    // init fallings and stack as main categories of obejcts
    let mut fallings = Vec::<Circle>::new();
    let mut stack = Vec::<Circle>::new();

    let gravity: Vector2d = Vector2d { x: (0.0), y: (2.0) };

    for i in 1..100 {
        let mut new_circle: Circle = Circle::new(5.0, &magenta);
        new_circle.sprite.translate(Vector2d {
            x: world_origin.x,
            y: world_origin.y - i as f64 * 10.0,
        });
        stack.push(new_circle);
    }

    let stack_size = stack.len();

    // main loop
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            println!("Mouse position: ({}, {})", mx, my);
            canvas.fill(&white);

            let mouse_pos: Vector2d = Vector2d {
                x: (mx as f64),
                y: (my as f64),
            };

            // spawn some new falling
            if rng.random_bool(0.01) {
                let mut falling: Circle = Circle::new(50.0, &cyan);
                falling.sprite.translate(Vector2d {
                    x: rng.random_range(0.0..500.0),
                    y: -100.0,
                });
                fallings.push(falling);
            }

            // render falling
            let mut idxs_to_be_destroyed: Vec<usize> = Vec::new();
            if fallings.len() > 0 {
                for (i, falling) in fallings.iter_mut().enumerate() {
                    println!(
                        "x: {}, y: {}",
                        falling.sprite.origin.x, falling.sprite.origin.x
                    );
                    falling.sprite.origin += gravity;

                    // render
                    canvas.draw_sprite(&falling.sprite);
                    if falling.sprite.origin.y > (canvas.size_y + falling.sprite.size_y) as f64 {
                        // destroy if outside of canvas
                        idxs_to_be_destroyed.push(i)
                    }
                }
            }
            for i in idxs_to_be_destroyed.iter() {
                fallings.remove(i.clone());
            }
            println!("{}", fallings.len());

            // render stack
            for (i, circle) in stack.iter_mut().enumerate() {
                let mut delta_object_mouse =
                    (mouse_pos - circle.sprite.origin) * Vector2d { x: (1.0), y: (0.0) };

                // calc and apply inertia
                let mut inertia: f64 = WOBBLE_FAC_1 / (i as f64 * WOBBLE_FAC_2);
                if inertia < 0.001 {
                    inertia = 0.001
                }
                delta_object_mouse.scale(inertia);

                circle.sprite.translate(delta_object_mouse);
                canvas.draw_sprite(&circle.sprite);
            }

            // render new framebuffer
            window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
        } else {
            println!("No mouse detected :(");
        };
    }

    Ok(())
}
