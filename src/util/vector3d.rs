use rand::rand_core::le;

use crate::util::Vector2d;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/*
    Some simple vector utilites to kick it off
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3d {
        Vector3d { x: x, y: y, z: z }
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

    pub fn add_xyz(&mut self, x: f64, y: f64, z: f64) -> Vector3d {
        self.x += x;
        self.y += y;
        self.z += z;

        return *self;
    }

    pub fn scale(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn length(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    pub fn set_v(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn rotate_euler(&mut self, roll: f64, pitch: f64, yaw: f64) {
        let (ca, cb, cg) = (yaw.cos(), pitch.cos(), roll.cos());
        let (sa, sb, sg) = (yaw.sin(), pitch.sin(), roll.sin());

        let new_x = (ca * cb) * self.x
            + (ca * sb * sg - sa * cg) * self.y
            + (ca * sb * cg + sa * sg) * self.z;

        let new_y = sa * cb * self.x
            + (sa * sb * sg + ca * cg) * self.y
            + (sa * sb * cg - ca * sg) * self.z;

        let new_z = -sb * self.x + cb * sg * self.y + cb * cg * self.z;

        (self.x, self.y, self.z) = (new_x, new_y, new_z);
    }

    fn rotate_x(&mut self, angle: f64) {
        let (c, s) = (angle.cos(), angle.sin());
        let (x, y, z) = (self.x, self.y, self.z);
        self.y = c * y - s * z;
        self.z = s * y + c * z;
        self.x = x;
    }

    fn rotate_y(&mut self, angle: f64) {
        let (c, s) = (angle.cos(), angle.sin());
        let (x, y, z) = (self.x, self.y, self.z);
        self.x = c * x + s * z;
        self.z = -s * x + c * z;
        self.y = y;
    }

    fn rotate_z(&mut self, angle: f64) {
        let (c, s) = (angle.cos(), angle.sin());
        let (x, y, z) = (self.x, self.y, self.z);
        self.x = c * x - s * y;
        self.y = s * x + c * y;
        self.z = z;
    }

    pub fn rotate_euler_xyz(&mut self, x: f64, y: f64, z: f64) {
        self.rotate_x(x);
        self.rotate_y(y);
        self.rotate_z(z);
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
