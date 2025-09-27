use crate::picture::Picture;
use std::error::Error;

pub struct Matrix {
    data: Vec<[isize; 4]>
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
        let mut data = vec![[0, 0, 0, 0]; 4];

        // matrix of 0s with 1s at principal diagonal
        for (i, point) in data.iter_mut().enumerate() {
            (*point)[i] = 1;
        }

        Matrix { data }
    }

    pub fn multiply(m1: &Matrix, m2: &Matrix) -> Matrix {
        let mut data = vec![[0, 0, 0, 0]; m2.data.len()];

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

        Matrix { data }
    }

    pub fn print(&self) {
        for i in 0..4 {
            for point in &self.data{
                // iterate through and have 3 characters for each so it's evenly spaced (left-aligned)
                print!("{:<3} ", point[i]);
            }

            println!()
        }
    }

    pub fn add_point(&mut self, x: isize, y: isize, z: isize, w: isize) {
        self.data.push([x, y, z, w]);
    }

    pub fn add_edge(&mut self, point1: (isize, isize, isize), point2: (isize, isize, isize)) {
        self.add_point(point1.0, point1.1, point1.2, 1);
        self.add_point(point2.0, point2.1, point2.2, 1);
    }

    pub fn render_edges(self, picture: &mut Picture, color: &(usize, usize, usize)) -> Result<(), Box<dyn Error>> {
        for edge in self.data.chunks(2) {
            // loop through in pairs
            picture.draw_line(edge[0][0] as isize, edge[0][1] as isize, edge[1][0] as isize, edge[1][1] as isize, &color)?;
        }

        Ok(())
    }
} 
