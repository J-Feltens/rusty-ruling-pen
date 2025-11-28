use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::graphics::colors::named_color;
use crate::graphics::{Canvas, PointLight, SSAA, calc_sphere};
use crate::graphics::{calc_cube, calc_torus};
use crate::util::calc_perspective_matrix;
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Vector3d, Vector4d};
use std::f64::consts::PI;

pub mod graphics;
pub mod util;
pub mod vectors;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 800;
const SCALE: minifb::Scale = minifb::Scale::X1;
const SSAA: SSAA = SSAA::X16;
const SHAPE_RESOLUTION: usize = 128;

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

    let mut canvas = Canvas::new(SIZE_X, SIZE_Y, named_color("black"), SSAA);

    // light
    let (deg_0, deg_60, deg_120, deg_180, deg_240, deg_300) = (
        0.0f64,
        PI * 1.0 / 3.0,
        PI * 2.0 / 3.0,
        PI * 3.0 / 3.0,
        PI * 4.0 / 3.0,
        PI * 5.0 / 3.0,
    );

    let light_array_radius = 6.0;
    let light_array_z = 3.0;
    let light_array_strength = 1.0;
    let light_colors = vec!["red", "yellow", "green", "cyan", "blue", "magenta"];
    let light_colors = vec!["red", "yellow", "green", "cyan", "blue", "magenta"];
    for i in 0..6 {
        canvas.add_point_light(PointLight::new(
            Vector3d::new(
                (PI * i as f64 / 3.0).cos() * light_array_radius,
                (PI * i as f64 / 3.0).sin() * light_array_radius,
                if i % 2 == 0 { 1.0 } else { -1.0 } * light_array_z,
            ),
            light_array_strength,
            named_color(light_colors[i]),
        ));
    }

    let mut triangles = vec![];
    // cube
    let cube = calc_cube(2.0, Vector3d::zero());
    let cube2 = calc_cube(2.0, Vector3d::new(1.0, 1.0, 1.0));
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
        1.2,
        SHAPE_RESOLUTION,
        &named_color("white"),
    );

    // triangles.append(&mut (cube.clone()));
    // triangles.append(&mut (cube2.clone()));
    triangles.append(&mut (torus.clone()));
    triangles.append(&mut (sphere.clone()));

    // spherical coords for simple camera movement
    let mut gimbal_radius: f64 = 30.0;
    let angle_increment: f64 = PI / 16.0;
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
                attrs[7] = triangle.color.x;
                attrs[8] = triangle.color.y;
                attrs[9] = triangle.color.z;
                attrs[10] = triangle.color.u;

                let ivec2 = IntegerVector2d::new(
                    (vec3.x * canvas.size_x_supersized_half as f64) as i32
                        + canvas.size_x_supersized_half as i32,
                    (vec3.y * canvas.size_y_supersized_half as f64) as i32
                        + canvas.size_y_supersized_half as i32,
                    attrs,
                );
                triangle_projected[i] = ivec2;
            }

            // cull triangles that is even partially out if bounds
            if skip_triangle {
                continue;
            }
            canvas.draw_polygon_onto_buffer(&triangle_projected, &lights_cam_space_reallight);
        }

        canvas.apply_ssaa();

        // update minifb with new buffer
        window.update_with_buffer(&canvas.buffer, canvas.size_x, canvas.size_y)?;

        // print statistics:
        let interval = global_timer.elapsed().as_millis();
        println!("{} FPS", 1.0 / (interval as f64 / 1000.0));
        println!("Rendertime: {} ms", interval,);
        println!("Render config:");
        println!(
            "  Image size: {}x{} pixels, {} pixels in total",
            canvas.size_x,
            canvas.size_y,
            canvas.buffer.len()
        );
        println!("  Antialiasing: {}", canvas.ssaa);
        global_timer = Instant::now();
        // thread::sleep(ANIM_INTERVAL);
    }

    Ok(())
}
