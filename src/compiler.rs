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

fn get_punc_values<T>(punc: &ast::punctuated::Punctuated<T>) -> Vec<T> {
    punc.into_pairs().map(ast::Pair::value).collect()
}

pub struct Compiler {
    pub ast: ast::Ast,
    pub verbose: bool,
}

impl Compiler {
    pub fn into_program(&self) -> Program {
        self.document("Starting compilatiom.");
        let p = Program {
            lines: self.compile_ast(&self.ast),
        };
        self.document("Ended compilation.");
        p
    }

    pub fn from(ast: ast::Ast, verbose: bool) -> Self {
        Self { ast, verbose }
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
                self.document("Compiling event.");
                let mut contents: Vec<Block> = self.compile_block(func.body().block());

                let mut line: Line = Line::Event {
                    name: func.name().names().to_string(),
                    line: contents,
                };
                line
            }
            ast::Stmt::LocalFunction(func) => {
                self.document("Compiling function.");
                let mut contents: Vec<Block> = self.compile_block(func.body().block());

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

    fn compile_block(&self, block: &ast::Block) -> Vec<Block> {
        let mut res: Vec<Block> = vec![];

        for smt in block.stmts() {
            match smt {
                ast::Stmt::Assignment(asg) => {
                    res.push(self.compile_assignment(asg));
                }
                _ => {}
            };
        }

        res
    }

    fn compile_assignment(&self, assignment: &ast::Assignment) -> Block {
        let vars: Vec<ast::Var> = get_punc_values(assignment.variables());
        let exprs: Vec<ast::Expression> = get_punc_values(assignment.expressions());
    }

    fn compile_expression(&self, expression: &ast::Expression) -> block::Primitive {
        match expression {
            ast::Expression::Number(token) => {
                let Ok(tkn) = token.to_string().parse::<f64>() else {
                    panic!("parse float error")
                };

                block::Primitive::NumberValue(tkn)
            }
            _ => block::Primitive::NumberValue(0.0),
        }
    }

    fn compile_chest(&self, punc: ast::punctuated::Punctuated<ast::Parameter>) -> Chest {
        Chest { contents: vec![] }
    }
}
