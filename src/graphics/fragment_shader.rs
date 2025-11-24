use crate::graphics::Color;
use crate::vectors::Vector3d;

pub fn phong_frag(x: Vector3d, n: Vector3d, l: Vector3d, v: Vector3d, color: Color) -> Color {
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
    let mut l_diff = n.dot(l);

    // specular
    let r = n * 2.0 * l_diff - l;

    return color.apply_lighting(l_diff);
}
