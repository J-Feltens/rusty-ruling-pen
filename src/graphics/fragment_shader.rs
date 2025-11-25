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
    let ambient = 0.2;
    let diffuse_fac = 0.5;
    let specular_fac = 0.9;
    let shinyness = 100.0;

    let mut r_total = ambient;
    let mut g_total = ambient;
    let mut b_total = ambient;

    for light in lights {
        let l = (light.pos - x).normalize();

        // diffuse
        let n_dot_l = clamp(n.dot(l));
        let l_diff = light.strength * n_dot_l * diffuse_fac;
        r_total += l_diff * light.emission.r;
        g_total += l_diff * light.emission.g;
        b_total += l_diff * light.emission.b;

        // specular
        let r = n * n_dot_l * 2.0 - l;
        let l_spec = light.strength * v.dot(r).powf(shinyness) * specular_fac;

        r_total += l_spec * light.emission.r;
        g_total += l_spec * light.emission.g;
        b_total += l_spec * light.emission.b;
    }

    return color.apply_colored_lighting(r_total, g_total, b_total);
}
