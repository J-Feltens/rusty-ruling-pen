use crate::util::linspace;
use crate::vectors::{Matrix3x3, Vector3d, Vector4d};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self { meshes: vec![] }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3d>,
    pub faces: Vec<Vec<usize>>,
    pub color: Vector4d,

    pub vertex_merge_radius: f64,
}

impl Mesh {
    pub fn init(color: Vector4d, vertex_merge_radius: f64) -> Self {
        Self::new(vec![], vec![], color, vertex_merge_radius)
    }
    pub fn new(
        vertices: Vec<Vector3d>,
        faces: Vec<Vec<usize>>,
        color: Vector4d,
        vertex_merge_radius: f64,
    ) -> Self {
        Self {
            vertices,
            faces,
            color,
            vertex_merge_radius,
        }
    }

    pub fn add_face(&mut self, v1: Vector3d, v2: Vector3d, v3: Vector3d) {
        let mut new_face = vec![0; 3];
        let mut found_v1 = false;
        let mut found_v2 = false;
        let mut found_v3 = false;
        for i in 0..self.vertices.len() {
            if self.vertices[i].equals_roughly(&v1, self.vertex_merge_radius) {
                new_face[0] = i;
                found_v1 = true;
            }
            if self.vertices[i].equals_roughly(&v2, self.vertex_merge_radius) {
                new_face[1] = i;
                found_v2 = true;
            }
            if self.vertices[i].equals_roughly(&v3, self.vertex_merge_radius) {
                new_face[2] = i;
                found_v3 = true;
            }
        }
        if !found_v1 {
            self.vertices.push(v1);
            new_face[0] = self.vertices.len() - 1;
        }
        if !found_v2 {
            self.vertices.push(v2);
            new_face[1] = self.vertices.len() - 1;
        }
        if !found_v3 {
            self.vertices.push(v3);
            new_face[2] = self.vertices.len() - 1;
        }
        self.faces.push(new_face);
    }

    pub fn interpolate_vertex_normals(&self) -> Vec<Vector3d> {
        let mut normals = vec![Vector3d::zero(); self.vertices.len()];

        for target_vertex in 0..self.vertices.len() {
            let mut new_normal = Vector3d::zero();
            for face in &self.faces {
                if face.contains(&target_vertex) {
                    // face contains target vertex, calculate face normal and add to new normal
                    new_normal += (self.vertices[face[1]] - self.vertices[face[0]])
                        .cross(self.vertices[face[2]] - self.vertices[face[0]])
                        .normalize();
                }
            }
            normals[target_vertex] = new_normal.normalize();
        }
        return normals;
    }
}

pub fn calc_cube(cube_size: f64, center: Vector3d, color: Vector4d) -> Mesh {
    // vertices
    let v1 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0) + center;
    let v2 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, -cube_size / 2.0) + center;
    let v3 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0) + center;
    let v4 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, -cube_size / 2.0) + center;

    let v5 = Vector3d::new(-cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0) + center;
    let v6 = Vector3d::new(cube_size / 2.0, -cube_size / 2.0, cube_size / 2.0) + center;
    let v7 = Vector3d::new(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0) + center;
    let v8 = Vector3d::new(-cube_size / 2.0, cube_size / 2.0, cube_size / 2.0) + center;

    let e1 = vec![2, 1, 0];
    let e2 = vec![3, 2, 0];

    let e3 = vec![4, 5, 6];
    let e4 = vec![4, 6, 7];

    let e5 = vec![0, 1, 5];
    let e6 = vec![5, 4, 0];
    let e7 = vec![1, 2, 6];
    let e8 = vec![6, 5, 1];
    let e9 = vec![2, 3, 7];
    let e10 = vec![7, 6, 2];
    let e11 = vec![3, 0, 4];
    let e12 = vec![4, 7, 3];

    let mesh = Mesh::new(
        vec![v1, v2, v3, v4, v5, v6, v7, v8],
        vec![e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12],
        color,
        0.01,
    );

    return mesh;
}
//
pub fn calc_torus(
    origin: Vector3d,
    major_radius: f64,
    minor_radius: f64,
    major_resolution: usize,
    minor_resolution: usize,
    color: &Vector4d,
) -> Mesh {
    let phis = linspace(0.0, 2.0 * PI, major_resolution);
    let thetas = linspace(0.0, 2.0 * PI, minor_resolution);

    let mut mesh = Mesh::init(color.clone(), 0.00001);

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
            mesh.add_face(p3, p2, p1);
            mesh.add_face(p2, p3, p4);
            // triangles.push(Triangle3d::new(p3, p2, p1, &color));
            // triangles.push(Triangle3d::new(p2, p3, p4, &color));
        }
    }
    return mesh;
}

pub fn calc_sphere(origin: Vector3d, radius: f64, resolution: usize, color: &Vector4d) -> Mesh {
    let phis = linspace(0.0, 2.0 * PI, resolution);
    let thetas = linspace(0.0, PI, resolution);

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

    let mut mesh = Mesh::init(color.clone(), 0.00001);
    for phi_idx in 0..resolution {
        for theta_idx in 0..resolution - 1 {
            if theta_idx == 0 {
                let (p1, p2, p3) = (
                    vertices[phi_idx * resolution + theta_idx],
                    vertices[phi_idx * resolution + theta_idx + 1],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx + 1],
                );
                mesh.add_face(p1, p2, p3);
            } else {
                let (p1, p2, p3, p4) = (
                    vertices[phi_idx * resolution + theta_idx],
                    vertices[phi_idx * resolution + theta_idx + 1],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx],
                    vertices[((phi_idx + 1) % resolution) * resolution + theta_idx + 1],
                );
                mesh.add_face(p1, p2, p3);
                mesh.add_face(p2, p4, p3);
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
        mesh.add_face(p1, p3, p2);
    }
    return mesh;
}
