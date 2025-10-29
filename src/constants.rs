// the number of steps made during a parametric loop
pub const STEPS: i32 = 10;

// cubic hermite and bezier matrices to find polynomial coefficients
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

// the points for each polygon of a cube
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