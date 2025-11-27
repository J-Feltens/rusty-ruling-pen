use crate::util::clamp;
use crate::vectors::{Vector3d, Vector4d};
use rand::Rng;
//
// #[derive(Copy, Clone, Debug, PartialEq, Default)]
// pub struct Color {
//     pub rgba: Vector4d,
// }
//
// impl Color {
//     pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
//         Self {
//             rgba: Vector4d::new(r, g, b, a),
//         }
//     }
//
//     pub fn named_color(color: &str) -> Self {
//         if color == "red" {
//             Self::new(1.0, 0.0, 0.0, 1.0)
//         } else if color == "green" {
//             Self::new(0.0, 1.0, 0.0, 1.0)
//         } else if color == "blue" {
//             Self::new(0.0, 0.0, 1.0, 1.0)
//         } else if color == "cyan" {
//             Self::new(0.0, 1.0, 1.0, 1.0)
//         } else if color == "yellow" {
//             Self::new(1.0, 1.0, 0.0, 1.0)
//         } else if color == "magenta" {
//             Self::new(1.0, 0.0, 1.0, 1.0)
//         } else if color == "black" {
//             Self::new(0.0, 0.0, 0.0, 1.0)
//         } else if color == "white" {
//             Self::new(1.0, 1.0, 1.0, 1.0)
//         } else {
//             panic!("I don't have all your colors... {}", color);
//         }
//     }
//
//     pub fn r(&self) -> &f64 {
//         &self.rgba.x
//     }
//     pub fn g(&self) -> &f64 {
//         &self.rgba.y
//     }
//     pub fn b(&self) -> &f64 {
//         &self.rgba.z
//     }
//     pub fn a(&self) -> &f64 {
//         &self.rgba.u
//     }
//
//     pub fn random() -> Self {
//         let mut rng = rand::rng();
//         Self::new(
//             rng.random::<f64>(),
//             rng.random::<f64>(),
//             rng.random::<f64>(),
//             1.0,
//         )
//     }
//
//     pub fn from_u32(val: u32) -> Self {
//         // Hard resets alpha to 1.0 (suboptimal, i know i know.)
//         let r = ((val >> 16) & 0xFF) as f64 / 255.0;
//         let g = ((val >> 8) & 0xFF) as f64 / 255.0;
//         let b = (val & 0xFF) as f64 / 255.0;
//
//         Self::new(r, g, b, 1.0)
//     }
//
//     pub fn as_u32(&self) -> u32 {
//         (((self.r() * 255.0) as u32) << 16)
//             | (((self.g() * 255.0) as u32) << 8)
//             | ((self.b() * 255.0) as u32)
//     }
//
//     pub fn apply_lighting(&self, mut l: f64) -> Color {
//         l = clamp(l);
//         Self::new(self.r() * l, self.g() * l, self.b() * l, *self.a())
//     }
//     pub fn apply_colored_lighting(&self, mut r: f64, mut g: f64, mut b: f64) -> Color {
//         r = clamp(r);
//         g = clamp(g);
//         b = clamp(b);
//         Self::new(self.r() * r, self.g() * g, self.b() * b, *self.a())
//     }
// }
pub fn named_color(color: &str) -> Vector4d {
    if color == "red" {
        Vector4d::new(1.0, 0.0, 0.0, 1.0)
    } else if color == "green" {
        Vector4d::new(0.0, 1.0, 0.0, 1.0)
    } else if color == "blue" {
        Vector4d::new(0.0, 0.0, 1.0, 1.0)
    } else if color == "cyan" {
        Vector4d::new(0.0, 1.0, 1.0, 1.0)
    } else if color == "yellow" {
        Vector4d::new(1.0, 1.0, 0.0, 1.0)
    } else if color == "magenta" {
        Vector4d::new(1.0, 0.0, 1.0, 1.0)
    } else if color == "black" {
        Vector4d::new(0.0, 0.0, 0.0, 1.0)
    } else if color == "white" {
        Vector4d::new(1.0, 1.0, 1.0, 1.0)
    } else {
        panic!("I don't have all your colors... {}", color);
    }
}

pub fn rgb_u8_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn rgb_f64_to_u32(r: f64, g: f64, b: f64) -> u32 {
    /*
       assumes r, g and b are all in range(0.0, ..., 1.0)
    */
    (((r * 255.0) as u32) << 16) | (((g * 255.0) as u32) << 8) | ((b * 255.0) as u32)
}

pub fn color_vec_to_u32(color: &Vector4d) -> u32 {
    rgb_f64_to_u32(color.x, color.y, color.z)
}

pub fn color_vec_from_u32(val: u32) -> Vector4d {
    // Hard resets alpha to 1.0 (suboptimal, i know i know.)
    let r = ((val >> 16) & 0xFF) as f64 / 255.0;
    let g = ((val >> 8) & 0xFF) as f64 / 255.0;
    let b = (val & 0xFF) as f64 / 255.0;

    Vector4d::new(r, g, b, 1.0)
}

pub fn color_vec_from_f64(r: f64, g: f64, b: f64, a: f64) -> Vector4d {
    Vector4d::new(r, g, b, a)
}

pub fn clamp_color(color: Vector4d) -> Vector4d {
    Vector4d::new(
        clamp(color.x),
        clamp(color.y),
        clamp(color.z),
        clamp(color.u),
    )
}

pub fn alpha_blend(color_1: &Vector4d, color_2: &Vector4d) -> Vector4d {
    // simply setting new alpha to the one of the second input, not sure if that's right
    clamp_color(Vector4d::new(
        color_1.x * (1.0 - color_2.u) + color_2.x * color_2.u,
        color_1.y * (1.0 - color_2.u) + color_2.y * color_2.u,
        color_1.z * (1.0 - color_2.u) + color_2.z * color_2.u,
        color_2.u,
    ))
}

pub fn apply_colored_lighting(base_color: &Vector4d, lighting_color: &Vector4d) -> Vector4d {
    Vector4d::new(
        base_color.x * lighting_color.x,
        base_color.y * lighting_color.y,
        base_color.z * lighting_color.z,
        base_color.u,
    )
}
