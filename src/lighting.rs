type Vector = [f32; 3];

use crate::constants::{AMBIENT_LIGHT_COLOR, AMBIENT_REFLECTION, DIFFUSE_REFLECTION, POINT_LIGHT_COLOR, POINT_LIGHT_VECTOR, SPECULAR_EXPONENT, SPECULAR_REFLECTION};

pub fn get_illumination(normal: &Vector) -> (usize, usize, usize) {
    let point_light_vector = normalize_vector(&POINT_LIGHT_VECTOR);

    // i_ambient = ambient color * ambient reflection constant
    let ambient = [
        AMBIENT_LIGHT_COLOR[0] * AMBIENT_REFLECTION[0],
        AMBIENT_LIGHT_COLOR[1] * AMBIENT_REFLECTION[1],
        AMBIENT_LIGHT_COLOR[2] * AMBIENT_REFLECTION[2],
    ];

    // i_diffuse = point color * diffuse reflection constant * (normalized normal dot normalized light)
    let n_dot_l = f32::max(0.0, dot_product(&normal, &point_light_vector));
    let diffuse = [
        POINT_LIGHT_COLOR[0] * DIFFUSE_REFLECTION[0] * n_dot_l,
        POINT_LIGHT_COLOR[1] * DIFFUSE_REFLECTION[1] * n_dot_l,
        POINT_LIGHT_COLOR[2] * DIFFUSE_REFLECTION[2] * n_dot_l,
    ];

    // i_specular = point color * specular reflection constant * (normalized reflection dot view)^exp
    // where exp > 1
    // normalized reflection = [2 * normalized normal * (normalized normal dot normalized light) - normalized light]

    // for normalized reflection dot view:
    // since view just <0, 0, 1>, we can be lazy and take the z value, raise it to exp, and call it r_z
    let r_z = f32::max(0.0, 2.0 * normal[2] * n_dot_l - point_light_vector[2]).powf(SPECULAR_EXPONENT);

    let specular = [
        POINT_LIGHT_COLOR[0] * SPECULAR_REFLECTION[0] * r_z,
        POINT_LIGHT_COLOR[1] * SPECULAR_REFLECTION[1] * r_z,
        POINT_LIGHT_COLOR[2] * SPECULAR_REFLECTION[2] * r_z,
    ];

    clamp_color([
        ambient[0] + diffuse[0] + specular[0],
        ambient[1] + diffuse[1] + specular[1],
        ambient[2] + diffuse[2] + specular[2],
    ])
}

fn dot_product(a: &Vector, b: &Vector) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize_vector(vector: &Vector) -> Vector {
    let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    [vector[0] / magnitude, vector[1] / magnitude, vector[2] / magnitude]
}

fn clamp_color(vector: Vector) -> (usize, usize, usize) {
    (
        vector[0].clamp(0.0, 255.0) as usize,
        vector[1].clamp(0.0, 255.0) as usize,
        vector[2].clamp(0.0, 255.0) as usize,
    )
}