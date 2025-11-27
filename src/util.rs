use crate::vectors::Vector4d;
use crate::vectors::matrices::Matrix4x4;

pub fn interpolate1d(mut start: f64, mut end: f64, factor: f64) -> f64 {
    assert!(factor >= 0.0 && factor <= 1.0, "Cannot clamp yet");

    if start > end {
        (end, start) = (start, end);
    }

    let delta = end - start;
    return start + factor * delta;
}

pub fn calc_perspective_matrix(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) -> Matrix4x4 {
    return Matrix4x4::from_vecs(
        Vector4d::new((2.0 * n) / (r - l), 0.0, (l + r) / (r - l), 0.0),
        Vector4d::new(0.0, (2.0 * n) / (t - b), (b + t) / (t - b), 0.0),
        Vector4d::new(0.0, 0.0, -(n) / (f - n), -(f * n) / (f - n)),
        Vector4d::new(0.0, 0.0, -(n) / (f - n), -(f * n) / (f - n)),
    );
}

pub fn calc_ortho_matrix(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) -> Matrix4x4 {
    return Matrix4x4::from_vecs(
        Vector4d::new(2.0 / (r - l), 0.0, 0.0, -(r + l) / (r - l)),
        Vector4d::new(0.0, 2.0 / (t - b), 0.0, -(t + b) / (t - b)),
        Vector4d::new(0.0, 0.0, 1.0 / (n - f), -n / (f - n)),
        Vector4d::new(0.0, 0.0, 0.0, 1.0),
    );
}

pub fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    /*
    [start, end)
    Including start, excluding end
     */
    assert!(start <= end);
    let mut ret = vec![0.0; n];
    let delta = (end - start) / n as f64;
    for i in 0..n {
        ret[i] = start + i as f64 * delta;
    }

    return ret;
}

pub fn clamp(val: f64) -> f64 {
    /*
    clamps value into range [0, 1]
    */
    if val < 0.0 {
        0.0
    } else if val > 1.0 {
        1.0
    } else {
        val
    }
}
