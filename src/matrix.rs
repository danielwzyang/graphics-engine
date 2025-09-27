pub struct Matrix {
    data: Vec<[f32; 4]>
}

impl Matrix {
    pub fn new() -> Matrix {
        let data = vec![];
        Matrix {
            data
        }
    }

    pub fn identity() -> Matrix {
        let mut data = vec![[0.0, 0.0, 0.0, 0.0]; 4];

        for (i, point) in data.iter_mut().enumerate() {
            (*point)[i] = 1.0;
        }

        Matrix { data }
    }

    pub fn multiply(m1: &Matrix, m2: &Matrix) -> Matrix {
        let mut data = vec![[0.0, 0.0, 0.0, 0.0]; m2.data.len()];

        for (i, point) in data.iter_mut().enumerate() {
            for j in 0..4 {
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
                print!("{:<7.2} ", point[i]);
            }
            println!()
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32) {
        self.data.push([x, y, z, 1.0]);
    }

    pub fn add_edge(&mut self, point1: (f32, f32, f32), point2: (f32, f32, f32)) {
        self.add_point(point1.0, point1.1, point1.2);
        self.add_point(point2.0, point2.1, point2.2);
    }
} 
