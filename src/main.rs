use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};

fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::BLACK);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Please input a path to a script.".into());
    }

    let path = args[1].clone();

    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }

    let mut edges = Matrix::new();

    for i in 0..20 {
        edges.add_circle(250.0, 250.0, (i as f32) * 10.0, 360.0);
    }

    let mut transformation_matrix = Matrix::identity();

    transformation_matrix.rotate(matrix::Rotation::X, 45.0);
    transformation_matrix.rotate(matrix::Rotation::Y, 45.0);
    transformation_matrix.rotate(matrix::Rotation::Z, 45.0);

    Matrix::multiply(&transformation_matrix, &mut edges);

    edges.render_edges(&mut picture, &colors::MAGENTA);

    picture.save_as_file("pic.png")?;

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
