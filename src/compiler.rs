use crate::block;
use block::{Block, Line};

use full_moon::{ast};

use std::fs::read_to_string;
use std::path::Path;

pub fn parse_file(file: &str) -> Ast {
    let path: &Path = Path::new(file);

    let Ok(code) = read_to_string(&path) else {
        panic!("Path is invalid")
    };

    let Ok(ast) = full_moon::parse(&code[..]) else {
        panic!("Code is malformed or you fuucking suck");
    };

    return ast;
}

pub struct Compiler {
    ast: ast::Ast,
    verbose: bool,
}

impl Compiler {
    pub fn into_lines(&self) -> Vec<Line> {
        self.compile_ast(&self.ast)
    }

    fn compile_ast(&self, ast: &ast::Ast) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];

        let block:  = ast.nodes();

        for statement in block.stmts() {
            lines.push(self.compile_line(statement));
        }

        lines
    }

    fn compile_line(&self, statement: &ast::Stmt) -> Line {
        match statement {
            ast::Stmt::FunctionDeclaration(func) => {
                
            }
        }
    }
}
