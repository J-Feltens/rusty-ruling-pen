// integer
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/*
    Some simple vector utilites to kick it off
*/
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct IntegerVector2d {
    pub x: i32,
    pub y: i32,
}

impl IntegerVector2d {
    pub fn new(x: i32, y: i32) -> IntegerVector2d {
        IntegerVector2d { x: x, y: y }
    }

    pub fn origin() -> IntegerVector2d {
        IntegerVector2d { x: (0), y: (0) }
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

// v1 + v2
impl Add for IntegerVector2d {
    type Output = IntegerVector2d;

    fn add(self, rhs: IntegerVector2d) -> IntegerVector2d {
        IntegerVector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// v1 += v2
impl AddAssign for IntegerVector2d {
    fn add_assign(&mut self, rhs: IntegerVector2d) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// v1 - v2
impl Sub for IntegerVector2d {
    type Output = IntegerVector2d;

    fn sub(self, rhs: IntegerVector2d) -> IntegerVector2d {
        IntegerVector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// v1 -= v2
impl SubAssign for IntegerVector2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// v * scalar
impl Mul<i32> for IntegerVector2d {
    type Output = IntegerVector2d;

    fn mul(self, rhs: i32) -> IntegerVector2d {
        IntegerVector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// v *= scalar
impl MulAssign<i32> for IntegerVector2d {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// v / scalar
impl Div<i32> for IntegerVector2d {
    type Output = IntegerVector2d;

    fn div(self, rhs: i32) -> IntegerVector2d {
        IntegerVector2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// v /= scalar
impl DivAssign<i32> for IntegerVector2d {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// v * vector
impl Mul<IntegerVector2d> for IntegerVector2d {
    type Output = IntegerVector2d;

    fn mul(self, v: IntegerVector2d) -> IntegerVector2d {
        IntegerVector2d {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

// v *= vector
impl MulAssign<IntegerVector2d> for IntegerVector2d {
    fn mul_assign(&mut self, v: IntegerVector2d) {
        self.x *= v.x;
        self.y *= v.y;
    }
}
