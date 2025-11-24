use crate::vectors::Vector3d;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct PointLight {
    pub pos: Vector3d,
    pub emission: f64,
}

impl PointLight {
    pub fn new(pos: Vector3d, emission: f64) -> Self {
        Self { pos, emission }
    }
}
