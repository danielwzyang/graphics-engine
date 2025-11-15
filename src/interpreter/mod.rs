mod lexer;
mod tokens;

use tokens::{TokenType, Function};
use std::{error::Error, collections::HashMap, sync::LazyLock};

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("display", TokenType::Command(Function::Display));
    map.insert("save", TokenType::Command(Function::Save));
    map.insert("clear", TokenType::Command(Function::Clear));

    map.insert("push", TokenType::Command(Function::Push));
    map.insert("pop", TokenType::Command(Function::Pop));
    
    map.insert("move", TokenType::Command(Function::Move));
    map.insert("scale", TokenType::Command(Function::Scale));
    map.insert("rotate", TokenType::Command(Function::Rotate));
    map.insert("x", TokenType::AxisOfRotation);
    map.insert("y", TokenType::AxisOfRotation);
    map.insert("z", TokenType::AxisOfRotation);
    
    map.insert("line", TokenType::Command(Function::Line));
    map.insert("circle", TokenType::Command(Function::Circle));
    map.insert("hermite", TokenType::Command(Function::Hermite));
    map.insert("bezier", TokenType::Command(Function::Bezier));

    map.insert("polygon", TokenType::Command(Function::Polygon));
    map.insert("box", TokenType::Command(Function::Box));
    map.insert("sphere", TokenType::Command(Function::Sphere));
    map.insert("torus", TokenType::Command(Function::Torus));
    map.insert("mesh", TokenType::Command(Function::Mesh));

    map.insert("light", TokenType::Command(Function::Light));
    map.insert("ambient", TokenType::Command(Function::Ambient));
    map.insert("constants", TokenType::Command(Function::Constants));
    map.insert("shading", TokenType::Command(Function::Shading));

    map
});

pub fn run_script(path: &str) -> Result<(), Box<dyn Error>> {
    let tokens = lexer::tokenize(path, KEYWORDS.clone());

    println!("{:#?}", tokens);

    Ok(())
}
