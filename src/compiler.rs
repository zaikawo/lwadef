use full_moon::{ast::Ast, node::Node, parse};

use std::fs::read_to_string;
use std::path::Path;

pub fn parse_file(file: &str) -> Ast {
    let path: Path = Path::new(file);

    let Ok(code) = read_to_string(path) else {
        panic!("Path is invalid")
    }

    let Ok(ast) = parse(code) else {
        panic!("Code is malformed or you fuucking suck");
    }

    return ast
}
