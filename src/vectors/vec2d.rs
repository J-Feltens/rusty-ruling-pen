use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn rotate_around_point(&mut self, phi: f64, center: Vector2d) {
        self.x -= center.x;
        self.y -= center.y;

        self.rotate(phi);

        self.x += center.x;
        self.y += center.y;
    }

    pub fn length(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }
}

impl fmt::Display for Vector2d {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
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

// v / scalar
impl Div<f64> for Vector2d {
    type Output = Vector2d;

    fn div(self, rhs: f64) -> Vector2d {
        Vector2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// v /= scalar
impl DivAssign<f64> for Vector2d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
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
