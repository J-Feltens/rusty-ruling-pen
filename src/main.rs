use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::{Rng, rng};
use std::process::exit;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{thread, time};

use crate::canvas::Canvas;
use crate::colors::{BLACK, CYAN, Color, MAGENTA, TRANSPARENT, WHITE, YELLOW};
use crate::dictionary::Dictionary;
use crate::game::Game;
use crate::sprites::{Circle, Sprite};
use crate::util::Vector2d;

pub mod canvas;
pub mod colors;
pub mod dictionary;
pub mod game;
pub mod sprites;
pub mod util;

const TARGET_FPS: u32 = 1000;

const SIZE_X: u32 = 1000;
const SIZE_Y: u32 = 600;

// the quick'n dirty
// fn main() {
// fn main() -> Result<(), Box<dyn std::error::Error>> {

pub struct Letter {
    letter: char,
    next: Vec<Letter>,
    is_word: bool,
}

impl Letter {
    pub fn new(letter: char) -> Letter {
        Letter {
            letter: letter.to_ascii_uppercase(),
            next: Vec::new(),
            is_word: false,
        }
    }

    pub fn has_next(&self, character: char) -> bool {
        for l in self.next.iter() {
            if l.letter.eq(&character.to_ascii_uppercase()) {
                return true;
            }
        }
        return false;
    }

    pub fn insert(&mut self, mut string: String) {
        // base case
        if string.len() <= 0 {
            self.is_word = true;
            return;
        }

        let cur_char: char = string.remove(0).to_ascii_uppercase();
        if self.has_next(cur_char) {
            // character already exists in self.next
            // println!(
            //     "Character '{}' already exists in '{}'",
            //     cur_char,
            //     self.letter
            // );
            for letter in self.next.iter_mut() {
                if letter.letter == cur_char {
                    // recursive call
                    letter.insert(string.clone());
                }
            }
        } else {
            // character does not exist yet in self.next, must create first
            // println!(
            //     "Character '{}' does not exist yet in '{}', adding now",
            //     cur_char, self.letter
            // );
            let new_letter: Letter = Letter::new(cur_char);
            self.next.push(new_letter);
            let new_letter_idx = self.next.len() - 1;

            // recursive call
            self.next[new_letter_idx].insert(string.clone());
        }
    }

    pub fn print_dict(&self) {
        if self.is_word {
            print!("{}\n", self.letter);
        }
        for i in 0..self.next.len() {
            print!("{}", self.letter);
            self.next[i].print_dict();
        }
    }
}

// fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
fn main() {
    // let mut game = Game::new(SIZE_X, SIZE_Y, TARGET_FPS);
    // game.init();
    // return game.run_game();

    let start = Game::cur_time_in_milliseconds().unwrap();

    let mut root: Letter = Letter::new(' ');
    let dictionary: Dictionary = Dictionary::new();
    for word in dictionary.dictionary.iter() {
        root.insert(word.to_string());
    }

    let end = Game::cur_time_in_milliseconds().unwrap();

    println!("Took {} ms", end - start);
    // root.print_dict();
}
