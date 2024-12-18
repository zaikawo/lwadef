#![allow(dead_code)]
mod block;
use block::{Block, BlockType, Chest, Line, Primitive, Program};

mod compiler;
use compiler::{parse_file, Compiler};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    names: String,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Shows compilation documentation."
    )]
    verbose: bool,
}

fn main() {
    println!("this compiled");

    let args = Args::parse();

    let out = parse_file(&args.names);

    let compiler = Compiler::from(out, args.verbose);

    let ln = &compiler.program();

    let cork = ln.compile();

    println!("-- OUT: --\n");

    for cor in cork {
        println!("{}\n", cor);
    }
}
