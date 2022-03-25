use std::fs;
use parser::parse;

use crate::execute::execute;

pub mod parser;
pub mod execute;

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();
    let file_content = fs::read_to_string(filename).expect("Something went wrong when reading file");
    let instructions = parse(&file_content).unwrap();
    let input = Box::new(std::io::stdin());
    let output = Box::new(std::io::stdout());
    execute(instructions, input, output);
}
