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

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

// the quick'n dirty
// fn main() {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new(SIZE_X, SIZE_Y, TARGET_FPS);

    game.init();

    return game.run_game();
}
