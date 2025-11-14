use std::{error::Error, io, io::BufRead, fs::File, path::Path};
use super::tokens::{Token, TokenType};
use regex::Regex;
use std::collections::HashMap;


pub fn tokenize(path: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    const keywords: HashMap<&str, TokenType> = HashMap::new();

    keywords.insert("constants", TokenType::Define);

    let tokens: Vec<Token> = vec![];

    let lines = read_lines(path).map_err(|_| format!("'{}' not found", path))?;

    let mut iterator = lines.map_while(Result::ok);

    while let Some(line) = iterator.next() {
        let line = line.trim();

        if line.starts_with("#") || line.starts_with("//") {
            continue;
        }

        let current = line.split_whitespace().collect::<Vec<&str>>();

        for token in current {

        }
    }

    Ok((tokens))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
