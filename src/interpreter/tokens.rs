#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Command(Function),
    AxisOfRotation,
    Number,
    FilePath,
    Identifier,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    Display,
    Save,
    Clear,
    Push,
    Pop,
    Move,
    Scale,
    Rotate,
    Line,
    Circle,
    Hermite,
    Bezier,
    Polygon,
    Box,
    Sphere,
    Torus,
    Mesh,
    SetLight,
    SetAmbient,
    SetConstants,
    SetShading,
}