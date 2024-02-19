use std::path::PathBuf;

use anyhow::Result;
use lexer::state::Lexer;
use parser::state::Parser;
use clap::Parser as ClapParser;

pub mod lexer;
pub mod parser;

#[derive(ClapParser)]
#[command(version, about)]
struct Args {
    /// The file to compile (file.cqo)
    #[arg(short, long)]
    file: PathBuf,

    /// The debug level (0: none (default), 1: some, 2: verbose)
    // TODO: Make this useful
    #[arg(short, long)]
    debug: u8
}

fn main() -> Result<()> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(args.file)?;
    let lexer = Lexer::new(&contents);
    let mut parser = Parser::new(lexer);

    while let Some(parsed) = parser.parse_term() {
        // println!("{:?}", parsed);
    }

    Ok(())
}
