use crate::lexer::state::Lexer;
use anyhow::Result;

pub mod lexer;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("examples/test.cqo")?;
    let mut lexer = Lexer::new(&contents);

    while let Some(token) = lexer.lex_token() {
        println!("{token:?}");
    }

    Ok(())
}
