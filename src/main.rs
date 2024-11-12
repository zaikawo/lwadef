mod block;
use block::{Block, BlockType, Chest, Line, Primitive, Program};

mod compiler;
use compiler::parse_file;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    names: String,

    #[arg(short = 'v')]
    verbose: bool,
}

fn main() {
    println!("this compiled");

    let args = Args::parse();

    let out = parse_file(&args.names);

    let ball = out.nodes();

    let stmts = ball.stmts();

    println!("");
}
