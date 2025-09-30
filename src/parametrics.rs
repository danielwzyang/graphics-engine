use std::error::Error;
use crate::Matrix;

const PI: f32 = 3.141;

pub fn add_circle(matrix: &mut Matrix, center: (f32, f32), r: f32, t_step: f32) -> Result<(), Box<dyn Error>> {
    let x = |t: f32| r * (2.0 * PI * t).cos() + center.0;
    let y = |t: f32| r * (2.0 * PI * t).sin() + center.1;

    let mut t: f32 = 0.0;
    let mut last_point = (x(t), y(t), 0.0);

    while t <= 1.0 {
        t += t_step;
        let current_point = (x(t), y(t), 0.0);

        matrix.add_edge(
            last_point,
            current_point,
        );

        last_point = current_point;
    }

    Ok(())
}
