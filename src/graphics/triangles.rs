use crate::{graphics::Color, vectors::Vector3d};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Triangle3d {
    pub vertices: Vec<Vector3d>,
    pub normal: Vector3d,

    pub color: Color,
}

impl Triangle3d {
    pub fn new(p1: Vector3d, p2: Vector3d, p3: Vector3d, color: &Color) -> Self {
        Self {
            vertices: vec![p1, p2, p3],
            normal: (p2 - p1).cross(p3 - p1).normalize(),
            color: color.clone(),
        }
    }

    pub fn p1(&self) -> &Vector3d {
        &self.vertices[0]
    }
    pub fn p2(&self) -> &Vector3d {
        &self.vertices[1]
    }
    pub fn p3(&self) -> &Vector3d {
        &self.vertices[2]
    }

    pub fn p1_mut(&mut self) -> &mut Vector3d {
        &mut self.vertices[0]
    }
    pub fn p2_mut(&mut self) -> &mut Vector3d {
        &mut self.vertices[1]
    }
    pub fn p3_mut(&mut self) -> &mut Vector3d {
        &mut self.vertices[2]
    }
}

impl fmt::Display for Triangle3d {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Triangle [{}, {}, {}] with normal {}",
            self.p1(),
            self.p2(),
            self.p3(),
            self.normal
        )
    }
}
