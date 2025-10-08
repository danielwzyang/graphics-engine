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
        let mut iterator = lines.map_while(Result::ok).enumerate();

        // while iterator has valid item
        while let Some((_, command)) = iterator.next() {
            // skip empty lines and comments
            if command.is_empty() || command.starts_with('#') {
                continue;
            }

            // match commands
            match command.as_str() {
                "line" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 6 {
                        println!("{}:{} -> expected 6 arguments for 'line' command", path, line_number + 1);
                    }

                    edges.add_edge(
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[5], path, line_number + 1)?,
                    );
                }

                "ident" => {
                    transformation_matrix = Matrix::identity();
                }

                "scale" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        println!("{}:{} -> expected 3 arguments for 'scale' command", path, line_number + 1);
                    }

                    transformation_matrix.dilate(
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                    );
                }

                "move" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        println!("{}:{} -> expected 3 arguments for 'move' command", path, line_number + 1);
                    }

                    transformation_matrix.translate(
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                    );
                }

                "rotate" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 2 {
                        println!("{}:{} -> expected 2 arguments for 'rotate' command", path, line_number + 1);
                    }

                    transformation_matrix.rotate(
                        match parts[0] {
                            "x" => matrix::Rotation::X,
                            "y" => matrix::Rotation::Y,
                            _ => matrix::Rotation::Z // for simplicity assume rotation by z
                        },
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                    );
                }

                "apply" => {
                    Matrix::multiply(&transformation_matrix, &mut edges);
                }

                "display" => {
                    picture.clear();
                    edges.render_edges(&mut picture, &colors::MAGENTA);
                    println!("Waiting for display to close...");
                    picture.display()?;
                }

                "save" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let filename = arguments.trim();

                    if filename.is_empty() {
                        println!("{}:{} -> expected filename for 'save' command", path, line_number + 1);
                    }

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

fn convert_parameter<T: std::str::FromStr>(parameter: &str, path: &str, line_number: usize) -> Result<T, Box<dyn Error>> {
    match parameter.parse::<T>() {
        Ok(value) => Ok(value),
        _ => Err(format!("{}:{} -> Invalid parameter: {}", path, line_number, parameter).into()),
    }
}
