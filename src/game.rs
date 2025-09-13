use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use std::iter::Enumerate;
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::sprites::primitives::Frame;
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;
use crate::{SIZE_X, SIZE_Y};

const RADIUS: f64 = 30.0;
const Y_LEVEL: f64 = 300.0;
const COLORS: [Color; 3] = [CYAN, YELLOW, MAGENTA];

#[derive(Debug)]
pub struct Game {
    x_size: u32,
    y_size: u32,
    target_fps: u32,
    target_interval_ms: u128,

    gravity: Vector2d,

    fallings: Vec<Circle>,
    players: Vec<Circle>,
    stack_root: f64,
    frames: Vec<Frame>,
    windows: Vec<Window>,
    rng: ThreadRng,
}

impl Game {
    pub fn new(x_size: u32, y_size: u32, target_fps: u32) -> Game {
        Game {
            x_size: x_size,
            y_size: y_size,
            target_fps: target_fps,
            target_interval_ms: (1000 / target_fps) as u128,
            fallings: Vec::new(),
            players: Vec::new(),
            windows: Vec::new(),
            frames: Vec::new(),
            rng: rand::rng(),
            gravity: Vector2d { x: 0.0, y: 3.0 },
            stack_root: Y_LEVEL,
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..2 {
            let mut player = Circle::new(RADIUS, &CYAN);
            self.players.push(player);
        }

        let mut window = Window::new(
            "RRP (Rusty Ruling Pen)",
            self.x_size as usize,
            self.y_size as usize,
            WindowOptions::default(),
        )?;
        self.windows.push(window);

        let mut frame: Frame = Frame::new(SIZE_X, SIZE_Y, 10, CYAN);
        frame.sprite.recalc_pixel_idxs();
        self.frames.push(frame);

        Ok(())
    }

    pub fn test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // initialize 32 bit buffer as canvas
        let mut canvas: Canvas = Canvas::new(self.x_size, self.y_size);
        while self.windows[0].is_open() && !self.windows[0].is_key_down(Key::Enter) {
            // game loop

            // update window with rendered framebuffer
            self.windows[0].update_with_buffer(
                &canvas.buffer,
                self.x_size as usize,
                self.y_size as usize,
            )?;
        }
        Ok(())
    }

    pub fn run_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // for fps
        let mut last_time_ms = Game::cur_time_in_milliseconds().unwrap();

        // initialize 32 bit buffer as canvas
        let mut canvas: Canvas = Canvas::new(self.x_size, self.y_size);

        let mut frame: u128 = 0;
        while self.windows[0].is_open() && !self.windows[0].is_key_down(Key::Enter) {
            // game loop
            if let Some((mouse_x, mouse_y)) = self.windows[0].get_mouse_pos(MouseMode::Clamp) {
                canvas.fill(&WHITE);

                let mouse_pos: Vector2d = Vector2d {
                    x: (mouse_x as f64),
                    y: (mouse_y as f64),
                };

                // scroll stack down
                if self.players[self.players.len() - 1].get_origin().y < Y_LEVEL {
                    self.stack_root += 1.0;
                } else if self.players[self.players.len() - 1].get_origin().y > Y_LEVEL as f64 {
                    self.stack_root -= 1.0;
                }

                // manual stack increase for debug purposes
                if self.windows[0].is_key_pressed(Key::C, KeyRepeat::No) {
                    let new_stacked: Circle = Circle::new(RADIUS, &MAGENTA);
                    self.players.push(new_stacked);
                }

                // collision
                let mut falling_idxs_to_be_removed: Vec<usize> = Vec::new();
                let mut stack_idxs_to_be_removed: Vec<usize> = Vec::new();
                for (i, falling) in self.fallings.iter().enumerate() {
                    if Game::is_collision(&self.players[self.players.len() - 1], &falling) {
                        {
                            // collision detected, time to decide if good or bad
                            if falling.color.as_u32() == self.frames[0].color.as_u32() {
                                // good collision
                                falling_idxs_to_be_removed.push(i);
                                let new_stacked: Circle = falling.clone();
                                self.players.push(new_stacked);
                                self.frames[0]
                                    .set_color(&COLORS[self.rng.random_range(0..COLORS.len())]);
                            } else {
                                // bad collision
                                if self.players.len() > 2 {
                                    for i in self.players.len() / 2..self.players.len() {
                                        stack_idxs_to_be_removed.push(i);
                                    }
                                }
                            }
                        }
                    }
                }
                for i in falling_idxs_to_be_removed {
                    if i < self.fallings.len() {
                        self.fallings.remove(i);
                    }
                }
                for i in stack_idxs_to_be_removed {
                    if i < self.players.len() {
                        self.players.remove(i);
                    }
                }

                // simple cursor for player
                for (i, player) in self.players.iter_mut().enumerate() {
                    // stack offset
                    player.sprite.origin.y = self.stack_root - RADIUS * 2.0 * i as f64;

                    let mut delta_object_mouse =
                        (mouse_pos - player.sprite.origin) * Vector2d { x: (1.0), y: (0.0) };

                    // calc and apply inertia
                    let inertia: f64 = 1.0 / (i as f64 * 5.0);
                    delta_object_mouse.scale(inertia);

                    player.sprite.translate(delta_object_mouse);
                    canvas.draw_sprite(&player.sprite);
                }

                // spawn new falling
                if self.rng.random_bool(0.05) {
                    self.spawn_falling();
                }

                // apply gravity on falling
                let mut falling_idxs_to_be_removed: Vec<usize> = Vec::new();
                for (i, falling) in self.fallings.iter_mut().enumerate() {
                    falling.translate(self.gravity);
                    if falling.get_origin().y > SIZE_Y as f64 {
                        falling_idxs_to_be_removed.push(i);
                    }
                }
                for i in falling_idxs_to_be_removed {
                    if i < self.fallings.len() {
                        self.fallings.remove(i);
                    }
                }

                // render sprites
                for falling in self.fallings.iter() {
                    // canvas.draw_crosshair(&falling.sprite.origin);
                    canvas.draw_sprite(&falling.sprite);
                }

                for player in self.players.iter() {
                    // canvas.draw_crosshair(&player.sprite.origin);
                    canvas.draw_sprite(&player.sprite);
                }

                for frame in self.frames.iter() {
                    canvas.draw_sprite(&frame.sprite);
                }

                // update window with rendered framebuffer
                self.windows[0].update_with_buffer(
                    &canvas.buffer,
                    self.x_size as usize,
                    self.y_size as usize,
                )?;
            } else {
                println!("No mouse detected :(");
            };

            // sleep to stay within target fps
            // let cur_time_ms = Game::cur_time_in_milliseconds().unwrap() as u128;
            // let time_diff_ms = cur_time_ms - last_time_ms;
            // if self.target_interval_ms > time_diff_ms {
            //     thread::sleep(time::Duration::from_millis(
            //         self.target_interval_ms as u64 - time_diff_ms as u64,
            //     ));
            // }

            // calc and display fps
            let cur_time_ms = Game::cur_time_in_milliseconds().unwrap() as u128;
            let fps = 1000.0 / (cur_time_ms - last_time_ms as u128) as f64;
            print!("{}[2J", 27 as char);
            println!("FPS: {}", fps);
            println!("Frame: {}", frame);

            println!("Stack root: {}", self.stack_root);
            println!(
                "Stack top: {}",
                self.players[self.players.len() - 1].get_origin().y
            );
            println!("Stacksize: {}", self.players.len());
            println!("Fallings count: {}", self.fallings.len());

            frame += 1;
            last_time_ms = cur_time_ms;
            // uncomment for single frame exec
            // exit(0);
        }

        Ok(())
    }

    pub fn spawn_falling(&mut self) {
        let color = &COLORS[self.rng.random_range(0..COLORS.len())];
        let mut new_origin = Vector2d::new(self.rng.random_range(0.0..self.y_size as f64), -100.0);
        for i in 0..100 {
            let mut shortest_dist: f64 = 99999.9;
            for falling in self.fallings.iter() {
                let dist = (falling.sprite.origin - new_origin).length();
                if dist < shortest_dist {
                    shortest_dist = dist;
                }
            }
            if shortest_dist < RADIUS * 4.0 {
                new_origin = Vector2d::new(self.rng.random_range(0.0..self.y_size as f64), -100.0);
            }
        }

        let mut circle = Circle::new(RADIUS, color);
        circle.set_origin(new_origin);
        self.fallings.push(circle);
    }

    fn cur_time_in_milliseconds() -> Result<u128, SystemTimeError> {
        let current_system_time = SystemTime::now();
        let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH)?;
        let milliseconds_timestamp = duration_since_epoch.as_millis();

        Ok(milliseconds_timestamp)
    }

    pub fn is_collision(player: &Circle, falling: &Circle) -> bool {
        if player.sprite.distance_to_sprite(&falling.sprite) < RADIUS * 2.0 {
            if falling.get_origin().y < player.get_origin().y {
                println!("\n\n\n Collision detected!!!");
                println!(
                    "player: {}, {}",
                    player.sprite.origin.x, player.sprite.origin.y
                );
                println!(
                    "circle: {}, {}",
                    falling.sprite.origin.x, falling.sprite.origin.y
                );
                return true;
            }
        }
        return false;
    }
}
