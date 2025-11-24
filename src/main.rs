use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::graphics::scanline::draw_polygon_onto_buffer;
use crate::graphics::{Canvas, PointLight};
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

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, Color::named_color("black"));

    // light
    let light = PointLight::new(
        Vector3d::new(1.0, 3.0, 5.0),
        1.0,
        Color::named_color("white"),
    );
    let light2 = PointLight::new(
        Vector3d::new(3.0, -8.0, -2.0),
        0.2,
        Color::named_color("white"),
    );
    canvas.add_point_light(light);
    canvas.add_point_light(light2);

    // cube
    let cube = calc_cube(2.0, Vector3d::zero());
    let cube2 = calc_cube(2.0, Vector3d::new(1.0, 1.0, 1.0));
    let torus = calc_torus(2.0, 1.0, 128 * 4, 64 * 4, &Color::named_color("cyan"));

    let mut triangles = vec![];

    triangles.append(&mut (torus.clone()));
    // triangles.append(&mut (cube.clone()));
    // triangles.append(&mut (cube2.clone()));

    // spherical coords for simple camera movement
    let mut gimbal_radius: f64 = 30.0;
    let angle_increment: f64 = 0.05;
    let radius_increment: f64 = 0.3;
    let mut camera_phi: f64 = 0.0;
    let mut camera_theta: f64 = 0.8;

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
        let e = Vector3d::new(
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

        // transform lights to camera space

        let mut lights_cam_space_reallight = canvas.lights.clone();
        for light in lights_cam_space_reallight.iter_mut() {
            light.pos = camera_matrix
                .times_vec(Vector4d::from_vector3d(&light.pos, 1.0))
                .truncate_to_3d();
        }

        // finally, triangles
        for triangle in triangles.iter() {
            // backface culling
            // Everlast - The Culling is Coming  =>   https://www.youtube.com/watch?v=yWYsbxkhlpU
            if w.dot(triangle.normal) < 0.0 {
                continue;
            }

            let mut skip_triangle = false;
            let mut triangle_projected = vec![IntegerVector2d::zero(); 3];
            for (i, vertex) in triangle.vertices.iter().enumerate() {
                let vertex_homo = Vector4d::from_vector3d(vertex, 1.0); // hehe

                // transform to camera space
                let vertex_cam_space = camera_matrix.times_vec(vertex_homo);

                // perspective projection
                let vertex_projected = perspective_projection_matrix.times_vec(vertex_cam_space);

                // perspective divide by z
                let vec3 = vertex_projected.truncate_to_3d() / vertex_projected.u;

                if vec3.x < -1.0 || vec3.x > 1.0 || vec3.y < -1.0 || vec3.y > 1.0 {
                    skip_triangle = true;
                }

                // store attributes like pos and normal while still in camera space
                let normal_cam_space =
                    camera_matrix.times_vec(Vector4d::from_vector3d(&triangle.normal, 0.0));
                let mut attrs: Vec<f64> = vec![0.0; 11];
                attrs[0] = vertex_cam_space.x;
                attrs[1] = vertex_cam_space.y;
                attrs[2] = vertex_cam_space.z;
                attrs[3] = vertex_projected.z;
                attrs[4] = normal_cam_space.x;
                attrs[5] = normal_cam_space.y;
                attrs[6] = normal_cam_space.z;
                attrs[7] = triangle.color.r;
                attrs[8] = triangle.color.g;
                attrs[9] = triangle.color.b;
                attrs[10] = triangle.color.a;

                let ivec2 = IntegerVector2d::new(
                    (vec3.x * SIZE_X_HALF as f64) as i32 + SIZE_X_HALF as i32,
                    (vec3.y * SIZE_Y_HALF as f64) as i32 + SIZE_Y_HALF as i32,
                    attrs,
                );
                triangle_projected[i] = ivec2;
            }

            // cull triangles that is even partially out if bounds
            if skip_triangle {
                continue;
            }
            draw_polygon_onto_buffer(
                &triangle_projected,
                &mut canvas,
                &lights_cam_space_reallight,
            );
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
        global_timer = Instant::now();
        // thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
