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

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, WHITE.clone());

    let red = Color::new(1.0, 0.0, 0.0, 0.6);
    let green = Color::new(0.0, 1.0, 0.0, 0.6);
    let blue = Color::new(0.0, 0.0, 1.0, 0.6);
    let cyan = Color::new(1.0, 0.0, 1.0, 0.6);
    let yellow = Color::new(1.0, 1.0, 0.0, 0.6);
    let magenta = Color::new(0.0, 1.0, 1.0, 0.6);

    // cube
    let cube_origin = Vector3d::new(-10.0, -10.0, 0.0);
    let cube_size = 100.0;
    // vertices
    let v1 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
    let v2 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
    let v3 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);
    let v4 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);

    let v5 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
    let v6 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
    let v7 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);
    let v8 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);

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

    fn to_screen_perspective(p_cam: Vector3d, focal_length: f64) -> (i32, i32) {
        // Cull points behind camera or on the plane
        if p_cam.z >= 0.0 {
            println!("Culling!!");
            return (0, 0);
        }

        let x_ndc = -focal_length * p_cam.x / p_cam.z;
        let y_ndc = -focal_length * p_cam.y / p_cam.z;

        let sx = (x_ndc + (SIZE_X as f64) / 2.0).round() as i32;
        let sy = ((SIZE_Y as f64) / 2.0 - y_ndc).round() as i32;

        return (sx, sy);
    }

    let mut cam_pos = Vector3d::new(150.0, 130.0, 20.0);
    let mut cam_look_at = Vector3d::new(0.0, 0.0, 0.0);
    let z_up = Vector3d::new(0.0, 0.0, 1.0); // camera up
    let mut fov = 120.0;

    while window.is_open() && !window.is_key_down(Key::Enter) && !window.is_key_down(Key::Q) {
        // render loop
        canvas.reset();
        canvas.checker(
            &WHITE,
            &Color {
                r: (0.0),
                g: (0.0),
                b: (0.0),
                a: (0.1),
            },
        );

        // calculate movement direction
        let left = z_up.cross(cam_look_at).normalize();
        let right = left * -1.0;

        // get active keys
        let keys_down = window.get_keys();
        if keys_down.contains(&Key::Right) {
            let mut look_at_xy = Vector2d::new(cam_look_at.x, cam_look_at.y);
            let cam_pos_xy = Vector2d::new(cam_pos.x, cam_pos.y);
            look_at_xy.rotate_around_point(-0.02, cam_pos_xy);
            cam_look_at.x = look_at_xy.x;
            cam_look_at.y = look_at_xy.y;
        }
        if keys_down.contains(&Key::Left) {
            let mut look_at_xy = Vector2d::new(cam_look_at.x, cam_look_at.y);
            let cam_pos_xy = Vector2d::new(cam_pos.x, cam_pos.y);
            look_at_xy.rotate_around_point(0.02, cam_pos_xy);
            cam_look_at.x = look_at_xy.x;
            cam_look_at.y = look_at_xy.y;
        }

        // let e = Vector3d::new(e_x, e_y, e_z); // camera pos
        // let a = Vector3d::new(0.0, 0.0, 0.0); // look at
        let e = cam_pos;
        let a = cam_look_at;
        let t = z_up;

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

            let (x1, y1) = to_screen_perspective(p1_cam, fov);
            let (x2, y2) = to_screen_perspective(p2_cam, fov);
            let (x3, y3) = to_screen_perspective(p3_cam, fov);

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
