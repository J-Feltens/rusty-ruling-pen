use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::vectors::{IntegerVector2d, Vector2d};

/*
    3d implementation of Vector2d
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn test() -> Self {
        Self::new(1.0, 2.0, 3.0)
    }

    pub fn add(&mut self, v: &Vector3d) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    pub fn scale(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn length(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn sum(&self) -> f64 {
        return self.x + self.y + self.z;
    }

    pub fn dot(&self, vec: Vector3d) -> f64 {
        return (*self * vec).sum();
    }

    pub fn cross(&self, v: Vector3d) -> Self {
        return Vector3d::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        );
    }
}

impl fmt::Display for Vector3d {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

// v1 + v2
impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// v1 += v2
impl AddAssign for Vector3d {
    fn add_assign(&mut self, rhs: Vector3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// v1 - v2
impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// v1 -= v2
impl SubAssign for Vector3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// v + scalar
impl Add<f64> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: f64) -> Self {
        Vector3d {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// v += scalar
impl AddAssign<f64> for Vector3d {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// v * scalar
impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Self {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// v *= scalar
impl MulAssign<f64> for Vector3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// v / scalar
impl Div<f64> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f64) -> Self {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// v /= scalar
impl DivAssign<f64> for Vector3d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// v * vector
impl Mul<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn mul(self, v: Vector3d) -> Self {
        Vector3d {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

// v *= vector
impl MulAssign<Vector3d> for Vector3d {
    fn mul_assign(&mut self, v: Vector3d) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
    }
}
