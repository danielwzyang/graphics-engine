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

    let mut edges = Matrix::new();

    edges.add_edge((50.0, 450.0, 0.0), (100.0, 450.0, 0.0));
    edges.add_edge((50.0, 450.0, 0.0), (50.0, 400.0, 0.0));
    edges.add_edge((100.0, 450.0, 0.0), (100.0, 400.0, 0.0));
    edges.add_edge((100.0, 400.0, 0.0), (50.0, 400.0, 0.0));

    edges.add_edge((200.0, 450.0, 0.0), (250.0, 450.0, 0.0));
    edges.add_edge((200.0, 450.0, 0.0), (200.0, 400.0, 0.0));
    edges.add_edge((250.0, 450.0, 0.0), (250.0, 400.0, 0.0));
    edges.add_edge((250.0, 400.0, 0.0), (200.0, 400.0, 0.0));

    edges.add_edge((150.0, 400.0, 0.0), (130.0, 360.0, 0.0));
    edges.add_edge((150.0, 400.0, 0.0), (170.0, 360.0, 0.0));
    edges.add_edge((130.0, 360.0, 0.0), (170.0, 360.0, 0.0));

    edges.add_edge((100.0, 340.0, 0.0), (200.0, 340.0, 0.0));
    edges.add_edge((100.0, 320.0, 0.0), (200.0, 320.0, 0.0));
    edges.add_edge((100.0, 340.0, 0.0), (100.0, 320.0, 0.0));
    edges.add_edge((200.0, 340.0, 0.0), (200.0, 320.0, 0.0)); 

    edges.rotation(matrix::Rotation::Z, -0.5);

    let mut picture = Picture::new(500, 500, 255);

    edges.render_edges(&mut picture, &colors::WHITE)?;

    picture.save_as_file("test.ppm")?;

    Ok(())
}