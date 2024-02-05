use anyhow::Result;
use lexer::{Token, Lexer};

pub mod lexer;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("examples/test.cqo")?;
    let lexer = Lexer::new(&contents);
    let toks: Vec<Token> = lexer.collect();

    println!("lexed {} tokens", toks.len());

    Ok(())
}
