use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

#[derive(Debug)]
pub struct Game {
    x_size: u32,
    y_size: u32,
    target_fps: u32,
    target_interval_ms: u128,

    circles: Vec<Circle>,
    players: Vec<Circle>,
    windows: Vec<Window>,
}

impl Game {
    pub fn new(x_size: u32, y_size: u32, target_fps: u32) -> Game {
        Game {
            x_size: x_size,
            y_size: y_size,
            target_fps: target_fps,
            target_interval_ms: (1000 / target_fps) as u128,
            circles: Vec::new(),
            players: Vec::new(),
            windows: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut circle = Circle::new(100.0, &MAGENTA);
        let mut player = Circle::new(100.0, &CYAN);
        self.players.push(player);

        let mut window = Window::new(
            "RRP (Rusty Ruling Pen)",
            self.x_size as usize,
            self.y_size as usize,
            WindowOptions::default(),
        )?;
        self.windows.push(window);

        circle.set_origin_xy(400.0, 200.0);
        self.circles.push(circle);
        Ok(())
    }

    pub fn run_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // for fps
        let mut last_time_ms = Game::cur_time_in_milliseconds().unwrap();

        // initialize 32 bit buffer as canvas
        let mut canvas: Canvas = Canvas::new(self.x_size, self.y_size);

        let mut frame: u128 = 0;
        while self.windows[0].is_open() && !self.windows[0].is_key_down(Key::Enter) {
            if let Some((mx, my)) = self.windows[0].get_mouse_pos(MouseMode::Clamp) {
                canvas.fill(&WHITE);

                // collision
                if self.players[0]
                    .sprite
                    .distance_to_sprite(&self.circles[0].sprite)
                    < 10.0
                {
                    exit(0)
                }

                // simple wasd movement for player
                if self.windows[0].is_key_down(Key::W) {
                    self.players[0].translate_xy(0.0, -10.0);
                }
                if self.windows[0].is_key_down(Key::A) {
                    self.players[0].translate_xy(-10.0, 0.0);
                }
                if self.windows[0].is_key_down(Key::S) {
                    self.players[0].translate_xy(0.0, 10.0);
                }
                if self.windows[0].is_key_down(Key::D) {
                    self.players[0].translate_xy(10.0, 0.0);
                }

                canvas.draw_sprite(&self.circles[0].sprite);
                canvas.draw_sprite(&self.players[0].sprite);
                let mouse_pos: Vector2d = Vector2d {
                    x: (mx as f64),
                    y: (my as f64),
                };

                canvas.draw_crosshair(self.circles[0].sprite.origin);
                canvas.draw_crosshair(self.players[0].sprite.origin);

                // render new framebuffer
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
            print!("{}[2J", 27 as char);

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

    fn cur_time_in_milliseconds() -> Result<u128, SystemTimeError> {
        let current_system_time = SystemTime::now();
        let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH)?;
        let milliseconds_timestamp = duration_since_epoch.as_millis();

        Ok(milliseconds_timestamp)
    }
}
