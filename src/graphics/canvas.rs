use crate::graphics::colors::{color_vec_from_f64, color_vec_from_u32, color_vec_to_u32};
use crate::graphics::fragment_shader::phong_frag;
use crate::graphics::scanline::{ActiveEdgeTable, ActiveEdgeTableEntry, EdgeTable, EdgeTableEntry};
use crate::graphics::shapes::{Mesh, Scene};
use crate::graphics::{Camera, PointLight, Triangle3d, alpha_blend};
use crate::vectors::matrices::Matrix4x4;
use crate::vectors::{IntegerVector2d, Vector3d, Vector4d};
use core::f64;
use std::fmt;

#[derive(Clone, Debug)]
pub enum SSAA {
    X0_125,
    X0_25,
    X1,
    X4,
    X16,
    X64,
}

impl fmt::Display for SSAA {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SSAA::X0_125 => write!(f, "0.125X SSAA (Upscaling mode)"),
            SSAA::X0_25 => write!(f, "0.25X SSAA (Upscalign mode)"),
            SSAA::X1 => write!(f, "1X SSAA"),
            SSAA::X4 => write!(f, "4X SSAA"),
            SSAA::X16 => write!(f, "16X SSAA"),
            SSAA::X64 => write!(f, "64X SSAA"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Canvas {
    pub size_x: usize,
    pub size_y: usize,

    pub buffer: Vec<u32>,
    pub buffer_supersized: Vec<u32>,
    pub z_buffer_supersized: Vec<f64>,
    pub bg_color: Vector4d,
    pub lights: Vec<PointLight>,

    // for super-sample-anti-aliasing
    // usually, names containing "supersized" refer to this context
    pub ssaa: SSAA,
    pub ssaa_fac: f64,
    pub size_x_supersized: usize,
    pub size_y_supersized: usize,
    pub size_x_supersized_half: usize,
    pub size_y_supersized_half: usize,
}

impl Canvas {
    pub fn new(size_x: usize, size_y: usize, bg_color: Vector4d, ssaa: SSAA) -> Canvas {
        let (
            ssaa_fac,
            size_x_supersized,
            size_y_supersized,
            size_x_supersized_half,
            size_y_supersized_half,
            buffer_supersized,
            z_buffer_supersized,
        ) = Self::calc_ssaa_variables(&ssaa, size_x, size_y, &bg_color);
        Canvas {
            size_x,
            size_y,

            buffer: vec![color_vec_to_u32(&bg_color); size_x * size_y],
            bg_color,
            lights: vec![],

            ssaa,
            ssaa_fac,
            size_x_supersized,
            size_y_supersized,
            size_x_supersized_half,
            size_y_supersized_half,

            z_buffer_supersized,
            buffer_supersized,
        }
    }

    pub fn calc_ssaa_variables(
        ssaa: &SSAA,
        size_x: usize,
        size_y: usize,
        bg_color: &Vector4d,
    ) -> (f64, usize, usize, usize, usize, Vec<u32>, Vec<f64>) {
        let ssaa_fac;
        match ssaa {
            SSAA::X0_125 => ssaa_fac = 0.25,
            SSAA::X0_25 => ssaa_fac = 0.5,
            SSAA::X1 => ssaa_fac = 1.0,
            SSAA::X4 => ssaa_fac = 2.0,
            SSAA::X16 => ssaa_fac = 4.0,
            SSAA::X64 => ssaa_fac = 8.0,
        }

        // check if upscaling is possible
        if ssaa_fac == 0.25 {
            if size_x % 2 != 0 || size_y % 2 != 0 {
                panic!(
                    "Canvas size must be divisible by 2 for 0.25 upscaling mode, found {}, {}",
                    size_x, size_y
                )
            }
        } else if ssaa_fac == 0.125 {
            if size_x % 4 != 0 || size_y % 4 != 0 {
                panic!(
                    "Canvas size must be divisible by 4 for 0.125 upscaling mode, found {}, {}",
                    size_x, size_y
                )
            }
        }

        let size_x_supersized = size_x as f64 * ssaa_fac;
        let size_y_supersized = size_y as f64 * ssaa_fac;

        let size_x_supersized_half = (size_x_supersized / 2.0) as usize;
        let size_y_supersized_half = (size_y_supersized / 2.0) as usize;

        let z_buffer_supersized = vec![f64::MAX; (size_x_supersized * size_y_supersized) as usize];
        let buffer_supersized = vec![
            crate::graphics::colors::color_vec_to_u32(bg_color);
            (size_x_supersized * size_y_supersized) as usize
        ];
        return (
            ssaa_fac,
            size_x_supersized as usize,
            size_y_supersized as usize,
            size_x_supersized_half,
            size_y_supersized_half,
            buffer_supersized,
            z_buffer_supersized,
        );
    }

    pub fn set_ssaa(&mut self, ssaa: SSAA) {
        let (
            ssaa_fac,
            size_x_supersized,
            size_y_supersized,
            size_x_supersized_half,
            size_y_supersized_half,
            buffer_supersized,
            z_buffer_supersized,
        ) = Self::calc_ssaa_variables(&ssaa, self.size_x, self.size_y, &self.bg_color);
        self.ssaa = ssaa;
        self.ssaa_fac = ssaa_fac;
        self.size_x_supersized = size_x_supersized;
        self.size_y_supersized = size_y_supersized;
        self.size_x_supersized_half = size_x_supersized_half;
        self.size_y_supersized_half = size_y_supersized_half;
        self.buffer_supersized = buffer_supersized;
        self.z_buffer_supersized = z_buffer_supersized;
    }

    pub fn increase_ssaa(&mut self) {
        match self.ssaa {
            SSAA::X0_125 => self.set_ssaa(SSAA::X0_25),
            SSAA::X0_25 => self.set_ssaa(SSAA::X1),
            SSAA::X1 => self.set_ssaa(SSAA::X4),
            SSAA::X4 => self.set_ssaa(SSAA::X16),
            SSAA::X16 => self.set_ssaa(SSAA::X64),
            SSAA::X64 => return,
        }
    }

    pub fn decrease_ssaa(&mut self) {
        match self.ssaa {
            SSAA::X0_125 => return,
            SSAA::X0_25 => self.set_ssaa(SSAA::X0_125),
            SSAA::X1 => self.set_ssaa(SSAA::X0_25),
            SSAA::X4 => self.set_ssaa(SSAA::X1),
            SSAA::X16 => self.set_ssaa(SSAA::X4),
            SSAA::X64 => self.set_ssaa(SSAA::X16),
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(color_vec_to_u32(&self.bg_color));
        self.buffer_supersized
            .fill(color_vec_to_u32(&self.bg_color));
    }

    pub fn reset_z_buffer(&mut self) {
        self.z_buffer_supersized.fill(f64::MAX);
    }

    pub fn integer_coords_in_canvas(&self, x: i32, y: i32) -> bool {
        return x >= 0
            && (x as usize) < self.size_y_supersized
            && y >= 0
            && (y as usize) < self.size_y_supersized;
    }

    pub fn set_pixel(&mut self, coords: (i32, i32), color: &Vector4d) {
        // only draw pixel if it is in buffer bounds, will pass silently
        if self.integer_coords_in_canvas(coords.0, coords.1) {
            let integer_coord_in_buffer = ((self.size_y_supersized as i32 - 1 - coords.1)
                * self.size_x_supersized as i32
                + coords.0) as usize;

            let color_from = &color_vec_from_u32(self.buffer_supersized[integer_coord_in_buffer]);

            // alpha-blend
            self.buffer_supersized[integer_coord_in_buffer] =
                color_vec_to_u32(&alpha_blend(color_from, &color));
        } else {
            println!("Drawing outside of canvas!");
        }
    }

    pub fn set_pixel_with_z(&mut self, coords: (i32, i32), z: f64, color: &Vector4d) {
        // only draw pixel if it is in buffer bounds, will pass silently
        if self.integer_coords_in_canvas(coords.0, coords.1) {
            let integer_coord_in_buffer = ((self.size_x_supersized as i32 - 1 - coords.1)
                * self.size_x_supersized as i32
                + coords.0) as usize;

            if z < self.z_buffer_supersized[integer_coord_in_buffer] {
                self.set_pixel(coords, color);
                self.z_buffer_supersized[integer_coord_in_buffer] = z;
            }
        }
    }

    pub fn add_layer(&mut self, layer: Canvas, pos_x: u32, pos_y: u32) {
        if pos_x + layer.size_x as u32 >= self.size_x as u32
            || pos_y + layer.size_y as u32 >= self.size_y as u32
        {
            println!("Layer too large for canvas!");
        }

        for y in 0..layer.size_y {
            for x in 0..layer.size_x {
                self.buffer[(pos_y as usize + y) * self.size_x + pos_x as usize + x] =
                    layer.buffer[y * layer.size_x + x];
            }
        }
    }

    pub fn apply_ssaa(&mut self) {
        if self.ssaa_fac >= 1.0 {
            // SSAA in antialiasing mode
            for y in 0..self.size_y {
                for x in 0..self.size_x {
                    let mut mixed = Vector4d::zeros();
                    for y_ in 0..self.ssaa_fac as usize {
                        for x_ in 0..self.ssaa_fac as usize {
                            mixed += color_vec_from_u32(
                                self.buffer_supersized[(self.ssaa_fac as usize * y + y_)
                                    * self.size_x_supersized
                                    + (self.ssaa_fac as usize * x + x_)],
                            );
                        }
                    }
                    mixed /= self.ssaa_fac as f64 * self.ssaa_fac as f64;
                    self.buffer[y * self.size_x + x] = color_vec_to_u32(&mixed);
                }
            }
        } else {
            // SSAA in upscaling mode
            if self.ssaa_fac == 0.5 {
                for y_ in 0..self.size_y_supersized {
                    for x_ in 0..self.size_x_supersized {
                        self.buffer[y_ * 2 * self.size_x + x_ * 2] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 2 + 1) * self.size_x + x_ * 2] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[y_ * 2 * self.size_x + (x_ * 2 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 2 + 1) * self.size_x + (x_ * 2 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                    }
                }
            } else if self.ssaa_fac == 0.25 {
                for y_ in 0..self.size_y_supersized {
                    for x_ in 0..self.size_x_supersized {
                        self.buffer[(y_ * 4) * self.size_x + (x_ * 4)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 1) * self.size_x + (x_ * 4)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 2) * self.size_x + (x_ * 4)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 3) * self.size_x + (x_ * 4)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];

                        self.buffer[(y_ * 4) * self.size_x + (x_ * 4 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 1) * self.size_x + (x_ * 4 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 2) * self.size_x + (x_ * 4 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 3) * self.size_x + (x_ * 4 + 1)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];

                        self.buffer[(y_ * 4) * self.size_x + (x_ * 4 + 2)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 1) * self.size_x + (x_ * 4 + 2)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 2) * self.size_x + (x_ * 4 + 2)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 3) * self.size_x + (x_ * 4 + 2)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];

                        self.buffer[(y_ * 4) * self.size_x + (x_ * 4 + 3)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 1) * self.size_x + (x_ * 4 + 3)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 2) * self.size_x + (x_ * 4 + 3)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                        self.buffer[(y_ * 4 + 3) * self.size_x + (x_ * 4 + 3)] =
                            self.buffer_supersized[y_ * self.size_x_supersized + x_];
                    }
                }
            }
        }
    }

    pub fn draw_polygon_onto_buffer(
        &mut self,
        points: &Vec<IntegerVector2d>,
        light_cam_space_reallight: &Vec<PointLight>,
    ) {
        /*
            implements scanline algorithm with some extended features/bugs courtesy of yours truly.

            as usual based on Marc Stammingers lecture slides:

            initialize ET
            set AET to empty
            set yscan to ylower of first entry in ET
                move all edges from ET with yscan =| ylower to AET

            while ET not empty or AET not empty
                sort AET for x
                draw lines from (AET[0].x,yscan) to (AET[1].x,yscan),
                    from (AET[2].x,yscan) to (AET[3].x,yscan), ……
                remove all edges from AET with yscan >= yupper
                for all edges in AET
                    x:= x + 1/m
                yscan += 1
                move all edges from ET with yscan == ylower to AET
        */

        if points.len() <= 2 {
            return; // a line ain't enough for a _poly_gon
        }

        // total culling of all polygons that are ever so slightly out of bounds.
        // will need major revamp to compute partial out-of-bounds polygons, something along the lines of
        // intersection with boundary vectors, linear interpolationd
        for i in 0..points.len() {
            if !self.integer_coords_in_canvas(points[i].x, points[i].y) {
                return;
            }
        }

        // build edge table, ignore horizontal edges
        let mut edge_table = EdgeTable::new();

        for i in 0..points.len() {
            let p1 = &points[i % points.len()];
            let p2 = &points[(i + 1) % points.len()];
            if p1.y != p2.y {
                edge_table.add_edge(EdgeTableEntry::from_points(
                    p1.clone(),
                    p2.clone(),
                    i as i32 + 1,
                ));
            }
        }
        if edge_table.list.len() <= 0 {
            return;
        }
        edge_table.sort();

        let mut active_edge_table = ActiveEdgeTable::new();

        let mut y_scan = edge_table.list[0].y_lower;

        while let Some(index) = edge_table
            .list
            .iter()
            .position(|edge| edge.y_lower == y_scan)
        {
            let edge = edge_table.list.remove(index);

            // compute values for active edge entry
            let x_intersect = edge.x_lower as f64;
            let attrs_intersect = edge.attrs_lower;

            active_edge_table.list.push(ActiveEdgeTableEntry::new(
                x_intersect,
                edge.y_upper,
                edge.dx_dy,
                edge.id,
                attrs_intersect,
                edge.dattrs_dy,
            ));

            active_edge_table.sort();
        }

        let mut iteration = 0;
        while edge_table.list.len() > 0 || active_edge_table.list.len() > 0 {
            // remove all edges from AET wich are entirely below y_scan
            while let Some(index) = active_edge_table
                .list
                .iter()
                .position(|edge| y_scan >= edge.y_upper)
            {
                active_edge_table.list.remove(index);
            }

            // add all edges from ET with y_lower == y_scan to AET
            while let Some(index) = edge_table
                .list
                .iter()
                .position(|edge| edge.y_lower as i32 == y_scan as i32)
            {
                let edge = edge_table.list.remove(index);

                // compute values for active edge entry
                let x_intersect = edge.x_lower as f64;
                let attr_intersect = edge.attrs_lower;

                active_edge_table.list.push(ActiveEdgeTableEntry::new(
                    x_intersect,
                    edge.y_upper,
                    edge.dx_dy,
                    edge.id,
                    attr_intersect,
                    edge.dattrs_dy,
                ));

                active_edge_table.sort();
            }

            edge_table.sort();
            active_edge_table.sort();

            // draw between x_1_intersect and x_2_intersect
            if active_edge_table.list.len() >= 2 {
                for i in 0..(active_edge_table.list.len() as f64 / 2.0) as usize {
                    let edge1 = &active_edge_table.list[2 * i];
                    let edge2 = &active_edge_table.list[2 * i + 1];

                    let mut cur_x = edge1.x_intersect;
                    let mut cur_attrs = edge1.attrs_intersect.clone();
                    let mut dattrs = vec![0.0 as f64; cur_attrs.len()];
                    for i in 0..dattrs.len() {
                        dattrs[i] =
                            (edge2.attrs_intersect[i] - cur_attrs[i]) / (edge2.x_intersect - cur_x);
                    }

                    while cur_x <= edge2.x_intersect {
                        // call fragment shader
                        let z_projected = cur_attrs[3];
                        let color = color_vec_from_f64(
                            cur_attrs[7],
                            cur_attrs[8],
                            cur_attrs[9],
                            cur_attrs[10],
                        );
                        let x = Vector3d::new(cur_attrs[0], cur_attrs[1], cur_attrs[2]);
                        let n = Vector3d::new(cur_attrs[4], cur_attrs[5], cur_attrs[6]).normalize();
                        // let l = (light_cam_space - x).normalize();
                        let v = (x * -1.0).normalize();
                        let phong_color = phong_frag(x, n, v, color, &light_cam_space_reallight);

                        self.set_pixel_with_z(
                            (cur_x.round() as i32, y_scan),
                            z_projected,
                            &phong_color,
                        );
                        cur_x += 1.0;
                        for i in 0..cur_attrs.len() {
                            cur_attrs[i] += dattrs[i];
                        }
                    }
                }
            }

            // increment x_intersect and attributes in every edge in AET
            for edge in active_edge_table.list.iter_mut() {
                edge.x_intersect += edge.dx_dy;
                for i in 0..edge.attrs_intersect.len() {
                    edge.attrs_intersect[i] += edge.dattrs_dy[i];
                }
            }

            // increment y_scan
            y_scan += 1;

            iteration += 1;
        }
    }
}
