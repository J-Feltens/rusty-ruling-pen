use crate::vectors::{Vector3d, Vector4d};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct PointLight {
    pub pos: Vector3d,
    pub strength: f64,
    pub emission: Vector4d,
}

impl PointLight {
    pub fn new(pos: Vector3d, strength: f64, emission: Vector4d) -> Self {
        Self {
            pos,
            strength,
            emission,
        }
    }
}
