use crate::{graphics::Color, vectors::Vector3d};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Triangle3d {
    pub p1: Vector3d,
    pub p2: Vector3d,
    pub p3: Vector3d,

    pub color: Color,
}

impl Triangle3d {
    pub fn new(p1: Vector3d, p2: Vector3d, p3: Vector3d, color: Color) -> Self {
        Self { p1, p2, p3, color }
    }

    // pub fn project(&self, z: f64) -> Vec<IntegerVector2d> {
    //     // project onto 2d at projection plane (0, 0, 1)
    //     // move color to attrs for gouraud
    //     return vec![
    //         IntegerVector2d::from_vector2d(
    //             self.p1.project(z),
    //             vec![self.color.r, self.color.g, self.color.b, self.color.a],
    //         ),
    //         IntegerVector2d::from_vector2d(
    //             self.p2.project(z),
    //             vec![self.color.r, self.color.g, self.color.b, self.color.a],
    //         ),
    //         IntegerVector2d::from_vector2d(
    //             self.p3.project(z),
    //             vec![self.color.r, self.color.g, self.color.b, self.color.a],
    //         ),
    //     ];
    // }
}
