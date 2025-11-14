use std::ptr::null;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Color {
        Color {
            r: (r),
            g: (g),
            b: (b),
            a: (a),
        }
    }

    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

pub fn rgb2u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub const TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0.0,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 1.0,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 1.0,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 1.0,
};
pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 1.0,
};
pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 1.0,
};

pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 1.0,
};

pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 1.0,
};
