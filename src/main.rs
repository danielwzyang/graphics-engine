mod constants;
mod picture;
mod matrix;
mod edge_list;
mod polygon_list;
mod parser;

use std::error::Error;
use std::env;
#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();

    let path = if arguments.len() < 2 {
        println!("A path to a script wasn't provided. '{}' was chosen by default.", &constants::DEFAULT_SCRIPT);
        &String::from(constants::DEFAULT_SCRIPT)
    } else { &arguments[1] };

    parser::read_script(path)
}
