use crate::vectors::Vector3d;

pub struct PointLight {
    pub pos: Vector3d,
    pub emission: f64,
}

impl PointLight {
    pub fn new(pos: Vector3d, emission: f64) -> Self {
        Self { pos, emission }
    }
}
