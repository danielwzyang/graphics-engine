use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::WHITE);

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        return Err("Please input a path to a script.".into());
    }

    let path = &arguments[1];

    let mut edges = Matrix::new();
    let mut transformation_matrix = Matrix::identity();

    if let Ok(lines) = read_lines(path) {
        // create an iterator to read through lines
        let mut iterator = lines.map_while(Result::ok);

        // while iterator has valid item
        while let Some(command) = iterator.next() {
            // skip empty lines and comments
            if command.is_empty() || command.starts_with('#') {
                continue;
            }

            // match commands
            match command.as_str() {
                "line" => {
                    let arguments = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();
                    edges.add_edge(
                        parts[0].parse::<f32>()?,
                        parts[1].parse::<f32>()?,
                        parts[2].parse::<f32>()?,
                        parts[3].parse::<f32>()?,
                        parts[4].parse::<f32>()?,
                        parts[5].parse::<f32>()?,
                    );
                }

                "ident" => {
                    transformation_matrix = Matrix::identity();
                }

                "scale" => {
                    let arguments = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();
                    transformation_matrix.dilate(
                        parts[0].parse::<f32>()?,
                        parts[1].parse::<f32>()?,
                        parts[2].parse::<f32>()?,
                    );
                }

                "move" => {
                    let arguments = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();
                    transformation_matrix.translate(
                        parts[0].parse::<f32>()?,
                        parts[1].parse::<f32>()?,
                        parts[2].parse::<f32>()?,
                    );    
                }

                "rotate" => {
                    let arguments = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();
                    transformation_matrix.rotate(
                        match parts[0] {
                            "x" => matrix::Rotation::X,
                            "y" => matrix::Rotation::Y,
                            _ => matrix::Rotation::Z
                        },
                        parts[1].parse::<f32>()?,
                    ); 
                }

                "apply" => {
                    Matrix::multiply(&transformation_matrix, &mut edges);
                }

                "display" => {
                    picture.clear();
                    edges.render_edges(&mut picture, &colors::MAGENTA);
                    picture.display()?;
                }

                "save" => {
                    let arguments = iterator.next().unwrap();
                    let filename = arguments.trim();
                    picture.clear();
                    edges.render_edges(&mut picture, &colors::MAGENTA);
                    picture.save_as_file(filename)?;
                }

                unknown => {
                    println!("Error parsing '{}'.", unknown);
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
