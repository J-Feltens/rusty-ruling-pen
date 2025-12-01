use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::graphics::colors::named_color;
use crate::graphics::{Canvas, PointLight, SSAA, calc_sphere, calc_teapot};
use crate::graphics::{calc_cube, calc_torus};
use crate::util::{calc_perspective_matrix, clear_console};
use crate::vectors::{Vector3d, Vector4d};
use std::f64::consts::PI;

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 800;
const SCALE: minifb::Scale = minifb::Scale::X1;
const SSAA: SSAA = SSAA::X0_125;
const SHAPE_RESOLUTION: usize = 64;
const RENDER_SMOOTH: bool = true;

// fn main() {
//     let m1 = Matrix4x4::test();
//     let m2 = m1.transpose();

//     println!("{}", m1);
//     println!("{}", m2);
// }

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

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, named_color("black"), SSAA, RENDER_SMOOTH);

    // light
    canvas.add_point_light(PointLight::new(
        Vector3d::new(5.0, 5.0, 0.0),
        1.0,
        Vector4d::new(1.0, 0.8, 0.3, 1.0),
    ));

    canvas.add_point_light(PointLight::new(
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
    let teapot = calc_teapot(named_color("white"), 1);

    // canvas.add_mesh(torus);
    // canvas.add_mesh(sphere);
    // canvas.add_mesh(cube);
    canvas.add_mesh(teapot);

    // spherical coords for simple camera movement
    let mut gimbal_radius: f64 = 30.0;
    let angle_increment: f64 = PI / 128.0;
    let radius_increment: f64 = 0.3;
    let mut camera_phi: f64 = 3.0 * 2.0 * PI / 32.0;
    let mut camera_theta: f64 = 0.7;

    // projection stuff
    let l = -2.0;
    let r = 2.0;
    let b = -2.0;
    let t = 2.0;
    let n = 1.0;
    let f = 10.0;

    let perspective_projection_matrix = calc_perspective_matrix(l, r, b, t, n, f);
    canvas.set_perspective_matrix(perspective_projection_matrix);

    while window.is_open() && !window.is_key_down(Key::Enter) && !window.is_key_down(Key::Space) {
        // render loop
        canvas.reset();
        canvas.reset_z_buffer();

        // get active keys
        let keys_down = window.get_keys();

        // wasd camera gimbal-like movement using spherical coords
        let mut increment_phi = 0.0;
        let mut increment_theta = 0.0;
        if keys_down.contains(&Key::W) {
            increment_theta -= angle_increment;
        }
        if keys_down.contains(&Key::A) {
            increment_phi -= angle_increment;
        }
        if keys_down.contains(&Key::S) {
            increment_theta += angle_increment;
        }
        if keys_down.contains(&Key::D) {
            increment_phi += angle_increment;
        }
        if keys_down.contains(&Key::E) {
            gimbal_radius += radius_increment;
        }
        if keys_down.contains(&Key::Q) {
            gimbal_radius -= radius_increment;
        }
        // increment camera angles
        camera_theta += increment_theta;
        // clamp phi
        camera_phi += increment_phi;
        if camera_theta > PI {
            camera_theta = PI;
        } else if camera_theta <= 0.0 {
            camera_theta = 0.0000001;
        }
        // set camera pos (eye)
        let e = Vector3d::new(
            gimbal_radius * camera_theta.sin() * camera_phi.cos(),
            gimbal_radius * camera_theta.sin() * camera_phi.sin(),
            gimbal_radius * camera_theta.cos(),
        );

        // finally, triangles
        canvas.render_scene_to_buffer(e);

        // update minifb with new buffer
        window.update_with_buffer(&canvas.buffer, canvas.size_x, canvas.size_y)?;

        // print statistics:
        let interval = global_timer.elapsed().as_millis();
        clear_console();
        println!("{} FPS", 1.0 / (interval as f64 / 1000.0));
        println!("Rendertime: {} ms", interval,);
        println!("Render config:");
        println!(
            "  Image size: \n       {}x{} pixels, {} pixels in total",
            canvas.size_x,
            canvas.size_y,
            canvas.buffer.len()
        );
        println!("  Antialiasing: \n        {}", canvas.ssaa);
        global_timer = Instant::now();
        println!(
            "       {}x{} pixels, {} pixels in total",
            canvas.size_x_supersized,
            canvas.size_y_supersized,
            canvas.buffer_supersized.len()
        );
        // thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
