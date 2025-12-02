use crate::util::calc_perspective_matrix;
use crate::vectors::Vector3d;
use crate::vectors::matrices::Matrix4x4;
use minifb::MouseMode;

#[derive(Debug, Clone)]
pub struct Camera {
    pub e: Vector3d,
    pub a: Vector3d,
    pub u: Vector3d,
    pub l: f64,
    pub r: f64,
    pub b: f64,
    pub t: f64,
    pub n: f64,
    pub f: f64,
}

impl Camera {
    pub fn new(
        e: Vector3d,
        a: Vector3d,
        u: Vector3d,
        l: f64,
        r: f64,
        b: f64,
        t: f64,
        n: f64,
        f: f64,
    ) -> Camera {
        Self {
            e,
            a,
            u,
            r,
            l,
            b,
            t,
            n,
            f,
        }
    }

    pub fn calc_perspective_projection_matrix(&self) -> Matrix4x4 {
        return calc_perspective_matrix(self.l, self.r, self.b, self.t, self.n, self.f);
    }

    pub fn calc_guvw(&self) -> (Vector3d, Vector3d, Vector3d, Vector3d) {
        // vector "from look at to camera"
        let g = self.a - self.e;

        // camera space spanning vectors
        let w = g.normalize() * -1.0;
        let u = self.u.cross(w).normalize();
        let v = w.cross(u);

        return (g, w, u, v);
    }
}
