mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::BLACK);

    let mut edges = Matrix::new();

    edges.add_circle((250.0, 250.0), 50.0, 360.0);

    edges.render_edges(&mut picture, &colors::WHITE);

    picture.save_as_file("test.ppm")?;

    Ok(())
}
