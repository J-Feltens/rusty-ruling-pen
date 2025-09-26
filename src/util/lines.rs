use crate::colors::Color;
use crate::util::Vector2d;
use crate::util::Vector3d;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Line3d {
    pub v1: Vector3d,
    pub v2: Vector3d,
    pub color: Color,
}

impl Line3d {
    pub fn new(v1: Vector3d, v2: Vector3d, color: &Color) -> Line3d {
        Line3d {
            v1: v1,
            v2: v2,
            color: color.clone(),
        }
    }
}

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

pub struct Lines3d {
    pub lines: Vec<Line3d>,
}

impl Lines3d {
    pub fn new(lines_endpoints: Vec<(Vector3d, Vector3d)>, color: &Color) -> Lines3d {
        let mut lines: Vec<Line3d> = Vec::new();
        for i in 0..lines_endpoints.len() {
            lines.push(Line3d {
                v1: (lines_endpoints[i].0),
                v2: (lines_endpoints[i].1),
                color: color.clone(),
            });
        }
        Lines3d { lines: lines }
    }
}
