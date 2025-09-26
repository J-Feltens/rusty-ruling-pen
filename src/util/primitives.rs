use std::vec;

use crate::{
    colors::Color,
    util::{Line3d, Vector3d},
};

pub fn cube(a: f64, origin: Vector3d, color: &Color) -> Vec<Line3d> {
    let a_half: f64 = a / 2.0;

    let v1 = origin.clone() + Vector3d::new(-a_half, a_half, a_half);
    let v2 = origin.clone() + Vector3d::new(-a_half, -a_half, a_half);
    let v3 = origin.clone() + Vector3d::new(-a_half, -a_half, -a_half);
    let v4 = origin.clone() + Vector3d::new(-a_half, a_half, -a_half);

    let v5 = origin.clone() + Vector3d::new(a_half, a_half, a_half);
    let v6 = origin.clone() + Vector3d::new(a_half, -a_half, a_half);
    let v7 = origin.clone() + Vector3d::new(a_half, -a_half, -a_half);
    let v8 = origin.clone() + Vector3d::new(a_half, a_half, -a_half);

    return vec![
        Line3d::new(v1, v2, color),
        Line3d::new(v2, v3, color),
        Line3d::new(v3, v4, color),
        Line3d::new(v4, v1, color),
        //
        Line3d::new(v5, v6, color),
        Line3d::new(v6, v7, color),
        Line3d::new(v7, v8, color),
        Line3d::new(v8, v5, color),
        //
        Line3d::new(v1, v5, color),
        Line3d::new(v2, v6, color),
        Line3d::new(v3, v7, color),
        Line3d::new(v4, v8, color),
    ];
}
