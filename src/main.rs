use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use std::{thread, time};

use crate::graphics::Cube;
use crate::graphics::scanline::draw_polygon_onto_buffer;
use crate::graphics::{Canvas, WHITE};
use crate::util::calc_perspective_matrix;
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Vector3d, Vector4d};

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const SIZE_X_HALF: usize = SIZE_X / 2;
const SIZE_Y_HALF: usize = SIZE_Y / 2;
const PI: f64 = 3.141;

const SCALE: minifb::Scale = minifb::Scale::X1;
const ANIM_INTERVAL: time::Duration = time::Duration::from_millis(0);

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

    // cube s

    let cube = Cube::new(1.2, Vector3d::zero());
    let cube2 = Cube::new(0.7, Vector3d::new(-0.5, 0.3, 0.3));

    let mut triangles = cube.triangles.clone();
    triangles.append(&mut (cube2.triangles.clone()));

    // spherical coords for simple camera movement
    let gimbal_radius: f64 = 15.0;
    let angle_increment: f64 = 0.03;
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

        let u_extended = Vector4d::from_vector3d(&u, -u.dot(e));
        let v_extended = Vector4d::from_vector3d(&v, -v.dot(e));
        let w_extended = Vector4d::from_vector3d(&w, -w.dot(e));
        let camera_matrix = Matrix4x4::from_vecs(
            u_extended,
            v_extended,
            w_extended,
            Vector4d::new(0.0, 0.0, 0.0, 1.0),
        );

        // finally, triangles
        for triangle in triangles.iter() {
            let mut skip_triangle = false;
            let mut triangle_projected = vec![IntegerVector2d::zero(); 3];
            for (i, vertex) in vec![triangle.p1, triangle.p2, triangle.p3]
                .iter()
                .enumerate()
            {
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
        window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;

        // print statistics:
        let T = global_timer.elapsed().as_millis();
        println!("Rendertime: {} ms ({} fps)", T, 1.0 / (T as f64 / 1000.0));

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
        thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
