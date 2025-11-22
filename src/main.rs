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
const SCALE: minifb::Scale = minifb::Scale::X1;
const ANIM_INTERVAL: time::Duration = time::Duration::from_millis(0);

fn main() {
    let m1 = Matrix3x3::test();
    let v1 = Vector3d::test();
    let v2 = m1.times_vec(v1);

    println!("v1:\n{}", v1);
    println!("m1:\n{}", m1);
    println!("v2:\n{}", v2);
}

// fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
//     let mut global_timer = Instant::now();

//     let mut window = Window::new(
//         "RRP (Rusty Ruling Pen)",
//         SIZE_X,
//         SIZE_Y,
//         WindowOptions {
//             borderless: false,
//             title: true,
//             scale: SCALE,
//             resize: false,
//             scale_mode: minifb::ScaleMode::UpperLeft,
//             topmost: true,
//             transparency: false,
//             none: false,
//         },
//     )?;

//     let mut canvas = Canvas::new(SIZE_X, SIZE_Y, WHITE.clone());

//     let red = Color::new(1.0, 0.0, 0.0, 0.6);
//     let green = Color::new(0.0, 1.0, 0.0, 0.6);
//     let blue = Color::new(0.0, 0.0, 1.0, 0.6);
//     let cyan = Color::new(1.0, 0.0, 1.0, 0.6);
//     let yellow = Color::new(1.0, 1.0, 0.0, 0.6);
//     let magenta = Color::new(0.0, 1.0, 1.0, 0.6);

//     // cube
//     let cube_origin = Vector3d::new(-10.0, -10.0, 0.0);
//     let cube_size = 100.0;
//     // vertices
//     let v1 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
//     let v2 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
//     let v3 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);
//     let v4 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);

//     let v5 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
//     let v6 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
//     let v7 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);
//     let v8 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);

//     // faces
//     let mut triangles = vec![
//         // floor
//         Triangle3d::new(v1, v2, v3, red),
//         Triangle3d::new(v1, v3, v4, red),
//         // lid
//         Triangle3d::new(v5, v6, v7, blue),
//         Triangle3d::new(v5, v7, v8, blue),
//         // sides
//         Triangle3d::new(v1, v2, v6, green),
//         Triangle3d::new(v1, v5, v6, green),
//         Triangle3d::new(v2, v3, v7, cyan),
//         Triangle3d::new(v2, v6, v7, cyan),
//         Triangle3d::new(v3, v4, v8, yellow),
//         Triangle3d::new(v3, v7, v8, yellow),
//         Triangle3d::new(v4, v1, v5, magenta),
//         Triangle3d::new(v4, v8, v5, magenta),
//     ];

//     for triangle in triangles.iter_mut() {
//         triangle.p1 += cube_origin;
//         triangle.p2 += cube_origin;
//         triangle.p3 += cube_origin;
//     }

//     let mut cam_pos = Vector3d::new(150.0, 130.0, 20.0);
//     let mut cam_look_at = Vector3d::new(1.0, 0.0, 0.0);
//     let z_up = Vector3d::new(0.0, 0.0, 1.0); // camera up
//     let mut fov = 120.0;

//     while window.is_open() && !window.is_key_down(Key::Enter) && !window.is_key_down(Key::Space) {
//         // render loop
//         canvas.reset();
//         canvas.checker(
//             &WHITE,
//             &Color {
//                 r: (0.0),
//                 g: (0.0),
//                 b: (0.0),
//                 a: (0.1),
//             },
//         );

//         // get active keys
//         let keys_down = window.get_keys();
//         if keys_down.contains(&Key::W) {
//             let increment = cam_look_at.clone().normalize() * 10.0;
//             cam_pos.add(&(increment * -1.0));
//         }
//         if keys_down.contains(&Key::A) {
//             let increment = cam_look_at.clone() * 10.0;
//             cam_pos.add(&(increment.cross(z_up).normalize()));
//         }
//         if keys_down.contains(&Key::S) {
//             let increment = cam_look_at.clone().normalize() * 10.0;
//             cam_pos.add(&(increment));
//         }
//         if keys_down.contains(&Key::D) {
//             let increment = cam_look_at.clone() * 10.0;
//             cam_pos.add(&(increment.cross(z_up).normalize() * -1.0));
//         }

//         // camera space stuff
//         let e = cam_pos;
//         let a = cam_look_at;
//         let t = z_up;

//         let g = a - e;
//         let w = (g * -1.0) / g.length();
//         let u = t.cross(w) / (t.cross(w)).length();
//         let v = w.cross(u);

//         // projection stuff
//         let l = -100.0;
//         let r = 100.0;
//         let b = -100.0;
//         let t = 100.0;
//         let n = 1.0;
//         let f = 1_000.0;

//         let camera_space_matrix = Matrix3x3::from_vecs(u, v, w);
//         let ortho_projection_matrix = Matrix4x4::from_vecs(
//             Vector4d::new(2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)),
//             Vector4d::new(0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)),
//             Vector4d::new(0.0, 0.0, 1.0 / (n - f), -n / (f - n)),
//             Vector4d::new(0.0, 0.0, 0.0, 1.0),
//         );

//         // finally, triangles
//         for triangle in triangles.iter() {
//             let p1_cam = camera_space_matrix.times_vec(triangle.p1 - e);
//             let p2_cam = camera_space_matrix.times_vec(triangle.p2 - e);
//             let p3_cam = camera_space_matrix.times_vec(triangle.p3 - e);

//             let p1_cam_homo = Vector4d::from_vector3d(p1_cam);
//             let p2_cam_homo = Vector4d::from_vector3d(p2_cam);
//             let p3_cam_homo = Vector4d::from_vector3d(p3_cam);

//             let p1_projected = ortho_projection_matrix.times_vec(p1_cam_homo);
//             let p2_projected = ortho_projection_matrix.times_vec(p2_cam_homo);
//             let p3_projected = ortho_projection_matrix.times_vec(p3_cam_homo);

//             let mut p1_2d = Vector2d::new(p1_projected.x, p1_projected.y);
//             let mut p2_2d = Vector2d::new(p2_projected.x, p2_projected.y);
//             let mut p3_2d = Vector2d::new(p3_projected.x, p3_projected.y);

//             let offset = Vector2d::new(SIZE_X as f64 / 2.0, SIZE_Y as f64 / 2.0);

//             p1_2d *= offset / 2.0;
//             p2_2d *= offset / 2.0;
//             p3_2d *= offset / 2.0;

//             p1_2d += offset;
//             p2_2d += offset;
//             p3_2d += offset;

//             draw_polygon_onto_buffer(
//                 &vec![
//                     IntegerVector2d::new(
//                         p1_2d.x as i32,
//                         p1_2d.y as i32,
//                         triangle.color.as_f64_vec(),
//                     ),
//                     IntegerVector2d::new(
//                         p2_2d.x as i32,
//                         p2_2d.y as i32,
//                         triangle.color.as_f64_vec(),
//                     ),
//                     IntegerVector2d::new(
//                         p3_2d.x as i32,
//                         p3_2d.y as i32,
//                         triangle.color.as_f64_vec(),
//                     ),
//                 ],
//                 &mut canvas,
//                 false,
//             );
//         }

//         // update minifb with new buffer
//         window.update_with_buffer(&canvas.buffer, SIZE_X as usize, SIZE_Y as usize)?;

//         println!("Rendertime: {} ms", global_timer.elapsed().as_millis());
//         global_timer = Instant::now();
//         thread::sleep(ANIM_INTERVAL);
//     }

//     Ok(())
// }
