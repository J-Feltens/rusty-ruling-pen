use crate::util::clamp;
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r: (r),
            g: (g),
            b: (b),
            a: (a),
        }
    }

    pub fn named_color(color: &str) -> Self {
        if color == "red" {
            Self::new(1.0, 0.0, 0.0, 1.0)
        } else if color == "green" {
            Self::new(0.0, 1.0, 0.0, 1.0)
        } else if color == "blue" {
            Self::new(0.0, 0.0, 1.0, 1.0)
        } else if color == "cyan" {
            Self::new(0.0, 1.0, 1.0, 1.0)
        } else if color == "yellow" {
            Self::new(1.0, 1.0, 0.0, 1.0)
        } else if color == "magenta" {
            Self::new(1.0, 0.0, 1.0, 1.0)
        } else if color == "black" {
            Self::new(0.0, 0.0, 0.0, 1.0)
        } else if color == "white" {
            Self::new(1.0, 1.0, 1.0, 1.0)
        } else {
            panic!("I don't have all your colors... {}", color);
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self {
            r: rng.random::<f64>(),
            g: rng.random::<f64>(),
            b: rng.random::<f64>(),
            a: 1.0, // you can pick another if you want
        }
    }

    pub fn from_u32(val: u32) -> Self {
        // Hard resets alpha to 1.0 (suboptimal, i know i know.)
        let r = ((val >> 16) & 0xFF) as f64 / 255.0;
        let g = ((val >> 8) & 0xFF) as f64 / 255.0;
        let b = (val & 0xFF) as f64 / 255.0;

        Self { r, g, b, a: 1.0 }
    }

    pub fn as_u32(&self) -> u32 {
        (((self.r * 255.0) as u32) << 16)
            | (((self.g * 255.0) as u32) << 8)
            | ((self.b * 255.0) as u32)
    }

    pub fn as_f64_vec(self) -> Vec<f64> {
        return vec![self.r, self.g, self.b, self.a];
    }

    pub fn apply_lighting(&self, mut l: f64) -> Color {
        l = clamp(l);
        Self::new(self.r * l, self.g * l, self.b * l, self.a)
    }
    pub fn apply_colored_lighting(&self, mut r: f64, mut g: f64, mut b: f64) -> Color {
        r = clamp(r);
        g = clamp(g);
        b = clamp(b);
        Self::new(self.r * r, self.g * g, self.b * b, self.a)
    }
}

pub fn rgb2u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn alpha_blend(color_1: &Color, color_2: &Color) -> Color {
    // simply setting new alpha to the one of the second input, not sure if that's right
    return Color {
        r: (color_1.r * (1.0 - color_2.a) + color_2.r * color_2.a),
        g: (color_1.g * (1.0 - color_2.a) + color_2.g * color_2.a),
        b: (color_1.b * (1.0 - color_2.a) + color_2.b * color_2.a),
        a: (color_2.a),
    };
}
