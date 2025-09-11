use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod sprites;
pub mod util;

const TARGET_FPS: f64 = 100.0;
const TARGET_INTERVAL_MS: u128 = (1000.0 / TARGET_FPS) as u128;

const WOBBLE_FAC_1: f64 = 0.5;
const WOBBLE_FAC_2: f64 = 2.0;

const GRAVITY: Vector2d = Vector2d { x: (0.0), y: (0.5) };
const COLLISION_RADIUS: f64 = 1.0;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

const RADIUS: f64 = 50.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    // for fps
    let mut last_time_ms = cur_time_in_milliseconds().unwrap();

    // initialize 32 bit buffer as canvas
    let mut canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y);

    // create a window and show the buffer
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X as usize,
        SIZE_Y as usize,
        WindowOptions::default(),
    )?;

    // init fallings and stack as main categories of obejcts
    let mut fallings = Vec::<Circle>::new();
    let mut stack = Vec::<Circle>::new();

    for i in 1..2 {
        let mut new_circle: Circle = Circle::new(RADIUS, &MAGENTA);
        new_circle.sprite.origin = Vector2d {
            x: (500.0),
            y: ((SIZE_Y / 2) as f64),
        };
        // translate according to stack index and stack origin
        new_circle.sprite.origin.y += -i as f64 * 2.0 * RADIUS;
        stack.push(new_circle);
    }

    // main loop
    let mut frame: u128 = 0;
    while window.is_open() && !window.is_key_down(Key::Enter) {
        if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
            canvas.fill(&WHITE);

            let mouse_pos: Vector2d = Vector2d {
                x: (mx as f64),
                y: (my as f64),
            };

            // scroll stack down
            if stack[stack.len() - 1].sprite.origin.y < 200.0 {
                for circle in stack.iter_mut() {
                    circle.sprite.origin.y += 1.0;
                }
            }

            println!("Stack Size: {}", stack.len());

            // spawn some new falling
            // if rng.random_bool(0.01) {
            if fallings.len() <= 0 {
                let mut falling: Circle = Circle::new(RADIUS, &CYAN);
                falling.sprite.translate(Vector2d {
                    x: rng.random_range(0.0..500.0),
                    y: -100.0,
                });
                fallings.push(falling);
            }

            // render falling
            let mut idxs_to_be_destroyed: Vec<usize> = Vec::new();
            if fallings.len() >= 0 {
                for (i, falling) in fallings.iter_mut().enumerate() {
                    falling.sprite.origin += GRAVITY;

                    // render
                    canvas.draw_sprite(&falling.sprite);
                    if falling.sprite.origin.y > (canvas.size_y + falling.sprite.size_y) as f64 {
                        // destroy if outside of canvas
                        idxs_to_be_destroyed.push(i)
                    }
                }
            }
            // collision
            for (i, falling) in fallings.iter_mut().enumerate() {
                let dist_to_circle = falling
                    .sprite
                    .distance_to_sprite(&stack[stack.len() - 1].sprite);

                // increase stack size
                if dist_to_circle <= COLLISION_RADIUS {
                    println!("Collision detected!");
                    let mut new_circle: Circle = Circle::new(RADIUS, &MAGENTA);
                    new_circle.sprite.origin = falling.sprite.origin.clone();
                    new_circle.sprite.origin.y = stack[stack.len() - 1].sprite.origin.y;
                    new_circle.sprite.origin.y -= 2.0 * RADIUS;
                    stack.push(new_circle);
                    idxs_to_be_destroyed.push(i);
                }
            }
            for i in idxs_to_be_destroyed.iter() {
                fallings.remove(i.clone());
            }
            println!("Fallings Count: {}", fallings.len());

            // render stack
            for (i, circle) in stack.iter_mut().enumerate() {
                // translate to mouse (more or less)
                let mut delta_object_mouse =
                    (mouse_pos - circle.sprite.origin) * Vector2d { x: (1.0), y: (0.0) };

                // calc and apply inertia
                let mut inertia: f64 = if i == 0 {
                    1.0
                } else {
                    WOBBLE_FAC_1 / (i as f64 * WOBBLE_FAC_2)
                };
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

        // sleep to stay within target fps
        let cur_time_ms = cur_time_in_milliseconds().unwrap() as u128;
        let time_diff_ms = cur_time_ms - last_time_ms;
        if TARGET_INTERVAL_MS > time_diff_ms {
            thread::sleep(time::Duration::from_millis(
                TARGET_INTERVAL_MS as u64 - time_diff_ms as u64,
            ));
        }
        print!("{}[2J", 27 as char);

        // calc and display fps
        let cur_time_ms = cur_time_in_milliseconds().unwrap() as u128;
        let fps = 1000.0 / (cur_time_ms - last_time_ms as u128) as f64;
        println!("FPS: {}", fps);

        println!("Frame: {}", frame);
        frame += 1;
        last_time_ms = cur_time_ms;
        // uncomment for single frame exec
        // exit(0);
    }

    Ok(())
}

fn cur_time_in_milliseconds() -> Result<u128, SystemTimeError> {
    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH)?;
    let milliseconds_timestamp = duration_since_epoch.as_millis();

    Ok(milliseconds_timestamp)
}
