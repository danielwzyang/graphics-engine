mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");

    let mut m2 = Matrix::new();

    m2.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));

    m2.print();

    println!();

    println!("Testing ident. m1 =");

    let m1 = Matrix::identity();

    m1.print();

    println!();

    println!("Testing Matrix mult. m1 * m2 =");

    Matrix::multiply(&m1, &mut m2);
    m2.print();

    println!();

    let mut m1 = Matrix::new();

    m1.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    m1.add_edge((7.0, 8.0, 9.0), (10.0, 11.0, 12.0));

    println!("Testing Matrix mult. m1 =");

    m1.print();

    println!();

    println!("Testing Matrix mult. m1 * m2 =");

    Matrix::multiply(&m1, &mut m2);
    m2.print();

    println!();

    let mut picture = Picture::new(500, 500, 255, &colors::BLACK);

    let mut edges = Matrix::new();

    let mut theta = 0.2;
    let theta_step = 0.15;
    let size_step = 5.0;

    for i in 0..33 {
        let p1 = (-((i as f32 + 1.0) * size_step), ((i as f32 + 1.0) * size_step), 0.0);
        let p2 = (((i as f32 + 1.0) * size_step), ((i as f32 + 1.0) * size_step), 0.0);
        let p3 = (((i as f32 + 1.0) * size_step), -((i as f32 + 1.0) * size_step), 0.0);
        let p4 = (-((i as f32 + 1.0) * size_step), -((i as f32 + 1.0) * size_step), 0.0);

        edges.add_edge(p1, p2);
        edges.add_edge(p2, p3);
        edges.add_edge(p3, p4);
        edges.add_edge(p4, p1);

        edges.rotate(matrix::Rotation::Z, theta);
        edges.translate(250.0, 250.0, 0.0);
        edges.render_edges(&mut picture, &colors::RAINBOW[i % 6])?;
        edges = Matrix::new();

        theta += theta_step;
    }

    picture.save_as_file("test.ppm")?;

    Ok(())
}
