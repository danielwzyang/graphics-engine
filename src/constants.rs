#![allow(dead_code)]

/* CONFIG */
pub const DEFAULT_SCRIPT: &str = "scripts/dino";
pub const DEFAULT_PICTURE_DIMENSIONS: (usize, usize) = (500, 500);
pub const DEFAULT_BACKGROUND_COLOR: (usize, usize, usize) = WHITE;
pub const PARAMETRIC_STEPS: i32 = 30;
pub const ENABLE_BACK_FACE_CULLING: bool = true;
pub const ENABLE_SCAN_LINE_CONVERSION: bool = true;
pub const ENABLE_Z_BUFFER: bool = true;

// only use gouraud and phong when there's no big flat surfaces e.g boxes
// boxes will appear fuzzy since we're averaging vertex normals
// flat looks fine if PARAMETRIC_STEPS is larger
#[derive(Debug)]
pub enum ShadingMode {
    Flat,
    Gouraud,
    Phong,
}
pub const SHADING_MODE: ShadingMode = ShadingMode::Phong;

/* COLORS */
pub const WHITE: (usize, usize, usize) = (255, 255, 255);
pub const BLACK: (usize, usize, usize) = (0, 0, 0);
pub const RED: (usize, usize, usize) = (255, 0, 0);
pub const GREEN: (usize, usize, usize) = (0, 255, 0);
pub const BLUE: (usize, usize, usize) = (0, 0, 255);
pub const CYAN: (usize, usize, usize) = (0, 255, 255);
pub const YELLOW: (usize, usize, usize) = (255, 255, 0);
pub const MAGENTA: (usize, usize, usize) = (255, 0, 255);

/* USEFUL MATH STUFF */
pub const HERMITE: [[f32; 4]; 4] = [
    [2.0, -3.0, 0.0, 1.0],
    [-2.0, 3.0, 0.0, 0.0],
    [1.0, -2.0, 1.0, 0.0],
    [1.0, -1.0, 0.0, 0.0],
];
pub const BEZIER: [[f32; 4]; 4] = [
    [-1.0, 3.0, -3.0, 1.0],
    [3.0, -6.0, 3.0, 0.0],
    [-3.0, 3.0, 0.0, 0.0],
    [1.0, 0.0, 0.0, 0.0],
];
pub const CUBE: [(usize, usize, usize); 12] = [
    (0, 2, 1),
    (0, 3, 2),
    (4, 1, 5),
    (4, 0, 1),
    (7, 0, 4),
    (7, 3, 0),
    (6, 3, 7),
    (6, 2, 3),
    (5, 2, 6),
    (5, 1, 2),
    (7, 5, 6),
    (7, 4, 5),
];

/* LIGHTING */
pub const AMBIENT_LIGHT_COLOR: [f32; 3] = [255.0, 255.0, 255.0];

pub const POINT_LIGHT_COLOR: [f32; 3] = [255.0, 255.0, 255.0];
pub const POINT_LIGHT_VECTOR: [f32; 3] = [1.0, 0.5, 1.0];

pub const SPECULAR_EXPONENT: f32 = 5.0;

pub const AMBIENT_REFLECTION: [f32; 3] = [0.1, 0.1, 0.1];
pub const DIFFUSE_REFLECTION: [f32; 3] = [0.5, 0.5, 0.5];
pub const SPECULAR_REFLECTION: [f32; 3] = [0.5, 0.5, 0.5];

// note: viewer vector is always <0, 0, 1> so all the math for backface culling and lighting is hardcoded