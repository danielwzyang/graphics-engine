use crate::matrix;

type Matrix = Vec<[f32; 4]>;

pub struct CoordinateStack {
    data: Vec<Matrix>
}

impl CoordinateStack {
    pub fn new() -> CoordinateStack {
        let data = vec![matrix::identity()];
        CoordinateStack {
            data
        }
    }

    pub fn peek(stack: &CoordinateStack) -> Matrix {
        stack.last().unwrap_or_else(matrix::identity())
    }

    pub fn pop(stack: &mut CoordinateStack) {
        stack.pop();
    }

    pub fn push(stack: &mut CoordinateStack) {
        if let Some(top) = stack.last() {
            stack.push(top.clone());
        } else {
            stack.push(matrix::identity())
        }
    }

    pub fn apply_transformation(stack: &mut CoordinateStack, transformation_matrix: &Matrix) {
        if let Some(top) = stack.last_mut() {
            let mut new_transform = transformation_matrix.clone();
            matrix::multiply(top, &mut new_transform);
            *top = new_transform;
        }
    }
}