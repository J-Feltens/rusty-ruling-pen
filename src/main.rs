use std::arch::global_asm;
use std::time::Instant;

use image::imageops::FilterType::Triangle;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use std::{thread, time};

use crate::graphics::colors::rgb2u32;
use crate::graphics::scanline::{
    ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTableEntry, draw_polygon_onto_buffer,
};
use crate::graphics::{
    BLACK, BLUE, CYAN, Canvas, Color, EdgeTable, GREEN, MAGENTA, RED, Triangle3d, WHITE, YELLOW,
    triangles,
};
use crate::util::interpolate1d;
use crate::vectors::ivec2d::Polygon2d;
use crate::vectors::{IntegerVector2d, Matrix3d, Vector2d, Vector3d};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const SCALE: minifb::Scale = minifb::Scale::X1;
const ANIM_INTERVAL: time::Duration = time::Duration::from_millis(0);

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut global_timer = Instant::now();

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
            topmost: true,
            transparency: false,
            none: false,
        },
    )?;

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, &WHITE);

    canvas.checker(
        &WHITE,
        &Color {
            r: (0.0),
            g: (0.0),
            b: (0.0),
            a: (0.1),
        },
    );

    let red = Color::new(1.0, 0.0, 0.0, 0.3);
    let green = Color::new(0.0, 1.0, 0.0, 0.3);
    let blue = Color::new(0.0, 0.0, 1.0, 0.3);
    let cyan = Color::new(1.0, 0.0, 1.0, 0.3);
    let yellow = Color::new(1.0, 1.0, 0.0, 0.3);
    let magenta = Color::new(0.0, 1.0, 1.0, 0.3);

    // cube
    let cube_origin = Vector3d::new(-10.0, -10.0, 0.0);
    // vertices
    let v1 = Vector3d::new(0.0, 0.0, 0.0);
    let v2 = Vector3d::new(20.0, 0.0, 0.0);
    let v3 = Vector3d::new(20.0, 20.0, 0.0);
    let v4 = Vector3d::new(0.0, 20.0, 0.0);

    let v5 = Vector3d::new(0.0, 0.0, 20.0);
    let v6 = Vector3d::new(20.0, 0.0, 20.0);
    let v7 = Vector3d::new(20.0, 20.0, 20.0);
    let v8 = Vector3d::new(0.0, 20.0, 20.0);

    // faces
    let mut triangles = vec![
        // floor
        Triangle3d::new(v1, v2, v3, red),
        Triangle3d::new(v1, v3, v4, red),
        // lid
        Triangle3d::new(v5, v6, v7, blue),
        Triangle3d::new(v5, v7, v8, blue),
        // sides
        Triangle3d::new(v1, v2, v6, green),
        Triangle3d::new(v1, v5, v6, green),
        Triangle3d::new(v2, v3, v7, cyan),
        Triangle3d::new(v2, v6, v7, cyan),
        Triangle3d::new(v3, v4, v8, yellow),
        Triangle3d::new(v3, v7, v8, yellow),
        Triangle3d::new(v4, v1, v5, magenta),
        Triangle3d::new(v4, v8, v5, magenta),
    ];

    for triangle in triangles.iter_mut() {
        triangle.p1 += cube_origin;
        triangle.p2 += cube_origin;
        triangle.p3 += cube_origin;
    }

    fn to_screen_ortho(p_cam: Vector3d, zoom_fac: f64) -> (i32, i32) {
        let sx = (zoom_fac * p_cam.x + (SIZE_X as f64) / 2.0).round() as i32;
        let sy = ((SIZE_Y as f64) / 2.0 - zoom_fac * p_cam.y).round() as i32;
        return (sx, sy);
    }

    let mut angle = 0.0 as f64;
    let angle_increment = 0.01 as f64;
    let radius = 100.0;

    while window.is_open() && !window.is_key_down(Key::Enter) {
        // render loop
        canvas.checker(&WHITE, &WHITE);

        angle += angle_increment;
        let e_x = angle.cos() * radius;
        let e_y = angle.sin() * radius;
        let e_z = 20.0;

        let e = Vector3d::new(e_x, e_y, e_z); // camera pos
        let a = Vector3d::new(0.0, 0.0, 0.0); // look at
        let t = Vector3d::new(0.0, 0.0, 1.0); // camera up

        let g = a - e;
        let w = (g * -1.0) / g.length();
        let u = t.cross(w) / (t.cross(w)).length();
        let v = w.cross(u);

        let camera_space_matrix = Matrix3d::from_vecs(u, v, w);

        // finally, triangles
        for triangle in triangles.iter() {
            let p1_cam = camera_space_matrix.times_vec(triangle.p1 - e);
            let p2_cam = camera_space_matrix.times_vec(triangle.p2 - e);
            let p3_cam = camera_space_matrix.times_vec(triangle.p3 - e);

            let (x1, y1) = to_screen_ortho(p1_cam, 5.0);
            let (x2, y2) = to_screen_ortho(p2_cam, 5.0);
            let (x3, y3) = to_screen_ortho(p3_cam, 5.0);

            draw_polygon_onto_buffer(
                &vec![
                    IntegerVector2d::new(x1, y1, triangle.color.as_f64_vec()),
                    IntegerVector2d::new(x2, y2, triangle.color.as_f64_vec()),
                    IntegerVector2d::new(x3, y3, triangle.color.as_f64_vec()),
                ],
                &mut canvas,
                false,
            );
        }

        // update minifb with new buffer
        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;

        println!("Rendertime: {} ms", global_timer.elapsed().as_millis());
        global_timer = Instant::now();
        thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
