#![allow(dead_code)]

use crate::{picture::Picture};

const PI: f32 = 3.14159;

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

    pub fn add_edge(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
        self.add_point(x0, y0, z0, 1.0);
        self.add_point(x1, y1, z1, 1.0);
    }

    pub fn render_edges(&self, picture: &mut Picture, color: &(usize, usize, usize)) {
        for edge in self.data.chunks(2) {
            // loop through in pairs
            picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color);
        }
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

    pub fn rotate(&mut self, axis: Rotation, degrees: f32) {
        let mut transformation_matrix = Matrix::identity();
        let theta = degrees * (PI / 180.0);

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

    fn run_parametric<X, Y>(&mut self, x: X, y: Y, steps: f32)
        where X: Fn(f32) -> f32, Y: Fn(f32) -> f32, {
        // we can use parametric equations for things like circles and splines
        // t = 0
        // t -> 1
        // x and y have their own functions of t
        let mut t: f32 = 0.0;
        let t_step = 1.0 / steps;

        // we need to store this so we can draw edges between consecutive points
        let mut last_point = (x(t), y(t), 0.0);

        // t -> 1
        while t <= 1.0 {
            t += t_step;
            let current_point = (x(t), y(t), 0.0);

            self.add_edge(
                last_point.0, last_point.1, last_point.2,
                current_point.0, current_point.1, current_point.2,
            );

            last_point = current_point;
        }
    }

    pub fn add_circle(&mut self, cx: f32, cy: f32, r: f32, steps: f32) {
        // x(t) = rcos(2 * pi * t) + cx
        // y(t) = rsin(2 * pi * t) + cy
        let x = |t: f32| r * (2.0 * PI * t).cos() + cx;
        let y = |t: f32| r * (2.0 * PI * t).sin() + cy;

        self.run_parametric(x, y, steps);
    }
}
