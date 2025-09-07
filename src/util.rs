use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::canvas::HEIGHT;
use crate::canvas::WIDTH;

/*
    Some simple vector utilites to kick it off
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    pub fn new(x: f64, y: f64) -> Vector2d {
        Vector2d { x: x, y: y }
    }

    pub fn origin() -> Vector2d {
        Vector2d { x: (0.0), y: (0.0) }
    }

    pub fn add(&mut self, v: &Vector2d) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn scale(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
    }

    pub fn rotate(&mut self, phi: f64) {
        let x: f64 = self.x * f64::cos(phi) - self.y * f64::sin(phi);
        let y: f64 = self.x * f64::sin(phi) + self.y * f64::cos(phi);

        self.x = x;
        self.y = y;
    }
}

// v1 + v2
impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// v1 += v2
impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Vector2d) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// v1 - v2
impl Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, rhs: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// v1 -= v2
impl SubAssign for Vector2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// v * scalar
impl Mul<f64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f64) -> Vector2d {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// v *= scalar
impl MulAssign<f64> for Vector2d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// v * vector
impl Mul<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn mul(self, v: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

// v *= vector
impl MulAssign<Vector2d> for Vector2d {
    fn mul_assign(&mut self, v: Vector2d) {
        self.x *= v.x;
        self.y *= v.y;
    }
}

/*
    An Object represents some sort of renderable, be it a circle, rectangle or, in the future, graphic
*/
#[derive(Debug)]
pub struct Object {
    pub origin: Vector2d,
    pub r: f64,
    pub color: u32,
}

impl Object {
    pub fn new(origin: Vector2d, r: f64, color: u32) -> Object {
        Object {
            origin: origin,
            r: r,
            color: color,
        }
    }

    pub fn set_origin(&mut self, vector: Vector2d) {
        self.origin = vector;
    }

    pub fn translate(&mut self, vector: Vector2d) {
        self.origin += vector;
    }

    pub fn draw_on_buffer(&self, buffer: &mut Vec<u32>) {
        // draw a circle for starters
        for y_ in 0..HEIGHT {
            for x_ in 0..WIDTH {
                if (x_ as i64 - self.origin.x as i64).pow(2)
                    + (y_ as i64 - self.origin.y as i64).pow(2)
                    < self.r as i64
                {
                    buffer[y_ * WIDTH + x_] = self.color;
                }
            }
        }
    }
}

pub struct Stack {
    pub stack: Vec<Object>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.stack.push(obj);
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.stack.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.stack.iter_mut()
    }
}
