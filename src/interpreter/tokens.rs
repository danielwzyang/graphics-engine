#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Command(Function),
    AxisOfRotation,
    Number,
    FilePath,
    Identifier,
}

#[derive(Clone, Copy, Debug)]
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
    Light,
    Ambient,
    Constants,
    Shading,
}