use crate::graphics::{Camera, Canvas, PointLight, Triangle3d};
use crate::util::linspace;
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Matrix3x3, Vector3d, Vector4d};
use std::f64::consts::PI;
use std::fs;

#[derive(Debug, Clone)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub lights: Vec<PointLight>,

    pub canvas: Canvas,

    pub render_smooth: bool,
    pub camera: Camera,
}

impl Scene {
    pub fn new(canvas: Canvas, camera: Camera, render_smooth: bool) -> Self {
        Self {
            meshes: vec![],
            lights: vec![],
            canvas,
            render_smooth,
            camera,
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn render_scene_to_buffer(&mut self) {
        // camera space stuff
        // let mut e = Vector3d::new(5.0, 5.0, 1.0) * 2.0; // cam pos

        let (g, u, v, w) = self.camera.calc_guvw();

        let camera_matrix = Matrix4x4::from_vecs(
            Vector4d::from_vector3d(&u, -u.dot(self.camera.e)),
            Vector4d::from_vector3d(&v, -v.dot(self.camera.e)),
            Vector4d::from_vector3d(&w, -w.dot(self.camera.e)),
            Vector4d::new(0.0, 0.0, 0.0, 1.0),
        );

        // transform lights to camera space

        let mut lights_cam_space_reallight = self.lights.clone();
        for light in lights_cam_space_reallight.iter_mut() {
            light.pos = camera_matrix
                .times_vec(Vector4d::from_vector3d(&light.pos, 1.0))
                .truncate_to_3d();
        }

        for mesh in self.meshes.clone() {
            for face in mesh.faces.iter() {
                let triangle = Triangle3d::new(
                    mesh.vertices[face[0]],
                    mesh.vertices[face[1]],
                    mesh.vertices[face[2]],
                    &mesh.color,
                );
                // println!("{}", triangle);

                // backface culling
                // Everlast - The Culling is Coming  =>   https://www.youtube.com/watch?v=yWYsbxkhlpU
                if w.dot(triangle.normal) < 0.0 {
                    continue;
                }

                let mut skip_triangle = false;
                let mut triangle_projected = vec![IntegerVector2d::zero(); 3];
                for (i, vertex) in triangle.vertices.iter().enumerate() {
                    let vertex_homo = Vector4d::from_vector3d(vertex, 1.0); // hehe

                    // transform to camera space
                    let vertex_cam_space = camera_matrix.times_vec(vertex_homo);

                    // perspective projection
                    let vertex_projected = self
                        .camera
                        .calc_perspective_projection_matrix()
                        .times_vec(vertex_cam_space);

                    // perspective divide by z
                    let vec3 = vertex_projected.truncate_to_3d() / vertex_projected.u;

                    if vec3.x < -1.0 || vec3.x > 1.0 || vec3.y < -1.0 || vec3.y > 1.0 {
                        skip_triangle = true;
                    }

                    // store attributes like pos and normal while still in camera space
                    let normal_cam_space;
                    if self.render_smooth {
                        normal_cam_space = camera_matrix
                            .times_vec(Vector4d::from_vector3d(&mesh.vertex_normals[face[i]], 0.0));
                    } else {
                        normal_cam_space =
                            camera_matrix.times_vec(Vector4d::from_vector3d(&triangle.normal, 0.0));
                    }
                    let mut attrs: Vec<f64> = vec![0.0; 11];
                    attrs[0] = vertex_cam_space.x;
                    attrs[1] = vertex_cam_space.y;
                    attrs[2] = vertex_cam_space.z;
                    attrs[3] = vertex_projected.z;
                    attrs[4] = normal_cam_space.x;
                    attrs[5] = normal_cam_space.y;
                    attrs[6] = normal_cam_space.z;
                    attrs[7] = triangle.color.x;
                    attrs[8] = triangle.color.y;
                    attrs[9] = triangle.color.z;
                    attrs[10] = triangle.color.u;

                    let ivec2 = IntegerVector2d::new(
                        (vec3.x * self.canvas.size_x_supersized_half as f64) as i32
                            + self.canvas.size_x_supersized_half as i32,
                        (vec3.y * self.canvas.size_y_supersized_half as f64) as i32
                            + self.canvas.size_y_supersized_half as i32,
                        attrs,
                    );
                    triangle_projected[i] = ivec2;
                }

                // cull triangles that is even partially out if bounds
                if skip_triangle {
                    continue;
                }
                self.canvas
                    .draw_polygon_onto_buffer(&triangle_projected, &lights_cam_space_reallight);
            }
        }

        self.canvas.apply_ssaa();
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3d>,
    pub faces: Vec<Vec<usize>>,
    pub color: Vector4d,

    pub vertex_merge_radius: f64,
    pub vertex_normals: Vec<Vector3d>,
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
            vertex_normals: Vec::new(),
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

    pub fn recalc_vertex_normals(&mut self) {
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
        self.vertex_normals = normals;
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

    let mut mesh = Mesh::new(
        vec![v1, v2, v3, v4, v5, v6, v7, v8],
        vec![e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12],
        color,
        0.01,
    );
    mesh.recalc_vertex_normals();
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

    mesh.recalc_vertex_normals();
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

    mesh.recalc_vertex_normals();
    return mesh;
}

pub fn calc_teapot(color: Vector4d, resolution: usize, booleaned: bool) -> Mesh {
    /*
       expects resolution to be either 1, 2 or 3
    */

    let file_path = format!(
        "src/graphics/{}.txt",
        if booleaned {
            if resolution == 1 {
                "utah_teapot_booleaned_5144"
            } else if resolution == 2 {
                "utah_teapot_booleaned_22885"
            } else if resolution == 3 {
                "utah_teapot_booleaned_158865"
            } else {
                panic!("Have you bot enough vertices already??")
            }
        } else {
            if resolution == 1 {
                "utah_teapot_3488"
            } else if resolution == 2 {
                "utah_teapot_19480"
            } else if resolution == 3 {
                "utah_teapot_145620"
            } else {
                panic!("Have you bot enough vertices already??")
            }
        }
    );
    let mut mesh = Mesh::init(color, 0.0);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let faces: Vec<&str> = contents.split("\n\n").collect();
    let rot_mat = Matrix3x3::calc_rotation_matrix(Vector3d::new(1.0, 0.0, 0.0), PI / 2.0);
    for face in faces {
        let vertices: Vec<&str> = face.split("\n").collect();
        let v1_string: Vec<&str> = vertices[0].split(" ").collect();
        let v2_string: Vec<&str> = vertices[2].split(" ").collect();
        let v3_string: Vec<&str> = vertices[4].split(" ").collect();
        let v1 = Vector3d::new(
            v1_string[0].parse::<f64>().unwrap(),
            v1_string[1].parse::<f64>().unwrap() - 1.0,
            v1_string[2].parse::<f64>().unwrap(),
        );
        let v2 = Vector3d::new(
            v2_string[0].parse::<f64>().unwrap(),
            v2_string[1].parse::<f64>().unwrap() - 1.0,
            v2_string[2].parse::<f64>().unwrap(),
        );
        let v3 = Vector3d::new(
            v3_string[0].parse::<f64>().unwrap(),
            v3_string[1].parse::<f64>().unwrap() - 1.0,
            v3_string[2].parse::<f64>().unwrap(),
        );

        mesh.add_face(
            rot_mat.times_vec(v1),
            rot_mat.times_vec(v2),
            rot_mat.times_vec(v3),
        );
    }
    mesh.recalc_vertex_normals();
    return mesh;
}
