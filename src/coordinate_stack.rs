use crate::matrix;

type Matrix = Vec<[f32; 4]>;

type CoordinateStack = Vec<Matrix>;

pub fn new() -> CoordinateStack {
    let data = vec![matrix::identity()];
    data
}

pub fn peek(stack: &CoordinateStack) -> Matrix {
    if stack.is_empty() {
        println!("Stack is empty, defaulting to identity matrix.");
        matrix::identity()
    } else {
        stack.last().unwrap().to_vec()
    }
}

pub fn pop(stack: &mut CoordinateStack) {
    if !stack.is_empty() {
        stack.pop();
    } else {
        println!("Stack was popped when empty.");
    }
}

pub fn push(stack: &mut CoordinateStack) {
    if let Some(top) = stack.last() {
        stack.push(top.clone());
    } else {
        stack.push(matrix::identity());
    }
}

pub fn apply_transformation(stack: &mut CoordinateStack, transformation_matrix: Matrix) {
    if let Some(top) = stack.last_mut() {
        let mut new_transform = transformation_matrix;
        matrix::multiply(top, &mut new_transform);
        *top = new_transform;
    }
}
