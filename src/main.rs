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
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Matrix3x3, Vector2d, Vector3d, Vector4d};

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

    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let green = Color::new(0.0, 1.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);
    let cyan = Color::new(1.0, 0.0, 1.0, 1.0);
    let yellow = Color::new(1.0, 1.0, 0.0, 1.0);
    let magenta = Color::new(0.0, 1.0, 1.0, 1.0);

    // cube
    let cube_origin = Vector3d::new(0.0, 0.0, 1.0);
    let cube_size = 1.0;
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

    // let tile_size = 0.2;
    // for y_ in -100..100 {
    //     for x_ in -100..100 {
    //         let (x, y) = (x_ as f64 * tile_size, y_ as f64 * tile_size);
    //         let c1 = Vector3d::new(x, y, 0.0);
    //         let c2 = Vector3d::new(x + tile_size, y, 0.0);
    //         let c3 = Vector3d::new(x + tile_size, y + tile_size, 0.0);
    //         let c4 = Vector3d::new(x, y + tile_size, 0.0);
    //         triangles.push(Triangle3d::new(c1, c2, c3, red));
    //         triangles.push(Triangle3d::new(c1, c3, c4, green));
    //     }
    // }

    // spherical coords for simple camera movement
    let gimbal_radius: f64 = 15.0;
    let angle_increment: f64 = 0.03;
    let mut camera_phi: f64 = 0.0;
    let mut camera_theta: f64 = PI / 2.0;
    let mut e = Vector3d::new(gimbal_radius, 0.0, 0.0) * 2.0; // cam pos

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
        let a = cube_origin.clone(); // look at
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

        // projection stuff
        let l = -2.0;
        let r = 2.0;
        let b = -2.0;
        let t = 2.0;
        let n = 1.0;
        let f = 10.0;

        let ortho_projection_matrix = Matrix4x4::from_vecs(
            Vector4d::new(2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)),
            Vector4d::new(0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)),
            Vector4d::new(0.0, 0.0, 1.0 / (n - f), -n / (f - n)),
            Vector4d::new(0.0, 0.0, 0.0, 1.0),
        );

        let perspective_projection_matrix = Matrix4x4::from_vecs(
            Vector4d::new((2.0 * n) / (r - l), 0.0, (l + r) / (r - l), 0.0),
            Vector4d::new(0.0, (2.0 * n) / (t - b), (b + t) / (t - b), 0.0),
            Vector4d::new(0.0, 0.0, -(n) / (f - n), -(f * n) / (f - n)),
            Vector4d::new(0.0, 0.0, -(n) / (f - n), -(f * n) / (f - n)),
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
