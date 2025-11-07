#![allow(dead_code)]

/* CONFIG */
pub const DEFAULT_SCRIPT: &str = "scripts/dino";
pub const STEPS: i32 = 20; // steps for parametric equations
pub const ENABLE_BACK_FACE_CULLING: bool = true;
pub const ENABLE_SCAN_LINE_CONVERSION: bool = true;
pub const ENABLE_Z_BUFFER: bool = true;

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
