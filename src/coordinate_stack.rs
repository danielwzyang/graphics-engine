use crate::matrix;

type Matrix = Vec<[f32; 4]>;

pub struct CoordinateStack {
    data: Vec<Matrix>
}

impl CoordinateStack {
    pub fn new() -> Self {
        let data = vec![matrix::identity()];
        Self {
            data
        }
    }

    pub fn peek(&self) -> Matrix {
        if self.data.is_empty() {
            println!("Stack is empty, defaulting to identity matrix.");
            matrix::identity()
        } else {
            self.data.last().unwrap().to_vec()
        }
    }

    pub fn pop(&mut self) {
        if !self.data.is_empty() {
            self.data.pop();
        } else {
            println!("Stack was popped when empty.");
        }
    }

    pub fn push(&mut self) {
        if let Some(top) = self.data.last() {
            self.data.push(top.clone());
        } else {
            self.data.push(matrix::identity());
        }
    }

    pub fn apply_transformation(&mut self, transformation_matrix: Matrix) {
        if let Some(top) = self.data.last_mut() {
            let mut new_transform = transformation_matrix;
            matrix::multiply(top, &mut new_transform);
            *top = new_transform;
        }
    }
}
