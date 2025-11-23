use crate::graphics::{Color, Triangle3d};
use crate::vectors::Vector3d;

pub struct Cube {
    pub triangles: Vec<Triangle3d>,
}

impl Cube {
    pub fn new(cube_size: f64, center: Vector3d) -> Self {
        // colors
        let red = Color::new(1.0, 0.0, 0.0, 1.0);
        let green = Color::new(0.0, 1.0, 0.0, 1.0);
        let blue = Color::new(0.0, 0.0, 1.0, 1.0);
        let cyan = Color::new(1.0, 0.0, 1.0, 1.0);
        let yellow = Color::new(1.0, 1.0, 0.0, 1.0);
        let magenta = Color::new(0.0, 1.0, 1.0, 1.0);

        // vertices
        let v1 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
        let v2 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0);
        let v3 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);
        let v4 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0);

        let v5 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
        let v6 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0);
        let v7 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);
        let v8 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, cube_size / 2.0);

        // faces
        let mut triangles = vec![
            // floor
            Triangle3d::new(v1, v2, v3, red),
            Triangle3d::new(v1, v3, v4, red),
            // lid
            Triangle3d::new(v5, v6, v7, blue),
            Triangle3d::new(v5, v7, v8, blue),
            // sides
            Triangle3d::new(v1, v2, v6, green),
            Triangle3d::new(v1, v5, v6, green),
            Triangle3d::new(v2, v3, v7, cyan),
            Triangle3d::new(v2, v6, v7, cyan),
            Triangle3d::new(v3, v4, v8, yellow),
            Triangle3d::new(v3, v7, v8, yellow),
            Triangle3d::new(v4, v1, v5, magenta),
            Triangle3d::new(v4, v8, v5, magenta),
        ];

        for triangle in triangles.iter_mut() {
            triangle.p1 += center;
            triangle.p2 += center;
            triangle.p3 += center;
        }

        return Self { triangles };
    }
}
