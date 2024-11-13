use crate::block;
use block::{Block, Chest, Line, Program};

use full_moon::ast;

use std::fs::read_to_string;
use std::path::Path;

pub fn parse_file(file: &str) -> ast::Ast {
    let path: &Path = Path::new(file);

    let Ok(code) = read_to_string(&path) else {
        panic!("Path is invalid")
    };

    println!("-- IN: --\n{}", code);

    let Ok(ast) = full_moon::parse(&code[..]) else {
        panic!("Code is malformed or you fuucking suck");
    };

    return ast;
}

pub struct Compiler {
    pub ast: ast::Ast,
    pub verbose: bool,
}

impl Compiler {
    pub fn into_program(&self) -> Program {
        Program {
            lines: self.compile_ast(&self.ast),
        }
    }

    pub fn from(ast: ast::Ast, ver: bool) -> Self {
        Self { ast, verbose: ver }
    }

    fn document(&self, doc: &str) {
        if self.verbose {
            println!("{}", doc);
        }
    }

    fn compile_ast(&self, ast: &ast::Ast) -> Vec<Line> {
        let mut lines: Vec<Line> = vec![];

        let block = ast.nodes();

        for statement in block.stmts() {
            lines.push(self.compile_line(statement));
        }

        lines
    }

    fn compile_line(&self, statement: &ast::Stmt) -> Line {
        match statement {
            ast::Stmt::FunctionDeclaration(func) => {
                let mut contents: Vec<Block> = vec![];

                let mut line: Line = Line::Event {
                    name: func.name().names().to_string(),
                    line: contents,
                };
                line
            }
            ast::Stmt::LocalFunction(func) => {
                let mut contents: Vec<Block> = vec![];

                let mut line: Line = Line::Function {
                    name: func.name().to_string(),
                    args: Chest { contents: vec![] },
                    line: contents,
                };
                line
            }
            _ => Line::Event {
                name: String::from("cock"),
                line: vec![],
            },
        }
    }
}
