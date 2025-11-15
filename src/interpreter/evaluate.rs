#![allow(dead_code)]

use std::{
    collections::HashMap,
    error::Error,
    path::Path,
};

use crate::{
    constants::{DEFAULT_BACKGROUND_COLOR, DEFAULT_FOREGROUND_COLOR, DEFAULT_LIGHTING_CONFIG, DEFAULT_PICTURE_DIMENSIONS, DEFAULT_REFLECTION_CONSTANTS, DEFAULT_SHADING_MODE, ShadingMode},
    matrix,
    render::{self, LightingConfig, Picture, ReflectionConstants},
};
use super::{
    coordinate_stack::CoordinateStack,
    parser::Command,
    read_lines,
};
use render::{
    edge_list::{add_bezier_curve, add_circle, add_edge, add_hermite_curve, render_edges},
    polygon_list::{add_box, add_polygon, add_sphere, add_torus, render_polygons},
};

type Matrix = Vec<[f32; 4]>;

enum Symbol {
    Constants(ReflectionConstants),
    Knob
}

struct ScriptContext {
    picture: Picture,
    edges: Matrix,
    polygons: Matrix,
    coordinate_stack: CoordinateStack,
    shading_mode: ShadingMode,
    lighting_config: LightingConfig,
    reflection_constants: ReflectionConstants,
    symbols: HashMap<String, Symbol>,
}

impl ScriptContext {
    fn new() -> Self {
        let (xres, yres) = DEFAULT_PICTURE_DIMENSIONS;

        Self {
            picture: Picture::new(xres, yres, 255, &DEFAULT_BACKGROUND_COLOR),
            edges: matrix::new(),
            polygons: matrix::new(),
            coordinate_stack: CoordinateStack::new(),
            shading_mode: DEFAULT_SHADING_MODE,
            lighting_config: DEFAULT_LIGHTING_CONFIG,
            reflection_constants: DEFAULT_REFLECTION_CONSTANTS,
            symbols: HashMap::new(),
        }
    }

    fn render_edges(&mut self) {
        matrix::multiply(&self.coordinate_stack.peek(), &mut self.edges);
        render_edges(&self.edges, &mut self.picture, &DEFAULT_FOREGROUND_COLOR);
        self.edges = matrix::new();
    }

    fn render_polygons(&mut self, constants: &Option<String>) -> Result<(), Box<dyn Error>> {
        let mut reflection_constants = &self.reflection_constants;

        if let Some(name) = constants && let Some(symbol) = self.symbols.get(name) {
            match symbol {
                Symbol::Constants(constants) => reflection_constants = constants,
                _ => return Err(format!("Expected symbol to be lighting constants: {}", name).into())
            }
        }
        
        crate::matrix::multiply(&self.coordinate_stack.peek(), &mut self.polygons);
        render_polygons(&self.polygons, &mut self.picture, &DEFAULT_FOREGROUND_COLOR, &self.shading_mode, &self.lighting_config, reflection_constants);
        self.polygons = matrix::new();

        Ok(())
    }
}

pub fn evaluate_syntax_tree(syntax_tree: Vec<Command>) -> Result<(), Box<dyn Error>> {
    let mut context = ScriptContext::new();

    for command in syntax_tree {
        match command {
            Command::Display => {
                context.picture.display()?
            }

            Command::Save { file_path } => {
                context.picture.save_as_file(&file_path)?
            }

            Command::Clear => {
                context.picture.clear();
            }

            Command::Push => {
                context.coordinate_stack.push();
            }

            Command::Pop => {
                context.coordinate_stack.pop();
            }

            Command::Move { a, b, c, knob: _ } => {
                context.coordinate_stack.apply_transformation(matrix::translation(a, b, c));
            }

            Command::Scale { a, b, c, knob: _ } => {
                context.coordinate_stack.apply_transformation(matrix::dilation(a, b, c));
            }

            Command::Rotate { axis, degrees, knob: _ } => {
                context.coordinate_stack.apply_transformation(matrix::rotation(axis, degrees));
            }

            Command::Line { x0, y0, z0, x1, y1, z1 } => {
                add_edge(&mut context.edges, x0, y0, z0, x1, y1, z1);
                context.render_edges();
            }

            Command::Circle { x, y, z, r } => {
                add_circle(&mut context.edges, x, y, z, r);
                context.render_edges();
            }

            Command::Hermite { x0, y0, x1, y1, rx0, ry0, rx1, ry1 } => {
                add_hermite_curve(&mut context.edges, x0, y0, x1, y1, rx0, ry0, rx1, ry1);
                context.render_edges();
            }

            Command::Bezier { x0, y0, x1, y1, x2, y2, x3, y3 } => {
                add_bezier_curve(&mut context.edges, x0, y0, x1, y1, x2, y2, x3, y3);
                context.render_edges();
            }

            Command::Polygon { x0, y0, z0, x1, y1, z1, x2, y2, z2 } => {
                add_polygon(&mut context.polygons, x0, y0, z0, x1, y1, z1, x2, y2, z2);
                context.render_polygons(&None)?;
            }

            Command::Box { constants, x, y, z, w, h, d } => {
                add_box(&mut context.polygons, x, y, z, w, h, d);
                context.render_polygons(&constants)?;
            }

            Command::Sphere { constants, x, y, z, r } => {
                add_sphere(&mut context.polygons, x, y, z, r);
                context.render_polygons(&constants)?;
            }

            Command::Torus { constants, x, y, z, r0, r1 } => {
                add_torus(&mut context.polygons, x, y, z, r0, r1);
                context.render_polygons(&constants)?;
            }

            Command::Mesh { constants, file_path } => {
                handle_mesh(&mut context, &constants, file_path)?;
                context.render_polygons(&constants)?;
            }

            Command::SetLight { r, g, b, x, y, z } => {
                context.lighting_config.point_light_color = [r, g, b];
                context.lighting_config.point_light_vector = [x, y, z];
            }

            Command::SetAmbient { r, g, b } => {
                context.lighting_config.ambient_light_color = [r, g, b];
            }

            Command::SetConstants { name, kar, kdr, ksr, kag, kdg, ksg, kab, kdb, ksb } => {
                let constants = ReflectionConstants {
                    ambient: [kar, kag, kab],
                    diffuse: [kdr, kdg, kdb],
                    specular: [ksr, ksg, ksb],
                };

                context.symbols.insert(name, Symbol::Constants(constants));
            }

            Command::SetShading { shading_mode } => {
                context.shading_mode = shading_mode.clone();
            }
        }
    }

    Ok(())
}

fn handle_mesh(
    context: &mut ScriptContext,
    constants: &Option<String>,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let lines = read_lines(&path).map_err(|_| format!("Mesh file '{}' not found", path))?;

    let extension = Path::new(&path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    if extension != "obj" && extension != "stl" {
        return Err(format!("Mesh file extension '.{}' not supported", path).into());
    }

    let mut vertices: Vec<[f32; 3]> = vec![];
    for line in lines.map_while(Result::ok) {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line.starts_with("v ") || line.starts_with("vertex ") {
            vertices.push([parts[1].parse::<f32>()?, parts[2].parse::<f32>()?, parts[3].parse::<f32>()?]);
        } else if line.starts_with("f ") {
            let a = parts[1].parse::<usize>()? - 1;
            let b = parts[2].parse::<usize>()? - 1;
            let c = parts[3].parse::<usize>()? - 1;

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

    context.render_polygons(constants)?;
    Ok(())
}