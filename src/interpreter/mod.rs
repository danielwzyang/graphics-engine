mod lexer;
mod tokens;

use tokens::{Token, TokenType};
use std::{error::Error};

pub fn run_script(path: &str) -> Result<(), Box<dyn Error>> {
    let tokens = lexer::tokenize(path);

    Ok(())
}
