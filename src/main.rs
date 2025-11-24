use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::graphics::colors::RED;
use crate::graphics::scanline::draw_polygon_onto_buffer;
use crate::graphics::{CYAN, Canvas, WHITE};
use crate::graphics::{Color, calc_cube, calc_torus};
use crate::util::calc_perspective_matrix;
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Vector3d, Vector4d};
use std::f64::consts::PI;

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const SIZE_X_HALF: usize = SIZE_X / 2;
const SIZE_Y_HALF: usize = SIZE_Y / 2;
const SCALE: minifb::Scale = minifb::Scale::X1;

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

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, WHITE.clone());

    // cube
    // let cube = calc_cube(1.2, Vector3d::zero());
    // let cube2 = calc_cube(0.7, Vector3d::new(-0.5, 0.5, 0.5));

    // let mut triangles = cube.clone();
    // triangles.append(&mut (cube2.clone()));

    let triangles = calc_torus(2.0, 0.5, 16, 16, &CYAN);

    for triangle in triangles.iter() {
        println!("{}", triangle);
    }

    // spherical coords for simple camera movement
    let mut gimbal_radius: f64 = 30.0;
    let angle_increment: f64 = 0.05;
    let radius_increment: f64 = 0.3;
    let mut camera_phi: f64 = 0.0;
    let mut camera_theta: f64 = PI / 2.0;
    let mut e; // cam pos

    // projection stuff
    let l = -2.0;
    let r = 2.0;
    let b = -2.0;
    let t = 2.0;
    let n = 1.0;
    let f = 10.0;

    let perspective_projection_matrix = calc_perspective_matrix(l, r, b, t, n, f);

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
        e = Vector3d::new(
            gimbal_radius * camera_theta.sin() * camera_phi.cos(),
            gimbal_radius * camera_theta.sin() * camera_phi.sin(),
            gimbal_radius * camera_theta.cos(),
        );

        // camera space stuff
        // let mut e = Vector3d::new(5.0, 5.0, 1.0) * 2.0; // cam pos
        let a = Vector3d::zero(); // look at
        let t = Vector3d::new(0.0, 0.0, 1.0); // cam up
        let g = a - e;

        // camera space spanning vectors
        let w = g.normalize() * -1.0;
        let u = t.cross(w).normalize();
        let v = w.cross(u);

        let camera_matrix = Matrix4x4::from_vecs(
            Vector4d::from_vector3d(&u, -u.dot(e)),
            Vector4d::from_vector3d(&v, -v.dot(e)),
            Vector4d::from_vector3d(&w, -w.dot(e)),
            Vector4d::new(0.0, 0.0, 0.0, 1.0),
        );

        // finally, triangles
        for triangle in triangles.iter() {
            // backface culling
            if w.dot(triangle.normal) < 0.0 {
                continue;
            }

            let mut skip_triangle = false;
            let mut triangle_projected = vec![IntegerVector2d::zero(); 3];
            for (i, vertex) in triangle.vertices.iter().enumerate() {
                let mut vec4 = Vector4d::from_vector3d(vertex, 1.0);
                vec4 = camera_matrix.times_vec(vec4);
                vec4 = perspective_projection_matrix.times_vec(vec4);

                // perspective divide by z
                let vec3 = vec4.truncate_to_3d() / vec4.u;

                if vec3.x < -1.0 || vec3.x > 1.0 || vec3.y < -1.0 || vec3.y > 1.0 {
                    skip_triangle = true;
                }

                let mut attrs = triangle.color.as_f64_vec();
                attrs.push(vec4.z);

                let ivec2 = IntegerVector2d::new(
                    (vec3.x * SIZE_X_HALF as f64) as i32 + SIZE_X_HALF as i32,
                    (vec3.y * SIZE_Y_HALF as f64) as i32 + SIZE_Y_HALF as i32,
                    attrs,
                );
                triangle_projected[i] = ivec2;
            }

            //cull triangles that is even partially out if bounds
            if skip_triangle {
                continue;
            }
            draw_polygon_onto_buffer(&triangle_projected, &mut canvas, false);
        }

        // update minifb with new buffer
        window.update_with_buffer(&canvas.buffer, SIZE_X, SIZE_Y)?;

        // print statistics:
        let interval = global_timer.elapsed().as_millis();
        println!(
            "Rendertime: {} ms ({} fps)",
            interval,
            1.0 / (interval as f64 / 1000.0)
        );

        println!(
            "   {0: <20} {1: <4.3},   {2: <4.3},   {3: <4.3}",
            "Camera Position:", e.x, e.y, e.z
        );
        println!(
            "   {0: <20} {1: <4.3},   {2: <4.3},   {3: <4.3}",
            "u:", u.x, u.y, u.z
        );
        println!(
            "   {0: <20} {1: <4.3},   {2: <4.3},   {3: <4.3}",
            "v:", v.x, v.y, v.z
        );
        println!(
            "   {0: <20} {1: <4.3},   {2: <4.3},   {3: <4.3}",
            "w:", w.x, w.y, w.z
        );
        println!(
            "   {0: <20} {1: <4.3},   {2: <4.3},   {3: <4.3}",
            "g:", g.x, g.y, g.z
        );
        global_timer = Instant::now();
        // thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
