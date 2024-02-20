use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as ClapParser;
use clap_verbosity_flag::Verbosity;
use lexer::state::Lexer;
use parser::state::Parser;

pub mod lexer;
pub mod parser;
pub mod utils;

#[derive(ClapParser)]
#[command(version, about)]
struct Args {
    /// The file to compile (file.cqo)
    #[arg(short, long)]
    file: PathBuf,

    // TODO: Make this useful
    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let contents = std::fs::read_to_string(args.file)?;
    let lexer = Lexer::new_with_debug(&contents, args.verbose.clone());
    let mut parser = Parser::new_with_debug(lexer, args.verbose.clone());

    while let Some(_parsed) = parser.parse_term() {
        // println!("{:?}", parsed);
    }

    Ok(())
}
