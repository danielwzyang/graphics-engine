#![allow(dead_code)]

type Matrix = Vec<[f32; 4]>;

use crate::picture::Picture;

pub enum Rotation {
    X,
    Y,
    Z,
}

const PI: f32 = 3.14159;
const PARAMETRIC_STEP: f32 = 0.05;
const HERMITE_INVERSE: [[f32; 4]; 4] = [
    [2.0, -3.0, 0.0, 1.0],
    [-2.0, 3.0, 0.0, 0.0],
    [1.0, -2.0, 1.0, 0.0],
    [1.0, -1.0, 0.0, 0.0],
];

pub fn new() -> Matrix {
    vec![]
}

pub fn identity() -> Matrix {
    let mut data = vec![[0.0, 0.0, 0.0, 0.0]; 4];

    // matrix of 0s with 1s at principal diagonal
    for (i, point) in data.iter_mut().enumerate() {
        (*point)[i] = 1.0;
    }

    data
}

pub fn multiply(m1: &Matrix, m2: &mut Matrix) {
    let mut data = vec![[0.0, 0.0, 0.0, 0.0]; m2.len()];

    // iterate through every point
    for (i, point) in data.iter_mut().enumerate() {
        // iterate through the items in the point (x, y, z, w)
        for j in 0..4 {
            // iterate through the rows of m1 and columns of m2
            for k in 0..4 {
                (*point)[j] += m1[k][j] * m2[i][k];
            }
        }
    }

    *m2 = data;
}

pub fn print(m1: &Matrix) {
    for i in 0..4 {
        for point in m1 {
            // iterate through and have 3 characters for each so it's evenly spaced (left-aligned)
            print!("{:^4}", point[i]);
        }

        println!()
    }
}

fn add_point(m: &mut Matrix, x: f32, y: f32, z: f32, w: f32) {
    m.push([x, y, z, w]);
}

pub fn add_edge(m: &mut Matrix, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
    add_point(m, x0, y0, z0, 1.0);
    add_point(m, x1, y1, z1, 1.0);
}

pub fn render_edges(m: &Matrix, picture: &mut Picture, color: &(usize, usize, usize)) {
    for edge in m.chunks(2) {
        // loop through in pairs
        picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color);
    }
}

pub fn translate(m: &mut Matrix, a: f32, b: f32, c: f32) {
    let mut transformation_matrix = identity();

    // x, y, and z of last point are a, b, and c
    /*
        1 0 0 a
        0 1 0 b
        0 0 1 c
        0 0 0 1
    */

    transformation_matrix[3][0] = a;
    transformation_matrix[3][1] = b;
    transformation_matrix[3][2] = c;

    multiply(&transformation_matrix, m)
}

pub fn dilate(m: &mut Matrix, a: f32, b: f32, c: f32) {
    let mut transformation_matrix = identity();

    // 1s are replaced by a, b, and c
    /*
        a 0 0 0
        0 b 0 0
        0 0 c 0
        0 0 0 1
    */

    transformation_matrix[0][0] = a;
    transformation_matrix[1][1] = b;
    transformation_matrix[2][2] = c;

    multiply(&transformation_matrix, m)
}

pub fn rotate(m: &mut Matrix, axis: Rotation, degrees: f32) {
    let mut transformation_matrix = identity();
    let theta = degrees * (PI / 180.0);

    match axis {
        Rotation::Z => {
            /*
                cosθ -sinθ 0 0
                sinθ cosθ 0 0
                0 0 1 0
                0 0 0 1
            */

            transformation_matrix[0][0] = theta.cos();
            transformation_matrix[0][1] = theta.sin();
            transformation_matrix[1][0] = -theta.sin();
            transformation_matrix[1][1] = theta.cos();
        }

        Rotation::X => {
            /*
                1 0 0 0
                0 cosθ -sinθ 0
                0 sinθ cosθ 0
                0 0 0 1
            */

            transformation_matrix[1][1] = theta.cos();
            transformation_matrix[1][2] = theta.sin();
            transformation_matrix[2][1] = -theta.sin();
            transformation_matrix[2][2] = theta.cos();
        }

        Rotation::Y => {
            /*
                cosθ 0 sinθ 0
                0 1 0 0
                -sinθ 0 cosθ 0
                0 0 0 1
            */

            transformation_matrix[0][0] = theta.cos();
            transformation_matrix[0][2] = -theta.sin();
            transformation_matrix[2][0] = theta.sin();
            transformation_matrix[2][2] = theta.cos();
        }
    }

    multiply(&transformation_matrix, m)
}

fn run_parametric<X, Y>(m: &mut Matrix, x: X, y: Y)
    where X: Fn(f32) -> f32, Y: Fn(f32) -> f32, {
    // we can use parametric equations for things like circles and splines
    // t = 0
    // t -> 1
    // x and y have their own functions of t
    let mut t: f32 = 0.0;

    // we need to store this so we can draw edges between consecutive points
    let mut last_point = (x(t), y(t), 0.0);

    // t -> 1
    while t <= 1.0 {
        t += PARAMETRIC_STEP;
        let current_point = (x(t), y(t), 0.0);

        add_edge(
            m,
            last_point.0, last_point.1, last_point.2,
            current_point.0, current_point.1, current_point.2,
        );

        last_point = current_point;
    }
}

pub fn add_circle(m: &mut Matrix, cx: f32, cy: f32, r: f32) {
    // x(t) = rcos(2 * pi * t) + cx
    // y(t) = rsin(2 * pi * t) + cy
    let x = |t: f32| r * (2.0 * PI * t).cos() + cx;
    let y = |t: f32| r * (2.0 * PI * t).sin() + cy;

    run_parametric(m, x, y);
}

pub fn add_hermite_curve(m: &mut Matrix, x0: f32, y0: f32, x1: f32, y1: f32, rx0: f32, ry0: f32, rx1: f32, ry1: f32) {
    let hi_vec = HERMITE_INVERSE.to_vec();
    let mut g = vec![[x0, x1, rx0, rx1], [y0, y1, ry0, ry1]];
    multiply(&hi_vec, &mut g);
    println!("{:?}", g);

    let x = |t: f32| t * (t * (t * g[0][0] + g[0][1]) + g[0][2]) + g[0][3];
    let y = |t: f32| t * (t * (t * g[1][0] + g[1][1]) + g[1][2]) + g[1][3];
    
    run_parametric(m, x, y);
}