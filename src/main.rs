use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::game::Game;
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod game;
pub mod sprites;
pub mod util;

const TARGET_FPS: u32 = 100;

const WOBBLE_FAC_1: f64 = 0.5;
const WOBBLE_FAC_2: f64 = 2.0;

const GRAVITY: Vector2d = Vector2d { x: (0.0), y: (0.5) };
const COLLISION_RADIUS: f64 = 1.0;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

const RADIUS: f64 = 50.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new(SIZE_X, SIZE_Y, TARGET_FPS);

    game.init();

    return game.run_game();
}
