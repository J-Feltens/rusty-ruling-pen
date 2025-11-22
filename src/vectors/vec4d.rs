use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::vectors::{IntegerVector2d, Vector2d, Vector3d};

/*
    3d implementation of Vector2d
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
}

impl Vector4d {
    pub fn new(x: f64, y: f64, z: f64, u: f64) -> Self {
        Self { x, y, z, u }
    }

    pub fn from_vector3d(vector: Vector3d) -> Self {
        return Self::new(vector.x, vector.y, vector.z, 0.0);
    }

    pub fn zero() -> Self {
        Self {
            x: (0.0),
            y: (0.0),
            z: (0.0),
            u: (0.0),
        }
    }

    pub fn test() -> Self {
        return Self::new(1.0, 2.0, 3.0, 4.0);
    }

    pub fn length(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.u * self.u))
            .sqrt();
    }

    pub fn sum(&self) -> f64 {
        return self.x + self.y + self.z + self.u;
    }

    pub fn add(&mut self, v: &Vector4d) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self.u += v.u;
    }

    pub fn scale(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.u *= s;
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            u: self.u / len,
        }
    }
}

impl fmt::Display for Vector4d {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.u)
    }
}

// v1 + v2
impl Add for Vector4d {
    type Output = Vector4d;

    fn add(self, rhs: Vector4d) -> Self {
        Vector4d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            u: self.u + rhs.u,
        }
    }
}

// v1 += v2
impl AddAssign for Vector4d {
    fn add_assign(&mut self, rhs: Vector4d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.u += rhs.u;
    }
}

// v1 - v2
impl Sub for Vector4d {
    type Output = Vector4d;

    fn sub(self, rhs: Vector4d) -> Self {
        Vector4d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            u: self.u - rhs.u,
        }
    }
}

// v1 -= v2
impl SubAssign for Vector4d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.u -= rhs.u;
    }
}

// v + scalar
impl Add<f64> for Vector4d {
    type Output = Vector4d;

    fn add(self, rhs: f64) -> Self {
        Vector4d {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            u: self.u + rhs,
        }
    }
}

// v += scalar
impl AddAssign<f64> for Vector4d {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.u += rhs;
    }
}

// v * scalar
impl Mul<f64> for Vector4d {
    type Output = Vector4d;

    fn mul(self, rhs: f64) -> Self {
        Vector4d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            u: self.u * rhs,
        }
    }
}

// v *= scalar
impl MulAssign<f64> for Vector4d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.u *= rhs;
    }
}

// v / scalar
impl Div<f64> for Vector4d {
    type Output = Vector4d;

    fn div(self, rhs: f64) -> Self {
        Vector4d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            u: self.u / rhs,
        }
    }
}

// v /= scalar
impl DivAssign<f64> for Vector4d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.u /= rhs;
    }
}

// v * vector
impl Mul<Vector4d> for Vector4d {
    type Output = Vector4d;

    fn mul(self, v: Vector4d) -> Self {
        Vector4d {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
            u: self.u * v.u,
        }
    }
}

// v *= vector
impl MulAssign<Vector4d> for Vector4d {
    fn mul_assign(&mut self, v: Vector4d) {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self.u *= v.u;
    }
}
