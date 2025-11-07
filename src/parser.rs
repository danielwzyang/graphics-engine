use crate::{matrix, constants};
use crate::coordinate_stack::CoordinateStack;
use crate::picture::Picture;
use crate::edge_list::{render_edges, add_bezier_curve, add_circle, add_edge, add_hermite_curve};
use crate::polygon_list::{add_box, add_polygon, add_sphere, add_torus, render_polygons};
use std::{error::Error, io, io::BufRead, fs::File, path::Path};

type Matrix = Vec<[f32; 4]>;

struct ScriptContext {
    picture: Picture,
    edges: Matrix,
    polygons: Matrix,
    coordinate_stack: CoordinateStack,
}

impl ScriptContext {
    fn new() -> Self {
        let (xres, yres) = constants::DEFAULT_PICTURE_DIMENSIONS;

        Self {
            picture: Picture::new(xres, yres, 255, &constants::DEFAULT_BACKGROUND_COLOR),
            edges: matrix::new(),
            polygons: matrix::new(),
            coordinate_stack: CoordinateStack::new(),
        }
    }

    fn render_edges(&mut self, color: &(usize, usize, usize)) {
        matrix::multiply(&self.coordinate_stack.peek(), &mut self.edges);
        render_edges(&self.edges, &mut self.picture, color);
        self.edges = matrix::new();
    }

    fn render_polygons(&mut self, color: &(usize, usize, usize)) {
        matrix::multiply(&self.coordinate_stack.peek(), &mut self.polygons);
        render_polygons(&self.polygons, &mut self.picture, color);
        self.polygons = matrix::new();
    }
}

pub fn read_script(path: &str) -> Result<(), Box<dyn Error>> {
    let mut context = ScriptContext::new();

    let lines = read_lines(path).map_err(|_| format!("'{}' not found", path))?;

    let mut iterator = lines.map_while(Result::ok).enumerate();

    while let Some((line_number, command)) = iterator.next() {
        // trim white space
        let command = command.trim();

        // ignore blank commands or comments
        if command.is_empty() || command.starts_with('#') {
            continue;
        }

        process_command(&mut context, &mut iterator, command, path, line_number)?;
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_command(
    context: &mut ScriptContext,
    iterator: &mut impl Iterator<Item = (usize, String)>,
    command: &str,
    path: &str,
    line_number: usize,
) -> Result<(), Box<dyn Error>> {
    match command {
        "display" => {
            println!("Waiting for display to close...");
            context.picture.display()?;
        }

        "clear" => context.picture.clear(),

        "save" => {
            let filename = get_next_param(iterator, path, "save", "<filepath>")?;
            context.picture.save_as_file(&filename)?;
        }

        "mesh" => handle_mesh(context, iterator, path, line_number)?,

        "pop" => context.coordinate_stack.pop(),

        "push" => context.coordinate_stack.push(),

        "scale" => {
            let params = get_next_params::<f32>(iterator, path, "scale", 3)?;

            context.coordinate_stack.apply_transformation(matrix::dilation(params[0], params[1], params[2]));
        }

        "move" => {
            let params = get_next_params::<f32>(iterator, path, "move", 3)?;

            context.coordinate_stack.apply_transformation(matrix::translation(params[0], params[1], params[2]));
        }

        "rotate" => {
            let (ln, args) = iterator.next().unwrap();
            let parts: Vec<&str> = args.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(format!("{}:{} -> 'rotate' expected <x|y|z> <degrees>", path, ln + 1).into());
            }

            let axis = match parts[0] {
                "x" => matrix::Rotation::X,
                "y" => matrix::Rotation::Y,
                "z" => matrix::Rotation::Z,
                p => return Err(format!("{}:{} -> invalid axis '{}'", path, ln + 1, p).into()),
            };

            context.coordinate_stack.apply_transformation(matrix::rotation(axis, convert_parameter::<f32>(parts[1], path, ln + 1)?));
        }

        "line" => {
            let p = get_next_params::<f32>(iterator, path, "line", 6)?;
            add_edge(&mut context.edges, p[0], p[1], p[2], p[3], p[4], p[5]);
            context.render_edges(&constants::BLUE);
        }

        "circle" => {
            let p = get_next_params::<f32>(iterator, path, "circle", 4)?;
            add_circle(&mut context.edges, p[0], p[1], p[2], p[3]);
            context.render_edges(&constants::BLUE);
        }

        "hermite" => {
            let p = get_next_params::<f32>(iterator, path, "hermite", 8)?;
            add_hermite_curve(&mut context.edges, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);
            context.render_edges(&constants::BLUE);
        }

        "bezier" => {
            let p = get_next_params::<f32>(iterator, path, "bezier", 8)?;
            add_bezier_curve(&mut context.edges, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);
            context.render_edges(&constants::BLUE);
        }

        "polygon" => {
            let p = get_next_params::<f32>(iterator, path, "polygon", 9)?;
            add_polygon(&mut context.polygons, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8]);
            context.render_polygons(&constants::BLUE);
        }

        "box" => {
            let p = get_next_params::<f32>(iterator, path, "box", 6)?;
            add_box(&mut context.polygons, p[0], p[1], p[2], p[3], p[4], p[5]);
            context.render_polygons(&constants::RED);
        }

        "sphere" => {
            let p = get_next_params::<f32>(iterator, path, "sphere", 4)?;
            add_sphere(&mut context.polygons, p[0], p[1], p[2], p[3]);
            context.render_polygons(&constants::BLUE);
        }

        "torus" => {
            let p = get_next_params::<f32>(iterator, path, "torus", 5)?;
            add_torus(&mut context.polygons, p[0], p[1], p[2], p[3], p[4]);
            context.render_polygons(&constants::GREEN);
        }

        unknown => println!("{}:{} -> unknown command '{}'.", path, line_number, unknown),
    }

    Ok(())
}

fn handle_mesh(
    context: &mut ScriptContext,
    iterator: &mut impl Iterator<Item = (usize, String)>,
    path: &str,
    line_number: usize,
) -> Result<(), Box<dyn Error>> {
    let filename = get_next_param(iterator, path, "mesh", "<filepath>")?;

    let lines = read_lines(&filename)
        .map_err(|_| format!("{}:{} -> mesh file '{}' not found", path, line_number + 1, filename))?;

    let extension = Path::new(&filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if extension != "obj" && extension != "stl" {
        return Err(format!("{}:{} -> extension '.{}' not supported", path, line_number + 1, extension).into());
    }

    let mut vertices: Vec<[f32; 3]> = vec![];
    for line in lines.map_while(Result::ok) {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line.starts_with("v ") || line.starts_with("vertex ") {
            vertices.push([
                convert_parameter::<f32>(parts[1], path, line_number)?,
                convert_parameter::<f32>(parts[2], path, line_number)?,
                convert_parameter::<f32>(parts[3], path, line_number)?,
            ]);
        } else if line.starts_with("f ") {
            let a = convert_parameter::<usize>(parts[1], path, line_number)? - 1;
            let b = convert_parameter::<usize>(parts[2], path, line_number)? - 1;
            let c = convert_parameter::<usize>(parts[3], path, line_number)? - 1;

            add_polygon(
                &mut context.polygons,
                vertices[a][0], vertices[a][1], vertices[a][2],
                vertices[b][0], vertices[b][1], vertices[b][2],
                vertices[c][0], vertices[c][1], vertices[c][2],
            );
        }
    }

    if extension == "stl" {
        for polygon in vertices.chunks(3) {
            add_polygon(
                &mut context.polygons,
                polygon[0][0], polygon[0][1], polygon[0][2],
                polygon[1][0], polygon[1][1], polygon[1][2],
                polygon[2][0], polygon[2][1], polygon[2][2],
            );
        }
    }

    context.render_polygons(&constants::BLUE);
    Ok(())
}

fn get_next_param(
    iterator: &mut impl Iterator<Item = (usize, String)>,
    path: &str,
    cmd: &str,
    expected: &str,
) -> Result<String, Box<dyn Error>> {
    let (ln, args) = iterator.next().unwrap();
    let arg = args.trim();
    if arg.is_empty() {
        return Err(format!("{}:{} -> '{}' expected {}", path, ln + 1, cmd, expected).into());
    }
    Ok(arg.to_string())
}

fn get_next_params<T: std::str::FromStr>(
    iterator: &mut impl Iterator<Item = (usize, String)>,
    path: &str,
    cmd: &str,
    count: usize,
) -> Result<Vec<T>, Box<dyn Error>> {
    let (ln, args) = iterator.next().unwrap();

    let parts: Vec<&str> = args.split_whitespace().collect();

    if parts.len() < count {
        return Err(format!("{}:{} -> '{}' expected {} parameters", path, ln + 1, cmd, count).into());
    }

    convert_parameters(parts, path, ln + 1)
}

fn convert_parameters<T: std::str::FromStr>(
    parameters: Vec<&str>,
    path: &str,
    line_number: usize,
) -> Result<Vec<T>, Box<dyn Error>> {
    parameters
        .iter()
        .map(|p| convert_parameter::<T>(p, path, line_number))
        .collect()
}

fn convert_parameter<T: std::str::FromStr>(
    parameter: &str,
    path: &str,
    line_number: usize,
) -> Result<T, Box<dyn Error>> {
    parameter.parse::<T>().map_err(|_| {
        format!(
            "{}:{} -> invalid parameter: '{}'. expected {}.",
            path,
            line_number,
            parameter,
            std::any::type_name::<T>()
        )
        .into()
    })
}
