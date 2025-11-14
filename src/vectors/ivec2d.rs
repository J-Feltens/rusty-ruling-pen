// integer
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/*
    An extended version of Vector2d used for rasterization, featuring:

    - i32 integer coords
    - a list of attributes, which will be interpolated using gouraud shading technique
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct IntegerVector2d {
    pub x: i32,
    pub y: i32,

    pub attr: f64, // varying attribute for testing interpolation, will be used as brightness
}

impl IntegerVector2d {
    pub fn new(x: i32, y: i32, attr: f64) -> IntegerVector2d {
        IntegerVector2d { x, y, attr }
    }

    pub fn from_floats(x: f64, y: f64, attr: f64) -> IntegerVector2d {
        IntegerVector2d {
            x: x.round() as i32,
            y: y.round() as i32,
            attr,
        }
    }

    pub fn origin() -> IntegerVector2d {
        IntegerVector2d {
            x: (0),
            y: (0),
            attr: 0.0,
        }
    }

    pub fn add(&mut self, v: &IntegerVector2d) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn scale(&mut self, s: i32) {
        self.x *= s;
        self.y *= s;
    }

    pub fn length(&self) -> f64 {
        return (((self.x * self.x) + (self.y * self.y)) as f64).sqrt();
    }
}
