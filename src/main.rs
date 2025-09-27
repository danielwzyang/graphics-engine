mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Testing add_edge. Adding (1, 2, 3), (4, 5, 6) m2 =");

    let mut m2 = Matrix::new();

    m2.add_edge((1, 2, 3), (4, 5, 6));

    m2.print();

    println!();

    println!("Testing ident. m1 =");

    let m1 = Matrix::identity();

    m1.print();

    println!();

    println!("Testing Matrix mult. m1 * m2 =");

    m2 = Matrix::multiply(&m1, &m2);
    m2.print();

    println!();

    let mut m1 = Matrix::new();

    m1.add_edge((1, 2, 3), (4, 5, 6));
    m1.add_edge((7, 8, 9), (10, 11, 12));

    println!("Testing Matrix mult. m1 =");

    m1.print();

    println!();

    println!("Testing Matrix mult. m1 * m2 =");

    m1 = Matrix::multiply(&m1, &m2);
    m1.print();

    println!();

    let mut edges = Matrix::new();

    edges.add_edge((50, 450, 0), (100, 450, 0));
    edges.add_edge((50, 450, 0), (50, 400, 0));
    edges.add_edge((100, 450, 0), (100, 400, 0));
    edges.add_edge((100, 400, 0), (50, 400, 0));

    edges.add_edge((200, 450, 0), (250, 450, 0));
    edges.add_edge((200, 450, 0), (200, 400, 0));
    edges.add_edge((250, 450, 0), (250, 400, 0));
    edges.add_edge((250, 400, 0), (200, 400, 0));

    edges.add_edge((150, 400, 0), (130, 360, 0));
    edges.add_edge((150, 400, 0), (170, 360, 0));
    edges.add_edge((130, 360, 0), (170, 360, 0));

    edges.add_edge((100, 340, 0), (200, 340, 0));
    edges.add_edge((100, 320, 0), (200, 320, 0));
    edges.add_edge((100, 340, 0), (100, 320, 0));
    edges.add_edge((200, 340, 0), (200, 320, 0)); 

    let mut picture = Picture::new(500, 500, 255);

    edges.render_edges(&mut picture, &colors::WHITE)?;

    picture.save_as_file("test.ppm")?;

    Ok(())
}