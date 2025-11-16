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
    pub fn new(x: f64, y: f64, z: f64) -> Vector3d {
        Vector3d { x, y, z }
    }

    pub fn origin() -> Vector3d {
        Vector3d {
            x: (0.0),
            y: (0.0),
            z: (0.0),
        }
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

    pub fn rotate(&mut self, phi: f64) {
        // let x: f64 = self.x * f64::cos(phi) - self.y * f64::sin(phi);
        // let y: f64 = self.x * f64::sin(phi) + self.y * f64::cos(phi);

        // self.x = x;
        // self.y = y;
    }

    pub fn rotate_around_point(&mut self, phi: f64, center: Vector3d) {
        // self.x -= center.x;
        // self.y -= center.y;

        // self.rotate(phi);

        // self.x += center.x;
        // self.y += center.y;
    }

    pub fn length(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn cross(&self, v: Vector3d) -> Vector3d {
        return Vector3d::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        );
    }
}

// v1 + v2
impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
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

    fn sub(self, rhs: Vector3d) -> Vector3d {
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

// v * scalar
impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Vector3d {
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

    fn div(self, rhs: f64) -> Vector3d {
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

    fn mul(self, v: Vector3d) -> Vector3d {
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
