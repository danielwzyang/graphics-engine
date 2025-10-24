use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

mod picture;
mod colors;
mod matrix;
use crate::picture::Picture;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::WHITE);

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        return Err("Please input a path to a script.".into());
    }

    let path = &arguments[1];

    let mut edges = matrix::new();
    let mut transformation_matrix = matrix::identity();

    if let Ok(lines) = read_lines(path) {
        // create an iterator to read through lines
        let mut iterator = lines.map_while(Result::ok).enumerate();

        // while iterator has valid item
        while let Some((line_number, command)) = iterator.next() {
            // skip empty lines and comments
            if command.is_empty() || command.starts_with('#') {
                continue;
            }

            // match commands
            match command.as_str() {
                "display" => {
                    picture.clear();
                    matrix::render_edges(&edges, &mut picture, &colors::BLACK);
                    println!("Waiting for display to close...");
                    picture.display()?;
                }

                "clear" => {
                    edges = matrix::new();
                }

                "save" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let filename = arguments.trim();

                    if filename.is_empty() {
                        panic!("{}:{} -> 'save' command expected <filepath>", path, line_number + 1);
                    }

                    picture.clear();
                    matrix::render_edges(&edges, &mut picture, &colors::BLACK);
                    picture.save_as_file(filename)?;
                }

                "ident" => {
                    transformation_matrix = matrix::identity();
                }

                "scale" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        panic!("{}:{} -> 'scale' command expected <x> <y> <z>", path, line_number + 1);
                    }

                    matrix::dilate(
                        &mut transformation_matrix,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                    );
                }

                "move" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        panic!("{}:{} -> 'move' command expected <x> <y> <z>", path, line_number + 1);
                    }
                    matrix::translate(
                        &mut transformation_matrix,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                    );
                }

                "rotate" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 2 {
                        panic!("{}:{} -> 'rotate' command expected <x | y | z> <degrees>", path, line_number + 1);
                    }

                    matrix::rotate(
                        &mut transformation_matrix,
                        match parts[0] {
                            "x" => matrix::Rotation::X,
                            "y" => matrix::Rotation::Y,
                            "z" => matrix::Rotation::Z,
                            parameter => panic!("{}:{} -> invalid parameter: '{}'. expected <x | y | z>", path, line_number + 1, parameter)
                        },
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                    );
                }

                "apply" => {
                    matrix::multiply(&transformation_matrix, &mut edges);
                }

                "line" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 6 {
                        panic!("{}:{} -> 'line' command expected <x0> <y0> <x1> <y1>", path, line_number + 1);
                    }

                    matrix::add_edge(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[5], path, line_number + 1)?,
                    );
                }

                "circle" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'circle' command expected <cx> <cy> <cz> <r>", path, line_number + 1);
                    }

                    matrix::add_circle(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                    );
                }

                "hermite" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 8 {
                        panic!("{}:{} -> 'hermite' command expected <x0> <y0> <x1> <y1> <rx0> <ry0> <rx1> <ry1>", path, line_number + 1);
                    }

                    matrix::add_hermite_curve(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[5], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[6], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[7], path, line_number + 1)?,
                    );
                }

                "bezier" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 8 {
                        panic!("{}:{} -> 'bezier' command expected <x0> <y0> <x1> <y1> <x2> <y2> <x3> <y3>", path, line_number + 1);
                    }

                    matrix::add_bezier_curve(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[5], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[6], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[7], path, line_number + 1)?,
                    );
                }

                "box" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 6 {
                        panic!("{}:{} -> 'box' command expected <x> <y> <z> <w> <h> <d>", path, line_number + 1);
                    }

                    matrix::add_box(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[5], path, line_number + 1)?,
                    );
                }

                "sphere" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'sphere' command expected <cx> <cy> <cz> <r>", path, line_number + 1);
                    }

                    matrix::add_sphere(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                    );
                }

                "torus" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'torus' command expected <cx> <cy> <cz> <r> <big_r>", path, line_number + 1);
                    }

                    matrix::add_torus(
                        &mut edges,
                        convert_parameter::<f32>(parts[0], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[2], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[3], path, line_number + 1)?,
                        convert_parameter::<f32>(parts[4], path, line_number + 1)?,
                    );
                }

                unknown => {
                    panic!("{}:{} -> error parsing '{}'.", path, line_number, unknown);
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
        _ => Err(format!("{}:{} -> invalid parameter: '{}'. expected {}.", path, line_number, parameter, std::any::type_name::<T>()).into()),
    }
}
