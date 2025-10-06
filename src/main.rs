use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::process::Command;

mod picture;
mod colors;
mod matrix;
use crate::{matrix::Matrix, picture::Picture};

fn main() -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &colors::BLACK);

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        return Err("Please input a path to a script.".into());
    }

    let path = &arguments[1];

    let mut edges = Matrix::new();
    let mut transformation_matrix = Matrix::identity();

    if let Ok(lines) = read_lines(path) {
        // create an iterator to read through lines
        let mut iterator = lines.map_while(Result::ok).map(|s| s.trim().to_string()).peekable();
        let mut current_line = 1;

        // while iterator has valid item
        while let Some(command) = iterator.next() {
            // skip empty lines and comments
            if command.is_empty() || command.starts_with('#') {
                continue;
            }

            // match commands
            match command.as_str() {
                "line" => {
                    if let Some(arguments) = iterator.next() {
                        current_line += 1;
                        let parts: Vec<&str> = arguments.split_whitespace().collect();
                        if parts.len() >= 6 {
                            match (
                                parts[0].parse::<f32>(), parts[1].parse::<f32>(), parts[2].parse::<f32>(), parts[3].parse::<f32>(), parts[4].parse::<f32>(), parts[5].parse::<f32>()
                            ) {
                                (Ok(x0), Ok(y0), Ok(z0), Ok(x1), Ok(y1), Ok(z1)) => {
                                    edges.add_edge(x0, y0, z0, x1, y1, z1);
                                }
                                _ => eprintln!("{}:{} -> Invalid numbers for 'line' arguments: {}", path, current_line, arguments),
                            }
                        } else {
                            eprintln!("{}:{} -> Not enough arguments for 'line' (expected 6): {}", path, current_line, arguments);
                        }
                    } else { 
                        eprintln!("{}:{} -> Missing arguments for 'line' command.", path, current_line); 
                    }
                }

                "ident" => {
                    transformation_matrix = Matrix::identity();
                }

                "scale" => {
                    if let Some(arguments) = iterator.next() {
                        current_line += 1;
                        let parts: Vec<&str> = arguments.split_whitespace().collect();
                        if parts.len() >= 3 {
                            match (parts[0].parse::<f32>(), parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                                (Ok(a), Ok(b), Ok(c)) => transformation_matrix.dilate(a, b, c),
                                _ => eprintln!("{}:{} -> Invalid numbers for 'dilate' arguments: {}", path, current_line, arguments),
                            }
                        } else { eprintln!("{}:{} -> Not enough arguments for 'dilate' (expected 3): {}", path, current_line, arguments); }
                    } else { eprintln!("{}:{} -> Missing arguments for 'dilate' command", path, current_line); }
                }

                "move" => {
                    if let Some(arguments) = iterator.next() {
                        current_line += 1;
                        let parts: Vec<&str> = arguments.split_whitespace().collect();
                        if parts.len() >= 3 {
                            match (parts[0].parse::<f32>(), parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                                (Ok(a), Ok(b), Ok(c)) => transformation_matrix.translate(a, b, c),
                                _ => eprintln!("{}:{} -> Invalid numbers for 'translate' arguments: {}", path, current_line, arguments),
                            }
                        } else { eprintln!("{}:{} -> Not enough arguments for 'translate' (expected 3): {}", path, current_line, arguments); }
                    } else { eprintln!("{}:{} -> Missing arguments for 'translate' command", path, current_line); }
                }

                "rotate" => {
                    if let Some(arguments) = iterator.next() {
                        current_line += 1;
                        let mut parts = arguments.split_whitespace();
                        if let (Some(axis), Some(degrees)) = (parts.next(), parts.next()) {
                            match degrees.parse::<f32>() {
                                Ok(deg) => {
                                    match axis.chars().next().map(|c| c.to_ascii_lowercase()) {
                                        Some('x') => transformation_matrix.rotate(matrix::Rotation::X, deg),
                                        Some('y') => transformation_matrix.rotate(matrix::Rotation::Y, deg),
                                        Some('z') => transformation_matrix.rotate(matrix::Rotation::Z, deg),
                                        _ => eprintln!("{}:{} -> Unknown axis for 'rotate': {}", path, current_line, axis),
                                    }
                                }
                                Err(_) => eprintln!("{}:{} -> Invalid degrees for 'rotate': {}", path, current_line, degrees),
                            }
                        } else { eprintln!("{}:{} -> Invalid arguments for 'rotate' (expected <axis> <degrees>): {}", path, current_line, arguments); }
                    } else { eprintln!("{}:{} -> Missing arguments for 'rotate' command", path, current_line); }
                }

                "apply" => {
                    Matrix::multiply(&transformation_matrix, &mut edges);
                }

                "display" => {
                    picture.clear();
                    edges.render_edges(&mut picture, &colors::WHITE);

                    if let Err(e) = picture.save_as_file("temp.ppm") {
                        eprintln!("{}:{} -> Failed to display: {}", path, current_line, e);
                    }

                    println!("Waiting for display window to close to proceed...");

                    Command::new("display")
                        .args(["temp.ppm"])
                        .output()?;
                }

                "save" => {
                    if let Some(arguments) = iterator.next() {
                        current_line += 1;
                        
                        let filename = arguments.trim();
                        picture.clear();
                        edges.render_edges(&mut picture, &colors::WHITE);

                        if let Err(e) = picture.save_as_file(filename) {
                            eprintln!("{}:{} -> Failed to save {}: {}", path, current_line, filename, e);
                        }
                    } else { eprintln!("{}:{} -> Missing filename for 'save' command", path, current_line); }
                }

                other => eprintln!("{}:{} -> Unknown command: '{}'", path, current_line, other),
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
