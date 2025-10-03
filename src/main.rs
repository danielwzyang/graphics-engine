mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::BLACK);

    let mut edges = Matrix::new();

    for i in 0..20 {
        edges.add_circle(250.0, 250.0, (i as f32) * 10.0, 360.0);
    }

    edges.rotate(matrix::Rotation::X, 0.75);
    edges.rotate(matrix::Rotation::Y, 1.0);
    edges.rotate(matrix::Rotation::Z, -0.5);

    edges.render_edges(&mut picture, &colors::MAGENTA);

    picture.save_as_file("test.ppm")?;

    Ok(())
}
