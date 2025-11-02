use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::f64::consts::PI;
use std::process::exit;
use std::thread::sleep;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW, rgb2u32};
use crate::graphics::canvas::{self, Canvas};
use crate::planets::Planet;
use crate::util::lines::Line2d;
use crate::util::{Vector2d, Vector3d};

pub mod colors;
pub mod graphics;
pub mod planets;
pub mod util;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 800;
const SCALE: minifb::Scale = minifb::Scale::X1;

const DT: f64 = 0.01;
const G: f64 = 10_000_000.0;

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

    // master_canvas.set_viewport_offset(Vector2d::new(100.0, -100.0));

    let mut planets = vec![
        Planet::new(
            Vector2d::new(300.0, 100.0),
            1.0,
            Vector2d::new(0.0, 0.0),
            Vector2d::new(0.0, 0.0),
            CYAN.clone(),
        ),
        Planet::new(
            Vector2d::new(400.0, 600.0),
            3.0,
            Vector2d::new(0.0, 0.0),
            Vector2d::new(0.0, 0.0),
            MAGENTA.clone(),
        ),
        Planet::new(
            Vector2d::new(580.0, 230.0),
            1.0,
            Vector2d::new(0.0, 0.0),
            Vector2d::new(0.0, 0.0),
            YELLOW.clone(),
        ),
    ];

    let mut t: f64 = 0.0;
    while !window.is_key_down(Key::Q) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            println!("{}", termion::clear::All);
            t += DT;
            master_canvas.reset(&BLACK);

            // calc accelerations
            for planet in planets.iter_mut() {
                planet.acceleration = Vector2d::new(0.0, 0.0);
            }
            for i in 0..planets.len() {
                for j in 0..planets.len() {
                    if i == j {
                        continue;
                    }

                    let r_ij = planets[j].position - planets[i].position;
                    let dist = r_ij.length();
                    if dist == 0.0 {
                        continue;
                    }

                    // Newtonian gravity (F = G * m1 * m2 / r^2)
                    // Acceleration = F / m1 = G * m2 / r^2 (direction: r_ij)
                    planets[i].acceleration = planets[i].acceleration
                        + r_ij * (G * planets[j].mass / (dist * dist * dist));
                }
            }

            // calc veolcity and position
            for i in 0..planets.len() {
                planets[i].velocity = planets[i].velocity + planets[i].acceleration * DT;
                planets[i].position = planets[i].position + planets[i].velocity * DT;

                master_canvas.draw_circle(planets[i].pos_as_int(), 30, &planets[i].color);
            }

            println!("{}t = {:8.8}", termion::cursor::Goto(1, 1), t);
            for (i, planet) in planets.iter().enumerate() {
                println!(
                    "{}Planet {}:",
                    termion::cursor::Goto(1, (i * 5 + 3) as u16),
                    i + 1,
                );
                println!(
                    "{}Position:     {:8.8}, {:8.8}",
                    termion::cursor::Goto(2, (i * 5 + 4) as u16),
                    planet.position.x,
                    planet.position.y,
                );
                println!(
                    "{}Velocity:     {:8.8}, {:8.8}",
                    termion::cursor::Goto(2, (i * 5 + 5) as u16),
                    planet.velocity.x,
                    planet.velocity.y,
                );
                println!(
                    "{}Acceleration: {:8.8}, {:8.8}",
                    termion::cursor::Goto(2, (i * 5 + 6) as u16),
                    planet.acceleration.x,
                    planet.acceleration.y,
                );
            }

            for planet in planets.iter() {
                master_canvas.draw_circle(planet.pos_as_int(), 30, &planet.color);
            }

            window.update_with_buffer(&master_canvas.get_buffer(), SIZE_X, SIZE_Y)?;
            sleep(time::Duration::from_secs_f64(DT * 2.0));
        } else {
            println!("No mouse detected :(")
        }
    }
    Ok(())
}
