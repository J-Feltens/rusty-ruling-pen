use crate::graphics::PointLight;
use crate::graphics::colors::apply_colored_lighting;
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
    let ambient = 0.01;
    let diffuse_fac = 0.7;
    let specular_fac = 1.0;
    let shinyness: i32 = 300;

    let mut lighting_total = Vector4d::ones() * ambient;

    for light in lights {
        let l = (light.pos - x).normalize();
        let n_dot_l = n.dot(l);
        if n_dot_l <= 0.0 {
            continue;
        }

        // diffuse
        let l_diff = light.strength * n_dot_l * diffuse_fac;
        lighting_total += light.emission * l_diff;

        // specular
        let r = n * n_dot_l * 2.0 - l;
        let v_dot_r = v.dot(r);
        let l_spec = light.strength * v_dot_r.powi(shinyness) * specular_fac;

        lighting_total += light.emission * l_spec;
    }

    return apply_colored_lighting(&color, &lighting_total);
}
