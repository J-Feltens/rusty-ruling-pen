use std::ptr::null;

#[derive(Clone, Debug, PartialEq, Default)]
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
