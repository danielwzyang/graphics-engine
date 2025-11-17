#![allow(dead_code)]

type Matrix = Vec<[f32; 4]>;

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    X,
    Y,
    Z,
}

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

pub fn multiply(m1: &[[f32; 4]], m2: &mut Matrix) {
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

pub fn add_point(m: &mut Matrix, x: f32, y: f32, z: f32, w: f32) {
    m.push([x, y, z, w]);
}

pub fn translation(a: f32, b: f32, c: f32) -> Matrix {
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

    transformation_matrix
}

pub fn dilation(a: f32, b: f32, c: f32) -> Matrix {
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

    transformation_matrix
}

pub fn rotation(axis: Rotation, degrees: f32) -> Matrix {
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

    transformation_matrix
}