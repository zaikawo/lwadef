mod block;
use block::{Block, BlockType, Chest, Line, Primitive, Program};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    names: Vec<String>,

    #[arg(short = 'v')]
    verbose: bool,
}

fn main() {
    println!("this compiled");

    let args = Args::parse();

    println!("{0}", args.names.len());
}

fn testcompile() {
    let m = Line::Event {
        name: "Join".to_string(),
        line: vec![Block {
            block: BlockType::PlayerAction,
            data: "SendMessage".to_string(),
            args: Chest {
                contents: vec![Primitive::NumberValue(64.3)],
            },
        }],
    };

    let p = Program { lines: vec![m] };

    println!("{}", p.compile().to_string());
}
