use crate::vectors::{Vector3d, Vector4d};
use std::fmt;

/*
    This implementation of matrices uses vectors as underlying datastructure.
    Hence, 3x3 matrix is represented by 3 Vector3d objects, where every vector represents a row:

    Matrix3x3 = [
        Vector3d_1.x, Vector3d_1.y, Vector3d_1.z,
        Vector3d_2.x, Vector3d_2.y, Vector3d_2.z,
        Vector3d_3.x, Vector3d_3.y, Vector3d_3.z,
    ]
*/

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix3x3 {
    pub a: Vector3d,
    pub b: Vector3d,
    pub c: Vector3d,
}

impl Matrix3x3 {
    pub fn from_floats(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        g: f64,
        h: f64,
        i: f64,
    ) -> Self {
        Matrix3x3 {
            a: Vector3d::new(a, b, c),
            b: Vector3d::new(d, e, f),
            c: Vector3d::new(g, h, i),
        }
    }

    pub fn eye() -> Self {
        Self::from_vecs(
            Vector3d::new(1.0, 0.0, 0.0),
            Vector3d::new(0.0, 1.0, 0.0),
            Vector3d::new(0.0, 0.0, 1.0),
        )
    }

    pub fn test() -> Self {
        Self::from_vecs(
            Vector3d::test(),
            Vector3d::test() + 3.0,
            Vector3d::test() + 6.0,
        )
    }

    pub fn from_vecs(v1: Vector3d, v2: Vector3d, v3: Vector3d) -> Self {
        // vectors as rows
        return Self {
            a: v1,
            b: v2,
            c: v3,
        };
    }

    pub fn times_vec(&self, vec: Vector3d) -> Vector3d {
        return Vector3d::new(self.a.dot(vec), self.b.dot(vec), self.c.dot(vec));
    }
}

impl fmt::Display for Matrix3x3 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},\n {},\n {}]", self.a, self.b, self.c)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Matrix4x4 {
    pub a: Vector4d,
    pub b: Vector4d,
    pub c: Vector4d,
    pub d: Vector4d,
}

impl Matrix4x4 {
    pub fn from_floats(
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
        Self {
            a: Vector4d::new(a, b, c, d),
            b: Vector4d::new(e, f, g, h),
            c: Vector4d::new(i, j, k, l),
            d: Vector4d::new(m, n, o, p),
        }
    }

    pub fn from_vecs(v1: Vector4d, v2: Vector4d, v3: Vector4d, v4: Vector4d) -> Self {
        // vectors as rows
        return Self {
            a: v1,
            b: v2,
            c: v3,
            d: v4,
        };
    }

    pub fn test() -> Self {
        // vectors as rows
        return Self::from_vecs(
            Vector4d::test(),
            Vector4d::test() + 4.0,
            Vector4d::test() + 8.0,
            Vector4d::test() + 12.0,
        );
    }

    pub fn times_vec(&self, vec: Vector4d) -> Vector4d {
        return Vector4d::new(
            self.a.dot(vec),
            self.b.dot(vec),
            self.c.dot(vec),
            self.d.dot(vec),
        );
    }
}

impl fmt::Display for Matrix4x4 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},\n {},\n {},\n {}]", self.a, self.b, self.c, self.d)
    }
}
