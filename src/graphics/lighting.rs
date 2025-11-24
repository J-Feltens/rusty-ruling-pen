use crate::graphics::Color;
use crate::vectors::Vector3d;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct PointLight {
    pub pos: Vector3d,
    pub strength: f64,
    pub emission: Color,
}

impl PointLight {
    pub fn new(pos: Vector3d, strength: f64, emission: Color) -> Self {
        Self {
            pos,
            strength,
            emission,
        }
    }
}
