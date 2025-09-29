#![allow(dead_code)]

use crate::picture::Picture;
use std::error::Error;

pub struct Matrix {
    data: Vec<[f32; 4]>
}

pub enum Rotation {
    X,
    Y,
    Z,
}

impl Matrix {
    // constructor
    pub fn new() -> Matrix {
        // empty edge list
        let data = vec![];
        Matrix {
            data
        }
    }

    pub fn identity() -> Matrix {
        let mut data = vec![[0.0, 0.0, 0.0, 0.0]; 4];

        // matrix of 0s with 1s at principal diagonal
        for (i, point) in data.iter_mut().enumerate() {
            (*point)[i] = 1.0;
        }

        Matrix { data }
    }

    pub fn multiply(m1: &Matrix, m2: &mut Matrix) {
        let mut data = vec![[0.0, 0.0, 0.0, 0.0]; m2.data.len()];

        // iterate through every point
        for (i, point) in data.iter_mut().enumerate() {
            // iterate through the items in the point (x, y, z, w)
            for j in 0..4 {
                // iterate through the rows of m1 and columns of m2
                for k in 0..4 {
                    (*point)[j] += m1.data[k][j] * m2.data[i][k];
                }
            }
        }

        m2.data = data;
    }

    pub fn print(&self) {
        for i in 0..4 {
            for point in &self.data{
                // iterate through and have 3 characters for each so it's evenly spaced (left-aligned)
                print!("{:^4}", point[i]);
            }

            println!()
        }
    }

    fn add_point(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.data.push([x, y, z, w]);
    }

    pub fn add_edge(&mut self, point1: (f32, f32, f32), point2: (f32, f32, f32)) {
        self.add_point(point1.0, point1.1, point1.2, 1.0);
        self.add_point(point2.0, point2.1, point2.2, 1.0);
    }

    pub fn render_edges(self, picture: &mut Picture, color: &(usize, usize, usize)) -> Result<(), Box<dyn Error>> {
        for edge in self.data.chunks(2) {
            // loop through in pairs
            picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color)?;
        }

        Ok(())
    }

    pub fn translate(&mut self, a: f32, b: f32, c: f32) {
        let mut transformation_matrix = Matrix::identity();

        // x, y, and z of last point are a, b, and c
        /*
            1 0 0 a
            0 1 0 b
            0 0 1 c
            0 0 0 1
        */

        transformation_matrix.data[3][0] = a;
        transformation_matrix.data[3][1] = b;
        transformation_matrix.data[3][2] = c;

        Matrix::multiply(&transformation_matrix, self)
    }

    pub fn dilate(&mut self, a: f32, b: f32, c: f32) {
        let mut transformation_matrix = Matrix::identity();

        // 1s are replaced by a, b, and c
        /*
            a 0 0 0
            0 b 0 0
            0 0 c 0
            0 0 0 1
        */

        transformation_matrix.data[0][0] = a;
        transformation_matrix.data[1][1] = b;
        transformation_matrix.data[2][2] = c;

        Matrix::multiply(&transformation_matrix, self)
    }

    pub fn rotate(&mut self, axis: Rotation, theta: f32) {
        let mut transformation_matrix = Matrix::identity();

        match axis {
            Rotation::Z => {
                /*
                    cosθ -sinθ 0 0
                    sinθ cosθ 0 0
                    0 0 1 0
                    0 0 0 1
                */

                transformation_matrix.data[0][0] = theta.cos();
                transformation_matrix.data[0][1] = theta.sin();
                transformation_matrix.data[1][0] = -theta.sin();
                transformation_matrix.data[1][1] = theta.cos();
            }

            Rotation::X => {
                /*
                    1 0 0 0
                    0 cosθ -sinθ 0
                    0 sinθ cosθ 0
                    0 0 0 1
                */

                transformation_matrix.data[1][1] = theta.cos();
                transformation_matrix.data[1][2] = theta.sin();
                transformation_matrix.data[2][1] = -theta.sin();
                transformation_matrix.data[2][2] = theta.cos();
            }

            Rotation::Y => {
                /*
                    cosθ 0 sinθ 0
                    0 1 0 0
                    -sinθ 0 cosθ 0
                    0 0 0 1
                */

                transformation_matrix.data[0][0] = theta.cos();
                transformation_matrix.data[0][2] = -theta.sin();
                transformation_matrix.data[2][0] = theta.sin();
                transformation_matrix.data[2][2] = theta.cos();
            }
        }

        Matrix::multiply(&transformation_matrix, self)
    }
}
