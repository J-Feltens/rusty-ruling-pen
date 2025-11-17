use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    vec,
};

use crate::vectors::Vector2d;

/*
    An extended version of Vector2d used for rasterization, featuring:

    - i32 integer coords
    - a list of attributes, which will be interpolated using gouraud shading technique
*/
#[derive(Clone, Debug, PartialEq, Default)]
pub struct IntegerVector2d {
    pub x: i32,
    pub y: i32,

    pub attrs: Vec<f64>, // attributes stored here will be interpolated using gouraud shading
}

impl IntegerVector2d {
    pub fn new(x: i32, y: i32, attrs: Vec<f64>) -> IntegerVector2d {
        IntegerVector2d { x, y, attrs }
    }

    pub fn from_floats(x: f64, y: f64, attrs: Vec<f64>) -> IntegerVector2d {
        IntegerVector2d {
            x: x.round() as i32,
            y: y.round() as i32,
            attrs,
        }
    }

    pub fn from_vector2d(vector: Vector2d, attrs: Vec<f64>) -> IntegerVector2d {
        IntegerVector2d {
            x: vector.x.round() as i32,
            y: vector.y.round() as i32,
            attrs,
        }
    }

    pub fn origin() -> IntegerVector2d {
        IntegerVector2d {
            x: (0),
            y: (0),
            attrs: Vec::new(),
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

/*
    A simple struct to store a polygon consisting of n IntegerVector2ds
*/
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Polygon2d {
    pub vertices: Vec<IntegerVector2d>,
}

impl Polygon2d {
    pub fn new(vertices: Vec<IntegerVector2d>) -> Polygon2d {
        Polygon2d { vertices }
    }
    
}
