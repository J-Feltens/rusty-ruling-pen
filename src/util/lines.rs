use crate::colors::Color;
use crate::util::Vector2d;
use crate::util::Vector3d;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Line2d {
    pub v1: Vector2d,
    pub v2: Vector2d,
}

impl Line2d {
    pub fn new(v1: Vector2d, v2: Vector2d) -> Line2d {
        Line2d { v1: v1, v2: v2 }
    }
}
