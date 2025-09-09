use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use crate::canvas::Canvas;
use crate::colors::Color;
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod sprites;
pub mod util;

const PI: f64 = 3.14159265359;
const WOBBLE_FAC_1: f64 = 0.5;
const WOBBLE_FAC_2: f64 = 2.0;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

const RADIUS: f64 = 20.0;
const DYNAMIC_SCROLL_SPEED: f64 = 0.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    // for fps
    let mut last_time_in_milliseconds = cur_time_in_milliseconds().unwrap();

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
        y: (500.0),
    };

    // init fallings and stack as main categories of obejcts
    let mut fallings = Vec::<Circle>::new();
    let mut stack = Vec::<Circle>::new();

    // initialize stack with first circle
    let mut new_circle: Circle = Circle::new(RADIUS, &magenta);
    new_circle.sprite.origin = Vector2d {
        x: (500.0),
        y: (500.0),
    };
    stack.push(new_circle);

    let gravity: Vector2d = Vector2d { x: (0.0), y: (2.0) };

    for i in 1..4 {
        let mut new_circle: Circle = Circle::new(RADIUS, &magenta);
        new_circle.sprite.origin = Vector2d {
            x: (500.0),
            y: (500.0),
        };
        stack.push(new_circle);
    }

    // main loop
    let mut should_scroll_down: f64 = 0.0;
    let mut frame: u128 = 0;
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            println!("");
            canvas.fill(&white);

            let mouse_pos: Vector2d = Vector2d {
                x: (mx as f64),
                y: (my as f64),
            };

            // increase stack size
            if frame % 100 == 0 {
                // should_scroll_down += RADIUS * 2.0;
            }

            // scroll if supposed too
            if should_scroll_down > 0.1 {
                let scroll_factor = 1.0;
                for i in 0..stack.len() {
                    stack[i].sprite.translate(Vector2d {
                        x: (0.0),
                        y: (scroll_factor),
                    });
                }
                should_scroll_down -= scroll_factor;
            } else {
                // floor down back to 0
                should_scroll_down = 0.0;
            }
            println!("Stack Size: {}", stack.len());

            // spawn some new falling
            // if rng.random_bool(0.01) {
            //     let mut falling: Circle = Circle::new(RADIUS, &cyan);
            //     falling.sprite.translate(Vector2d {
            //         x: rng.random_range(0.0..500.0),
            //         y: -100.0,
            //     });
            //     fallings.push(falling);
            // }

            // render falling
            let mut idxs_to_be_destroyed: Vec<usize> = Vec::new();
            if fallings.len() >= 0 {
                for (i, falling) in fallings.iter_mut().enumerate() {
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
            println!("Fallings Count: {}", fallings.len());

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
                // println!(
                //     "circle: {}, {}",
                //     circle.sprite.origin.x, circle.sprite.origin.y
                // );
                canvas.draw_sprite(&circle.sprite);
            }

            // render new framebuffer
            window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
        } else {
            println!("No mouse detected :(");
        };

        // for fps
        let cur_time = cur_time_in_milliseconds().unwrap() as u128;
        let fps = 1000.0 / (cur_time - last_time_in_milliseconds as u128) as f64;
        println!("FPS: {}", fps);
        last_time_in_milliseconds = cur_time;
        println!("Frame: {}", frame);
        frame += 1;
    }

    Ok(())
}

fn cur_time_in_milliseconds() -> Result<u128, SystemTimeError> {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH)?;
    let milliseconds_timestamp = duration_since_epoch.as_millis();

    Ok(milliseconds_timestamp)
}
