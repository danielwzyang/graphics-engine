type Vector = [f32; 3];

use crate::constants::SPECULAR_EXPONENT;

#[derive(Clone, Copy)]
pub struct LightingConfig {
    pub ambient_light_color: [f32; 3],
    pub point_light_color: [f32; 3],
    pub point_light_vector: [f32; 3],
    pub ambient_reflection: [f32; 3],
    pub diffuse_reflection: [f32; 3],
    pub specular_reflection: [f32; 3],
    // note: viewer vector is always <0, 0, 1> so all the math for backface culling and lighting is hardcoded
}

pub fn get_illumination(normal: &Vector, config: &LightingConfig) -> (usize, usize, usize) {
    let normal = &normalize_vector(&normal);
    let point_light_vector = normalize_vector(&config.point_light_vector);

    // i_ambient = ambient color * ambient reflection constant
    let ambient = [
        config.ambient_light_color[0] * config.ambient_reflection[0],
        config.ambient_light_color[1] * config.ambient_reflection[1],
        config.ambient_light_color[2] * config.ambient_reflection[2],
    ];

    // i_diffuse = point color * diffuse reflection constant * (normalized normal dot normalized light)
    let n_dot_l = f32::max(0.0, dot_product(&normal, &point_light_vector));
    let diffuse = [
        config.point_light_color[0] * config.diffuse_reflection[0] * n_dot_l,
        config.point_light_color[1] * config.diffuse_reflection[1] * n_dot_l,
        config.point_light_color[2] * config.diffuse_reflection[2] * n_dot_l,
    ];

    // i_specular = point color * specular reflection constant * (normalized reflection dot view)^exp
    // where exp > 1
    // normalized reflection = [2 * normalized normal * (normalized normal dot normalized light) - normalized light]

    // for normalized reflection dot view:
    // since view just <0, 0, 1>, we can be lazy and take the z value, raise it to exp, and call it r_z
    let r_z = f32::max(0.0, 2.0 * normal[2] * n_dot_l - point_light_vector[2]).powf(SPECULAR_EXPONENT);

    let specular = [
        config.point_light_color[0] * config.specular_reflection[0] * r_z,
        config.point_light_color[1] * config.specular_reflection[1] * r_z,
        config.point_light_color[2] * config.specular_reflection[2] * r_z,
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