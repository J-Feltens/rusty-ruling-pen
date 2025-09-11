use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

const RADIUS: f64 = 50.0;

#[derive(Debug)]
pub struct Game {
    x_size: u32,
    y_size: u32,
    target_fps: u32,
    target_interval_ms: u128,

    gravity: Vector2d,

    fallings: Vec<Circle>,
    players: Vec<Circle>,
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
            rng: rand::rng(),
            gravity: Vector2d { x: 0.0, y: 2.0 },
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut player = Circle::new(RADIUS, &CYAN);
        self.players.push(player);

        let mut window = Window::new(
            "RRP (Rusty Ruling Pen)",
            self.x_size as usize,
            self.y_size as usize,
            WindowOptions::default(),
        )?;
        self.windows.push(window);
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

                // collision
                let mut falling_idxs_to_be_removed: Vec<usize> = Vec::new();
                for (i, falling) in self.fallings.iter().enumerate() {
                    if Game::is_collision(&self.players[self.players.len() - 1], &falling) {
                        falling_idxs_to_be_removed.push(i);
                        let new_stacked: Circle = falling.clone();
                        self.players.push(new_stacked);
                    }
                }
                for i in falling_idxs_to_be_removed {
                    self.fallings.remove(i);
                }

                // simple wasd movement for player
                // if self.windows[0].is_key_down(Key::W) {
                //     self.players[0].translate_xy(0.0, -10.0);
                // }
                // if self.windows[0].is_key_down(Key::A) {
                //     self.players[0].translate_xy(-10.0, 0.0);
                // }
                // if self.windows[0].is_key_down(Key::S) {
                //     self.players[0].translate_xy(0.0, 10.0);
                // }
                // if self.windows[0].is_key_down(Key::D) {
                //     self.players[0].translate_xy(10.0, 0.0);
                // }

                // simple cursor for player
                let stack_root = self.players[0].get_origin();
                for (i, player) in self.players.iter_mut().enumerate() {
                    if i == 0 {
                        player.set_origin(mouse_pos);
                    } else {
                        // stack offset
                        player.set_origin(stack_root);
                        player.translate_xy(0.0, -RADIUS * 2.0 * i as f64);
                        let d_mouse: Vector2d =
                            (mouse_pos - player.get_center()) / (2.0 + (200.0 * i as f64));
                    }
                    println!(
                        "Player {} pos: {}, {}",
                        i,
                        player.get_origin().x,
                        player.get_origin().y
                    );
                }

                // spawn new falling
                if self.rng.random_bool(0.01) {
                    self.spawn_falling();
                }

                // apply gravity on falling
                for falling in self.fallings.iter_mut() {
                    falling.translate(self.gravity);
                }

                // render sprites
                for falling in self.fallings.iter_mut() {
                    // canvas.draw_crosshair(&falling.sprite.origin);
                    canvas.draw_sprite(&falling.sprite);
                }

                for player in self.players.iter_mut() {
                    // canvas.draw_crosshair(&player.sprite.origin);
                    canvas.draw_sprite(&player.sprite);
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
            let cur_time_ms = Game::cur_time_in_milliseconds().unwrap() as u128;
            let time_diff_ms = cur_time_ms - last_time_ms;
            if self.target_interval_ms > time_diff_ms {
                thread::sleep(time::Duration::from_millis(
                    self.target_interval_ms as u64 - time_diff_ms as u64,
                ));
            }
            // print!("{}[2J", 27 as char);

            // calc and display fps
            let cur_time_ms = Game::cur_time_in_milliseconds().unwrap() as u128;
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

    pub fn spawn_falling(&mut self) {
        let mut circle = Circle::new(RADIUS, &MAGENTA);
        circle.set_origin_xy(self.rng.random_range(0.0..self.y_size as f64), -100.0);
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
        return false;
    }
}
