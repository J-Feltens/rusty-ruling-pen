use minifb::{Key, Window, WindowOptions};
use rand::Rng;

use crate::Vector::Vector2d;

mod Canvas;
mod Colors;
mod Vector;

const PI: f64 = 3.14159265359;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut rng = rand::rng();

    let white: Colors::Color = Colors::Color::from_rgb(255, 255, 255);
    let black: Colors::Color = Colors::Color::from_rgb(0, 0, 0);

    // initialize 32 bit buffer as canvas
    let mut buffer: Vec<u32> = vec![white.c; Canvas::WIDTH * Canvas::HEIGHT];
    for y in 0..Canvas::HEIGHT {
        for x in 0..Canvas::WIDTH {
            buffer[y * Canvas::WIDTH + x] = white.c;
        }
    }

    // draw_circle(&mut buffer, 100, 50, 30, Color::from_rgb(0, 0, 0).c);
    let v1 = Vector::Vector2d::new(0.0, 0.0);
    let v2 = Vector::Vector2d::new(0.0, 100.0);

    let mut l = Vector::Line::new(v1, v2);

    l.v1.add(&Vector2d {
        x: (200.0),
        y: (200.0),
    });
    l.v2.add(&Vector2d {
        x: (200.0),
        y: (200.0),
    });

    // l.draw(&mut buffer, 128, black, 10);

    // create a window and show the buffer
    let mut window = Window::new(
        "RRP (Rusty Ruling Pen)",
        Canvas::WIDTH,
        Canvas::HEIGHT,
        WindowOptions::default(),
    )?;

    // main loop
    while window.is_open() && !window.is_key_down(Key::Enter) {
        let mut new_buffer = buffer.clone();

        l.v1.rotate(0.1);

        l.draw(&mut new_buffer, 16, Colors::Color::from_rgb(0, 0, 0), 10);

        window.update_with_buffer(&new_buffer, Canvas::WIDTH, Canvas::HEIGHT)?;
    }

    Ok(())
}
