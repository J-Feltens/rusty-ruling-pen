use std::time::Instant;

use minifb::{CursorStyle, Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};

use crate::graphics::colors::named_color;
use crate::graphics::{Camera, Canvas, PointLight, SSAA, Scene, calc_sphere, calc_teapot};
use crate::graphics::{calc_cube, calc_torus};
use crate::util::{calc_perspective_matrix, clear_console};
use crate::vectors::{Matrix3x3, Vector3d, Vector4d};
use std::f64::consts::PI;
use std::{thread, time};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 800;
const SCALE: minifb::Scale = minifb::Scale::X1;
const SSAA: SSAA = SSAA::X4;
const SHAPE_RESOLUTION: usize = 64;
const RENDER_SMOOTH: bool = true;
const TARGET_FPS: usize = 60;
const TARGET_INTERVAL_MILLIS: f64 = 1000.0 / TARGET_FPS as f64;

// fn main() {
//     let m1 = Matrix4x4::test();
//     let m2 = m1.transpose();

//     println!("{}", m1);
//     println!("{}", m2);
// }

/*
   Wowww, I really fucked up the git timeline. Let this be a warning to future me to lay off the weed.
*/

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

    let e = Vector3d::new(30.0, 30.0, 30.0);
    let a = Vector3d::zero(); // look at
    let t = Vector3d::new(0.0, 0.0, 1.0); // cam up

    let camera = Camera::new(e, a, t, -2.0, 2.0, -2.0, 2.0, 1.0, 10.0);
    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, named_color("black"), SSAA);

    let mut scene = Scene::new(canvas, camera, RENDER_SMOOTH);

    // light
    scene.add_light(PointLight::new(
        Vector3d::new(5.0, 5.0, 0.0),
        1.0,
        Vector4d::new(1.0, 0.8, 0.3, 1.0),
    ));

    scene.add_light(PointLight::new(
        Vector3d::new(-7.0, 5.0, 3.0),
        1.0,
        named_color("cyan"),
    ));

    // cube
    let cube = calc_cube(2.0, Vector3d::zero(), named_color("white"));

    let cube2 = calc_cube(2.0, Vector3d::new(1.0, 1.0, 1.0), named_color("white"));
    let torus = calc_torus(
        Vector3d::zero(),
        2.7,
        1.2,
        SHAPE_RESOLUTION * 2,
        SHAPE_RESOLUTION,
        &named_color("white"),
    );
    let sphere = calc_sphere(
        Vector3d::zero(),
        1.3,
        SHAPE_RESOLUTION,
        &named_color("white"),
    );
    let teapot = calc_teapot(named_color("white"), 1, true);

    // canvas.add_mesh(torus);
    // canvas.add_mesh(sphere);
    // canvas.add_mesh(cube);
    scene.add_mesh(teapot);

    let mut prev_mouse_pos = (0.0 as f32, 0.0 as f32);
    while window.is_open() && !window.is_key_down(Key::Enter) && !window.is_key_down(Key::Space) {
        global_timer = Instant::now();

        let (g, u, v, w) = scene.camera.calc_guvw();

        // handle keyboard and mouse

        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            scene.canvas.decrease_ssaa();
        }
        if window.is_key_pressed(Key::T, KeyRepeat::No) {
            scene.canvas.increase_ssaa();
        }
        // handle mouse input
        let cur_mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap();
        let mouse_delta = (
            cur_mouse_pos.0 - prev_mouse_pos.0,
            cur_mouse_pos.1 - prev_mouse_pos.1,
        );
        prev_mouse_pos = cur_mouse_pos;

        if window.get_mouse_down(MouseButton::Left) {
            if window.is_key_pressed(Key::LeftCtrl, KeyRepeat::Yes) {
                // rotate mode
                let cam_pos_local = scene.camera.e - scene.camera.a;
                let rot_mat = Matrix3x3::calc_rotation_matrix(
                    Vector3d::new(1.0, 0.0, 0.0),
                    0.01 * mouse_delta.0 as f64,
                );
                scene.camera.e = (rot_mat.times_vec(cam_pos_local)) + scene.camera.a;
            } else if window.is_key_pressed(Key::LeftAlt, KeyRepeat::Yes) {
                // pan mode
                // pan mode
                scene.camera.e -= u * 0.03 * mouse_delta.0 as f64;
                scene.camera.a -= u * 0.03 * mouse_delta.0 as f64;
                scene.camera.e += v * 0.03 * mouse_delta.1 as f64;
                scene.camera.a += v * 0.03 * mouse_delta.1 as f64;
            }
        }

        // render loop
        scene.canvas.reset();
        scene.canvas.reset_z_buffer();

        // finally, render scene
        scene.render_scene_to_buffer();

        // update minifb with new buffer
        window.update_with_buffer(
            &scene.canvas.buffer,
            scene.canvas.size_x,
            scene.canvas.size_y,
        )?;

        // compute sleep duration to reach target fps
        let render_time_millis = global_timer.elapsed().as_millis();
        let delta_to_target_interval =
            (TARGET_INTERVAL_MILLIS - render_time_millis as f64).max(0.0);
        let interval = (render_time_millis as f64 + delta_to_target_interval) / 1000.0;

        // print statistics:
        clear_console();

        println!("{} FPS", 1.0 / interval);
        println!("Rendertime: {} ms", interval,);
        println!("Render config:");
        println!(
            "  Image size: \n       {}x{} pixels, {} pixels in total",
            scene.canvas.size_x,
            scene.canvas.size_y,
            scene.canvas.buffer.len()
        );
        println!("  Antialiasing: \n        {}", scene.canvas.ssaa);
        println!(
            "       {}x{} pixels, {} pixels in total",
            scene.canvas.size_x_supersized,
            scene.canvas.size_y_supersized,
            scene.canvas.buffer_supersized.len()
        );
        println!(
            "Camera: \n    eye: {}\n    u: {}\n    v: {}\n    w: {}",
            scene.camera.e, u, v, w
        );
        println!("Mouse delta:\n     {}, {}", mouse_delta.0, mouse_delta.0);
        thread::sleep(time::Duration::from_millis(delta_to_target_interval as u64));
    }

    Ok(())
}
