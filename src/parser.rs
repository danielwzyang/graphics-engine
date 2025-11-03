use crate::{matrix, constants, coordinate_stack};
use crate::coordinate_stack::{peek, pop, push, apply_transformation};
use crate::picture::Picture;
use crate::edge_list::{render_edges, add_bezier_curve, add_circle, add_edge, add_hermite_curve};
use crate::polygon_list::{add_box, add_polygon, add_sphere, add_torus, render_polygons};
use std::{error::Error, io, io::BufRead, fs::File, path::Path};

pub fn read_script(path: &str) -> Result<(), Box<dyn Error>> {
    let mut picture = Picture::new(500, 500, 255, &constants::WHITE);
    let mut edges = matrix::new();
    let mut polygons = matrix::new();
    let mut coordinate_stack = coordinate_stack::new();

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
                    println!("Waiting for display to close...");
                    picture.display()?;
                }

                "clear" => {
                    picture.clear();
                }

                "save" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let filename = arguments.trim();

                    if filename.is_empty() {
                        panic!("{}:{} -> 'save' command expected <filepath>", path, line_number + 1);
                    }

                    picture.save_as_file(filename)?;
                }

                "pop" => {
                    pop(&mut coordinate_stack);
                }

                "push" => {
                    push(&mut coordinate_stack);
                }

                "scale" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        panic!("{}:{} -> 'scale' command expected <x> <y> <z>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    apply_transformation(
                        &mut coordinate_stack,
                        matrix::dilation(p[0], p[1], p[2]),
                    );
                }

                "move" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 3 {
                        panic!("{}:{} -> 'move' command expected <x> <y> <z>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    apply_transformation(
                        &mut coordinate_stack,
                        matrix::translation(p[0], p[1], p[2]),
                    );
                }

                "rotate" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 2 {
                        panic!("{}:{} -> 'rotate' command expected <x | y | z> <degrees>", path, line_number + 1);
                    }

                    apply_transformation(
                        &mut coordinate_stack,
                        matrix::rotation(
                            match parts[0] {
                                "x" => matrix::Rotation::X,
                                "y" => matrix::Rotation::Y,
                                "z" => matrix::Rotation::Z,
                                parameter => panic!("{}:{} -> invalid parameter: '{}'. expected <x | y | z>", path, line_number + 1, parameter)
                            },
                            convert_parameter::<f32>(parts[1], path, line_number + 1)?,
                        ),
                    )
                }

                "line" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 6 {
                        panic!("{}:{} -> 'line' command expected <x0> <y0> <x1> <y1>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_edge(&mut edges, p[0], p[1], p[2], p[3], p[4], p[5]);
                    matrix::multiply(&peek(&coordinate_stack), &mut edges);
                    render_edges(&edges, &mut picture, &constants::BLUE);
                    edges = matrix::new();
                }

                "circle" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'circle' command expected <cx> <cy> <cz> <r>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_circle(&mut edges, p[0], p[1], p[2], p[3]);
                    matrix::multiply(&peek(&coordinate_stack), &mut edges);
                    render_edges(&edges, &mut picture, &constants::BLUE);
                    edges = matrix::new();
                }

                "hermite" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 8 {
                        panic!("{}:{} -> 'hermite' command expected <x0> <y0> <x1> <y1> <rx0> <ry0> <rx1> <ry1>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_hermite_curve(&mut edges, p[0], p[1], p[2], p[3], p[4], p[5], p[7], p[8]);
                    matrix::multiply(&peek(&coordinate_stack), &mut edges);
                    render_edges(&edges, &mut picture, &constants::BLUE);
                    edges = matrix::new();
                }

                "bezier" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 8 {
                        panic!("{}:{} -> 'bezier' command expected <x0> <y0> <x1> <y1> <x2> <y2> <x3> <y3>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_bezier_curve(&mut edges, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);
                    matrix::multiply(&peek(&coordinate_stack), &mut edges);
                    render_edges(&edges, &mut picture, &constants::BLUE);
                    edges = matrix::new();
                }

                "polygon" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 9 {
                        panic!("{}:{} -> 'polygon' command expected <x0> <y0> <z0> <x1> <y1> <z1> <x2> <y2> <z2>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_polygon(&mut polygons, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8]);
                    matrix::multiply(&peek(&coordinate_stack), &mut polygons);
                    render_polygons(&polygons, &mut picture, &constants::BLUE);
                    polygons = matrix::new();
                }

                "box" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 6 {
                        panic!("{}:{} -> 'box' command expected <x> <y> <z> <w> <h> <d>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_box(&mut polygons, p[0], p[1], p[2], p[3], p[4], p[5]);
                    matrix::multiply(&peek(&coordinate_stack), &mut polygons);
                    render_polygons(&polygons, &mut picture, &constants::BLUE);
                    polygons = matrix::new();
                }

                "sphere" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'sphere' command expected <cx> <cy> <cz> <r>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_sphere(&mut polygons, p[0], p[1], p[2], p[3]);
                    matrix::multiply(&peek(&coordinate_stack), &mut polygons);
                    render_polygons(&polygons, &mut picture, &constants::BLUE);
                    polygons = matrix::new();
                }

                "torus" => {
                    let (line_number, arguments) = iterator.next().unwrap();
                    let parts: Vec<&str> = arguments.split_whitespace().collect();

                    if parts.len() < 4 {
                        panic!("{}:{} -> 'torus' command expected <cx> <cy> <cz> <r1> <r2>", path, line_number + 1);
                    }

                    let p = convert_parameters::<f32>(parts, path, line_number + 1)?;

                    add_torus(&mut polygons, p[0], p[1], p[2], p[3], p[4]);
                    matrix::multiply(&peek(&coordinate_stack), &mut polygons);
                    render_polygons(&polygons, &mut picture, &constants::BLUE);
                    polygons = matrix::new();
                }

                unknown => {
                    println!("{}:{} -> unknown command '{}'.", path, line_number, unknown);
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

fn convert_parameters<T: std::str::FromStr>(parameters: Vec<&str>, path: &str, line_number: usize) -> Result<Vec<T>, Box<dyn Error>> {
    let mut res = vec![];
    for parameter in parameters {
        res.push(convert_parameter::<T>(parameter, path, line_number)?);
    }
    Ok(res)
}

fn convert_parameter<T: std::str::FromStr>(parameter: &str, path: &str, line_number: usize) -> Result<T, Box<dyn Error>> {
    match parameter.parse::<T>() {
        Ok(value) => Ok(value),
        _ => Err(format!("{}:{} -> invalid parameter: '{}'. expected {}.", path, line_number, parameter, std::any::type_name::<T>()).into()),
    }
}
