type Vector = [f32; 3];

use crate::{
    constants::SPECULAR_EXPONENT,
    vector::{normalize_vector, dot_product}
};

#[derive(Clone, Copy)]
pub struct LightingConfig {
    pub ambient_light_color: [f32; 3],
    pub point_light_color: [f32; 3],
    pub point_light_vector: [f32; 3],
    // note: viewer vector is always <0, 0, 1> so all the math for backface culling and lighting is hardcoded
}

#[derive(Clone, Copy)]
pub struct ReflectionConstants {
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
}

pub fn get_illumination(normal: &Vector, config: &LightingConfig, constants: &ReflectionConstants) -> (usize, usize, usize) {
    let normal = &normalize_vector(&normal);
    let point_light_vector = normalize_vector(&config.point_light_vector);

    let ambient = get_ambient(&config.ambient_light_color, &constants.ambient);
    let diffuse = get_diffuse(normal, &point_light_vector, &config.point_light_color, &constants.diffuse);
    let specular = get_specular(normal, &point_light_vector, &config.point_light_color, &constants.specular);

    clamp_color([
        ambient[0] + diffuse[0] + specular[0],
        ambient[1] + diffuse[1] + specular[1],
        ambient[2] + diffuse[2] + specular[2],
    ])
}

pub fn get_ambient(ambient_light_color: &Vector, ambient_constant: &Vector) -> Vector {
    // i_ambient = ambient color * ambient reflection constant
    [
        ambient_light_color[0] * ambient_constant[0],
        ambient_light_color[1] * ambient_constant[1],
        ambient_light_color[2] * ambient_constant[2],
    ]
}

pub fn get_diffuse(normal: &Vector, light_vector: &Vector, light_color: &Vector, diffuse_constant: &Vector) -> Vector {
    // i_diffuse = point color * diffuse reflection constant * (normalized normal dot normalized light)
    let n_dot_l = f32::max(0.0, dot_product(normal, light_vector));
    [
        light_color[0] * diffuse_constant[0] * n_dot_l,
        light_color[1] * diffuse_constant[1] * n_dot_l,
        light_color[2] * diffuse_constant[2] * n_dot_l,
    ]
}

pub fn get_specular(normal: &Vector, light_vector: &Vector, light_color: &Vector, specular_constant: &Vector) -> Vector {
    // i_specular = point color * specular reflection constant * (normalized reflection dot view)^exp
    // where exp > 1
    // normalized reflection = [2 * normalized normal * (normalized normal dot normalized light) - normalized light]
    
    // for normalized reflection dot view:
    // since view just <0, 0, 1>, we can be lazy and take the z value, raise it to exp, and call it r_z
    let n_dot_l = f32::max(0.0, dot_product(normal, light_vector));
    let r_z = f32::max(0.0, 2.0 * normal[2] * n_dot_l - light_vector[2]).powf(SPECULAR_EXPONENT);

    [
        light_color[0] * specular_constant[0] * r_z,
        light_color[1] * specular_constant[1] * r_z,
        light_color[2] * specular_constant[2] * r_z,
    ]
}

fn clamp_color(vector: Vector) -> (usize, usize, usize) {
    (
        vector[0].clamp(0.0, 255.0) as usize,
        vector[1].clamp(0.0, 255.0) as usize,
        vector[2].clamp(0.0, 255.0) as usize,
    )
}
