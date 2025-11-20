use crate::vectors::{Vector3d, Vector4d};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix3x3 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub g: f64,
    pub h: f64,
    pub i: f64,
}

impl Matrix3x3 {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Self {
        Matrix3x3 {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
        }
    }

    pub fn from_vecs(v1: Vector3d, v2: Vector3d, v3: Vector3d) -> Self {
        // vectors as rows
        return Self::new(v1.x, v1.y, v1.z, v2.x, v2.y, v2.z, v3.x, v3.y, v3.z);
    }

    pub fn times_vec(&self, vec: Vector3d) -> Vector3d {
        return Vector3d::new(
            self.a * vec.x + self.b * vec.y + self.c * vec.z,
            self.d * vec.x + self.e * vec.y + self.f * vec.z,
            self.g * vec.x + self.h * vec.y + self.i * vec.z,
        );
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix4x4 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,

    pub e: f64,
    pub f: f64,
    pub g: f64,
    pub h: f64,

    pub i: f64,
    pub j: f64,
    pub k: f64,
    pub l: f64,

    pub m: f64,
    pub n: f64,
    pub o: f64,
    pub p: f64,
}

impl Matrix4x4 {
    pub fn new(
        a: f64,
        b: f64,
        c: f64,
        d: f64,

        e: f64,
        f: f64,
        g: f64,
        h: f64,

        i: f64,
        j: f64,
        k: f64,
        l: f64,

        m: f64,
        n: f64,
        o: f64,
        p: f64,
    ) -> Self {
        Matrix4x4 {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            j,
            k,
            l,
            m,
            n,
            o,
            p,
        }
    }

    pub fn times_vec(&self, vec: Vector4d) -> Vector4d {
        return Vector4d::new(
            self.a * vec.x + self.b * vec.y + self.c * vec.z + self.d * vec.u,
            self.e * vec.x + self.f * vec.y + self.g * vec.z + self.h * vec.u,
            self.i * vec.x + self.j * vec.y + self.k * vec.z + self.l * vec.u,
            self.m * vec.x + self.n * vec.y + self.o * vec.z + self.p * vec.z,
        );
    }

    pub fn from_vecs(v1: Vector4d, v2: Vector4d, v3: Vector4d, v4: Vector4d) -> Self {
        // vectors as rows
        return Self::new(
            v1.x, v1.y, v1.z, v1.u, v2.x, v2.y, v2.z, v2.u, v3.x, v3.y, v3.z, v3.u, v4.x, v4.y,
            v4.z, v4.u,
        );
    }
}
