use crate::colors::BLACK;
use crate::colors::BLUE;
use crate::colors::CYAN;
use crate::colors::GREEN;
use crate::colors::MAGENTA;
use crate::colors::RED;
use crate::colors::YELLOW;
use crate::util::Line3d;
use crate::util::Vector2d;
use crate::util::Vector3d;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct CoordSystem {
    pub origin: Vector3d,

    pub u: Vector3d,
    pub v: Vector3d,
    pub w: Vector3d,

    pub u_line: Line3d,
    pub v_line: Line3d,
    pub w_line: Line3d,
}

impl CoordSystem {
    pub fn new(origin: Vector3d) -> CoordSystem {
        let u = Vector3d::new(50.0, 0.0, 0.0) + origin;
        let v = Vector3d::new(0.0, 50.0, 0.0) + origin;
        let w = Vector3d::new(0.0, 0.0, 50.0) + origin;
        CoordSystem {
            origin: origin,

            u: u,
            v: v,
            w: w,

            u_line: Line3d::new(origin, u, &RED),
            v_line: Line3d::new(origin, v, &GREEN),
            w_line: Line3d::new(origin, w, &BLUE),
        }
    }

    pub fn get_lines(&self) -> Vec<Line3d> {
        return vec![self.u_line, self.v_line, self.w_line];
    }
}
