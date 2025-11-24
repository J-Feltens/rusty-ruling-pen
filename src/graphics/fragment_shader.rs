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

    let l_diff = n.dot(l);
    return color.apply_lighting(l_diff);
}
