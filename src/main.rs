use anyhow::Result;
use lexer::state::Lexer;
use parser::state::Parser;

pub mod lexer;
pub mod parser;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("examples/simple.cqo")?;
    let lexer = Lexer::new(&contents);
    let parser = Parser::new(lexer.collect());

    parser.parse();

    Ok(())
}
