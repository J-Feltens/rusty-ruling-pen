use crate::graphics::{Color, PointLight};
use crate::util::clamp;
use crate::vectors::Vector3d;

pub fn phong_frag(
    x: Vector3d,
    n: Vector3d,
    v: Vector3d,
    color: Color,
    lights: &Vec<PointLight>,
) -> Color {
    /*
        with
            x:          surface position in camera space
            v:          look at (pointing towards cam)
            l:          lighting vector
            n:          normal
            color:      color
    */

    // Phong
    // diffuse
    let mut l_total = 0.0;
    for light in lights {
        let l = (light.pos - x).normalize();
        let l_diff = light.emission * n.dot(l);
        l_total += l_diff;
    }

    l_total = clamp(l_total);
    return color.apply_lighting(l_total);
}
