#![allow(dead_code)]

use std::error::Error;
use std::collections::VecDeque;

use crate::{constants::ShadingMode, matrix::Rotation};

use super::tokens::{Token, TokenType, Function};

// file paths +  identifiers stored as String
#[derive(Debug)]
pub enum Command {
    Display,
    Save { file_path: String },
    Clear,
    Push,
    Pop,
    Move { a: f32, b: f32, c: f32, knob: Option<String> },
    Scale { a: f32, b: f32, c: f32, knob: Option<String> },
    Rotate { axis: Rotation, degrees: f32, knob: Option<String> },
    Line {  x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32 },
    Circle { x: f32, y: f32, z: f32, r: f32 },
    Hermite { x0: f32, y0: f32, x1: f32, y1: f32, rx0: f32, ry0: f32, rx1: f32, ry1: f32 },
    Bezier { x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32 },
    Polygon { x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32 },
    Box { constants: Option<String>, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32 },
    Sphere { constants: Option<String>, x: f32, y: f32, z: f32, r: f32 },
    Torus { constants: Option<String>, x: f32, y: f32, z: f32, r0: f32, r1: f32 },
    Mesh { constants: Option<String>, file_path: String },
    SetLight { r: f32, g: f32, b: f32, x: f32, y: f32, z: f32 },
    SetAmbient { r: f32, g: f32, b: f32 },
    SetConstants { name: String, kar: f32, kdr: f32, ksr: f32, kag: f32, kdg: f32, ksg: f32, kab: f32, kdb: f32, ksb: f32 },
    SetShading { shading_mode: ShadingMode },
}

pub struct Parser {
    stack: VecDeque<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self { stack: VecDeque::new() }
    }

    fn pop_optional_identifier(&mut self) -> Option<String> {
        if let Some(token) = self.stack.front() && token.token_type == TokenType::Identifier {
            let token = self.stack.pop_front().unwrap();
            return Some(token.value.clone())
        }

        None
    }
    
    fn pop(&mut self) -> Result<Token, Box<dyn Error>> {
        if let Some(token) = self.stack.pop_front() {
            Ok(token)
        } else {
            Err("Expected token but stack was empty.".into())
        }
    }

    pub fn create_syntax_tree(&mut self, tokens: VecDeque<Token>) -> Result<Vec<Command>, Box<dyn Error>> {
        let mut commands: Vec<Command> = vec![];

        self.stack = tokens;

        while !self.stack.is_empty() {
            let token = self.pop()?;

            match token.token_type {
                TokenType::Command(function) => {
                    match function {
                        Function::Display => { commands.push(Command::Display) }
                        Function::Save => { commands.push(self.handle_save()?) }
                        Function::Clear => { commands.push(Command::Clear) }
                        Function::Push => { commands.push(Command::Push) }
                        Function::Pop => { commands.push(Command::Pop) }
                        Function::Move => { commands.push(self.handle_move()?) }
                        Function::Scale => { commands.push(self.handle_scale()?) }
                        Function::Rotate => { commands.push(self.handle_rotate()?) }
                        Function::Line => { commands.push(self.handle_line()?) }
                        Function::Circle => { commands.push(self.handle_circle()?) }
                        Function::Hermite => { commands.push(self.handle_hermite()?) }
                        Function::Bezier => { commands.push(self.handle_bezier()?) }
                        Function::Polygon => { commands.push(self.handle_polygon()?) }
                        Function::Box => { commands.push(self.handle_box()?) }
                        Function::Sphere => { commands.push(self.handle_sphere()?) }
                        Function::Torus => { commands.push(self.handle_torus()?) }
                        Function::Mesh => { commands.push(self.handle_mesh()?) }
                        Function::SetLight => { commands.push(self.handle_set_light()?) }
                        Function::SetAmbient => { commands.push(self.handle_set_ambient()?) }
                        Function::SetConstants => { commands.push(self.handle_set_constants()?) }
                        Function::SetShading => { commands.push(self.handle_set_shading()?) }
                    }
                }

                _ => {
                    return Err(format!("Unexpected token: {:?}", token).into())
                }
            }
        }

        Ok(commands)
    }

    fn handle_save(&mut self) -> Result<Command, Box<dyn Error>> {
        let file_path = self.pop()?.value;

        Ok(Command::Save { file_path })
    }

    fn handle_move(&mut self) -> Result<Command, Box<dyn Error>> {
        let a = Parser::convert_parameter(self.pop()?.value)?;
        let b = Parser::convert_parameter(self.pop()?.value)?;
        let c = Parser::convert_parameter(self.pop()?.value)?;
        let knob = self.pop_optional_identifier();

        Ok(Command::Move { a, b, c, knob })
    }

    fn handle_scale(&mut self) -> Result<Command, Box<dyn Error>> {
        let a = Parser::convert_parameter(self.pop()?.value)?;
        let b = Parser::convert_parameter(self.pop()?.value)?;
        let c = Parser::convert_parameter(self.pop()?.value)?;
        let knob = self.pop_optional_identifier();

        Ok(Command::Scale { a, b, c, knob })
    }

    fn handle_rotate(&mut self) -> Result<Command, Box<dyn Error>> {
        let axis_str = self.pop()?.value.to_lowercase();
        let axis = match axis_str.as_str() {
            "x" => Rotation::X,
            "y" => Rotation::Y,
            "z" => Rotation::Z,
            _ => return Err(format!("Invalid rotation axis: {}", axis_str).into()),
        };
        let degrees = Parser::convert_parameter(self.pop()?.value)?;
        let knob = self.pop_optional_identifier();

        Ok(Command::Rotate { axis, degrees, knob })
    }

    fn handle_line(&mut self) -> Result<Command, Box<dyn Error>> {
        let x0 = Parser::convert_parameter(self.pop()?.value)?;
        let y0 = Parser::convert_parameter(self.pop()?.value)?;
        let z0 = Parser::convert_parameter(self.pop()?.value)?;
        let x1 = Parser::convert_parameter(self.pop()?.value)?;
        let y1 = Parser::convert_parameter(self.pop()?.value)?;
        let z1 = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Line { x0, y0, z0, x1, y1, z1 })
    }

    fn handle_circle(&mut self) -> Result<Command, Box<dyn Error>> {
        let x = Parser::convert_parameter(self.pop()?.value)?;
        let y = Parser::convert_parameter(self.pop()?.value)?;
        let z = Parser::convert_parameter(self.pop()?.value)?;
        let r = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Circle { x, y, z, r })
    }

    fn handle_hermite(&mut self) -> Result<Command, Box<dyn Error>> {
        let x0 = Parser::convert_parameter(self.pop()?.value)?;
        let y0 = Parser::convert_parameter(self.pop()?.value)?;
        let x1 = Parser::convert_parameter(self.pop()?.value)?;
        let y1 = Parser::convert_parameter(self.pop()?.value)?;
        let rx0 = Parser::convert_parameter(self.pop()?.value)?;
        let ry0 = Parser::convert_parameter(self.pop()?.value)?;
        let rx1 = Parser::convert_parameter(self.pop()?.value)?;
        let ry1 = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Hermite { x0, y0, x1, y1, rx0, ry0, rx1, ry1 })
    }

    fn handle_bezier(&mut self) -> Result<Command, Box<dyn Error>> {
        let x0 = Parser::convert_parameter(self.pop()?.value)?;
        let y0 = Parser::convert_parameter(self.pop()?.value)?;
        let x1 = Parser::convert_parameter(self.pop()?.value)?;
        let y1 = Parser::convert_parameter(self.pop()?.value)?;
        let x2 = Parser::convert_parameter(self.pop()?.value)?;
        let y2 = Parser::convert_parameter(self.pop()?.value)?;
        let x3 = Parser::convert_parameter(self.pop()?.value)?;
        let y3 = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Bezier { x0, y0, x1, y1, x2, y2, x3, y3 })
    }

    fn handle_polygon(&mut self) -> Result<Command, Box<dyn Error>> {
        let x0 = Parser::convert_parameter(self.pop()?.value)?;
        let y0 = Parser::convert_parameter(self.pop()?.value)?;
        let z0 = Parser::convert_parameter(self.pop()?.value)?;
        let x1 = Parser::convert_parameter(self.pop()?.value)?;
        let y1 = Parser::convert_parameter(self.pop()?.value)?;
        let z1 = Parser::convert_parameter(self.pop()?.value)?;
        let x2 = Parser::convert_parameter(self.pop()?.value)?;
        let y2 = Parser::convert_parameter(self.pop()?.value)?;
        let z2 = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Polygon { x0, y0, z0, x1, y1, z1, x2, y2, z2 })
    }

    fn handle_box(&mut self) -> Result<Command, Box<dyn Error>> {
        let constants = self.pop_optional_identifier();
        let x = Parser::convert_parameter(self.pop()?.value)?;
        let y = Parser::convert_parameter(self.pop()?.value)?;
        let z = Parser::convert_parameter(self.pop()?.value)?;
        let w = Parser::convert_parameter(self.pop()?.value)?;
        let h = Parser::convert_parameter(self.pop()?.value)?;
        let d = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Box { constants, x, y, z, w, h, d })
    }

    fn handle_sphere(&mut self) -> Result<Command, Box<dyn Error>> {
        let constants = self.pop_optional_identifier();
        let x = Parser::convert_parameter(self.pop()?.value)?;
        let y = Parser::convert_parameter(self.pop()?.value)?;
        let z = Parser::convert_parameter(self.pop()?.value)?;
        let r = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Sphere { constants, x, y, z, r })
    }

    fn handle_torus(&mut self) -> Result<Command, Box<dyn Error>> {
        let constants = self.pop_optional_identifier();
        let x = Parser::convert_parameter(self.pop()?.value)?;
        let y = Parser::convert_parameter(self.pop()?.value)?;
        let z = Parser::convert_parameter(self.pop()?.value)?;
        let r0 = Parser::convert_parameter(self.pop()?.value)?;
        let r1 = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::Torus { constants, x, y, z, r0, r1 })
    }

    fn handle_mesh(&mut self) -> Result<Command, Box<dyn Error>> {
        let constants = self.pop_optional_identifier();
        let file_path = self.pop()?.value;

        Ok(Command::Mesh { constants, file_path })
    }

    fn handle_set_light(&mut self) -> Result<Command, Box<dyn Error>> {
        let r = Parser::convert_parameter(self.pop()?.value)?;
        let g = Parser::convert_parameter(self.pop()?.value)?;
        let b = Parser::convert_parameter(self.pop()?.value)?;
        let x = Parser::convert_parameter(self.pop()?.value)?;
        let y = Parser::convert_parameter(self.pop()?.value)?;
        let z = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::SetLight { r, g, b, x, y, z })
    }

    fn handle_set_ambient(&mut self) -> Result<Command, Box<dyn Error>> {
        let r = Parser::convert_parameter(self.pop()?.value)?;
        let g = Parser::convert_parameter(self.pop()?.value)?;
        let b = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::SetAmbient { r, g, b })
    }

    fn handle_set_constants(&mut self) -> Result<Command, Box<dyn Error>> {
        let name = self.pop()?.value;
        let kar = Parser::convert_parameter(self.pop()?.value)?;
        let kdr = Parser::convert_parameter(self.pop()?.value)?;
        let ksr = Parser::convert_parameter(self.pop()?.value)?;
        let kag = Parser::convert_parameter(self.pop()?.value)?;
        let kdg = Parser::convert_parameter(self.pop()?.value)?;
        let ksg = Parser::convert_parameter(self.pop()?.value)?;
        let kab = Parser::convert_parameter(self.pop()?.value)?;
        let kdb = Parser::convert_parameter(self.pop()?.value)?;
        let ksb = Parser::convert_parameter(self.pop()?.value)?;

        Ok(Command::SetConstants { name, kar, kdr, ksr, kag, kdg, ksg, kab, kdb, ksb })
    }

    fn handle_set_shading(&mut self) -> Result<Command, Box<dyn Error>> {
        let mode_str = self.pop()?.value.to_lowercase();
        let shading_mode = match mode_str.as_str() {
            "flat" => ShadingMode::Flat,
            "gouraud" => ShadingMode::Gouraud,
            "phong" => ShadingMode::Phong,
            _ => return Err(format!("Invalid shading mode: {}", mode_str).into()),
        };

        Ok(Command::SetShading { shading_mode })
    }

    fn convert_parameter(parameter: String) -> Result<f32, Box<dyn Error>> {
        Ok(parameter.parse().expect(format!("Error parsing float: {}", parameter).as_str()))
    }
}