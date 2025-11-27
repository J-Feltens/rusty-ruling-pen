use crate::graphics::Triangle3d;
use crate::graphics::colors::named_color;
use crate::util::linspace;
use crate::vectors::{Matrix3x3, Vector3d, Vector4d};
use image::error::UnsupportedErrorKind::Color;
use std::f64::consts::PI;

pub fn calc_cube(cube_size: f64, center: Vector3d) -> Vec<Triangle3d> {
    // colors
    let red = &named_color("red");
    let green = &named_color("green");
    let blue = &named_color("blue");
    let cyan = &named_color("cyan");
    let yellow = &named_color("yellow");
    let magenta = &named_color("magenta");

    // vertices
    let v1 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0) + center;
    let v2 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0) + center;
    let v3 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0) + center;
    let v4 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0) + center;

    let v5 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0) + center;
    let v6 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0) + center;
    let v7 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0) + center;
    let v8 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, cube_size / 2.0) + center;

    // faces
    let triangles = vec![
        // floor
        Triangle3d::new(v3, v2, v1, red),
        Triangle3d::new(v4, v3, v1, red),
        // lid
        Triangle3d::new(v5, v6, v7, magenta),
        Triangle3d::new(v5, v7, v8, magenta),
        // sides
        Triangle3d::new(v1, v2, v6, yellow),
        Triangle3d::new(v6, v5, v1, yellow),
        Triangle3d::new(v2, v3, v7, cyan),
        Triangle3d::new(v7, v6, v2, cyan),
        Triangle3d::new(v3, v4, v8, green),
        Triangle3d::new(v8, v7, v3, green),
        Triangle3d::new(v4, v1, v5, blue),
        Triangle3d::new(v5, v8, v4, blue),
    ];
    return triangles;
}
//
pub fn calc_torus(
    origin: Vector3d,
    major_radius: f64,
    minor_radius: f64,
    major_resolution: usize,
    minor_resolution: usize,
    color: &Vector4d,
) -> Vec<Triangle3d> {
    let phis = linspace(0.0, 2.0 * PI, major_resolution);
    let thetas = linspace(0.0, 2.0 * PI, minor_resolution);

    let mut vertices = vec![];
    for major in 0..major_resolution {
        let rot_mat = Matrix3x3::calc_rotation_matrix(Vector3d::new(0.0, 0.0, 1.0), phis[major]);
        for minor in 0..minor_resolution {
            let mut new_vec = Vector3d::new(
                minor_radius * thetas[minor].cos() + major_radius,
                0.0,
                minor_radius * thetas[minor].sin(),
            );
            new_vec = rot_mat.times_vec(new_vec);
            vertices.push(new_vec + origin);
        }
    }

    let mut triangles = vec![];
    for major in 0..major_resolution {
        for minor in 0..minor_resolution {
            let (p1, p2, p3, p4) = (
                vertices[(minor_resolution * (major % major_resolution)) + minor],
                vertices[(minor_resolution * (major % major_resolution))
                    + (minor + 1) % minor_resolution],
                vertices[(minor_resolution * ((major + 1) % major_resolution)) + minor],
                vertices[(minor_resolution * ((major + 1) % major_resolution))
                    + ((minor + 1) % minor_resolution)],
            );

            // let rand_color = Color::random();
            triangles.push(Triangle3d::new(p3, p2, p1, &color));
            triangles.push(Triangle3d::new(p2, p3, p4, &color));
        }
    }
    return triangles;
}

pub fn calc_sphere(
    origin: Vector3d,
    radius: f64,
    resolution: usize,
    color: &Vector4d,
) -> Vec<Triangle3d> {
    let phis = linspace(0.0, 2.0 * PI, resolution);
    let mut thetas = linspace(0.0, PI, resolution);

    let mut vertices = vec![Vector3d::zero(); resolution * resolution];
    for phi_idx in 0..resolution {
        for theta_idx in 0..resolution {
            let cos_phi = phis[phi_idx].cos();
            let sin_phi = phis[phi_idx].sin();
            let cos_theta = thetas[theta_idx].cos();
            let sin_theta = thetas[theta_idx].sin();
            vertices[phi_idx * resolution + theta_idx] = Vector3d::new(
                radius * sin_theta * cos_phi,
                radius * sin_theta * sin_phi,
                radius * cos_theta,
            ) + origin;
        }
    }

    let mut triangles = vec![];
    for phi_idx in 0..resolution {
        for theta_idx in 0..resolution - 1 {
            if theta_idx == 0 {
                let (p1, p2, p3) = (
                    vertices[phi_idx * resolution + theta_idx],
                    vertices[phi_idx * resolution + theta_idx + 1],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx + 1],
                );
                triangles.push(Triangle3d::new(p1, p2, p3, color));
            } else {
                let (p1, p2, p3, p4) = (
                    vertices[phi_idx * resolution + theta_idx],
                    vertices[phi_idx * resolution + theta_idx + 1],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx + 1],
                );
                triangles.push(Triangle3d::new(p1, p2, p3, color));
                triangles.push(Triangle3d::new(p2, p4, p3, color));
            }
        }
    }

    // add bottom most tris
    let bottom_vertex = origin - Vector3d::new(0.0, 0.0, radius);
    for phi_idx in 0..resolution {
        let (p1, p2, p3) = (
            vertices[phi_idx * resolution + resolution - 1],
            vertices[((phi_idx + 1) % resolution) * resolution + resolution - 1],
            bottom_vertex,
        );
        triangles.push(Triangle3d::new(p1, p3, p2, color));
    }
    return triangles;
}
