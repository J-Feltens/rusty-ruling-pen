use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::dictionary::Dictionary;
use crate::game::Game;
use crate::letters::Letter;
use crate::sprites::{Circle, LetterCircle, Sprite, sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod dictionary;
pub mod game;
pub mod letters;
pub mod sprites;
pub mod util;

const TARGET_FPS: u32 = 1000;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

// the quick'n dirty
// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    // fn main() {
    // let mut game = Game::new(SIZE_X, SIZE_Y, TARGET_FPS);
    // game.init();
    // return game.run_game();

    // let mut root: Letter = Letter::new(' ');
    // let dictionary: Dictionary = Dictionary::new();
    // for word in dictionary.dictionary.iter() {
    //     root.insert(word.to_string());
    // }
    // println!("{}", root.is_partial_word("aced".to_string()));

    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        SIZE_X as usize,
        SIZE_Y as usize,
        WindowOptions::default(),
    )?;
    let mut canvas: Canvas = Canvas::new(SIZE_X, SIZE_Y);

    let letter: LetterCircle = LetterCircle::new('p', 30.0, BLACK);

    canvas.draw_sprite(&letter.circle.sprite);

    while window.is_open() && !window.is_key_down(Key::Enter) {
        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;
    }
    Ok(())
}
