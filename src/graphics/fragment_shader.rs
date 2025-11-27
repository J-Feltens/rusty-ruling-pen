use crate::graphics::PointLight;
use crate::graphics::colors::apply_colored_lighting;
use crate::util::clamp;
use crate::vectors::{Vector3d, Vector4d};

pub fn phong_frag(
    x: Vector3d,
    n: Vector3d,
    v: Vector3d,
    color: Vector4d,
    lights: &Vec<PointLight>,
) -> Vector4d {
    /*
        with
            x:          surface position in camera space
            v:          look at (pointing towards cam)
            l:          lighting vector
            n:          normal
            color:      color
    */

    // Phong
    let ambient = 0.05;
    let diffuse_fac = 0.7;
    let specular_fac = 0.9;
    let shinyness = 100.0;

    let mut lighting_total = Vector4d::zero();

    for light in lights {
        let l = (light.pos - x).normalize();

        // diffuse
        let n_dot_l = clamp(n.dot(l));
        let l_diff = light.strength * n_dot_l * diffuse_fac;
        lighting_total += (light.emission * l_diff);

        // specular
        let r = n * n_dot_l * 2.0 - l;
        let l_spec = light.strength * v.dot(r).powf(shinyness) * specular_fac;

        lighting_total += (light.emission * l_spec);
    }

    return apply_colored_lighting(&color, &lighting_total);
}
