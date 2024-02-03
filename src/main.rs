use anyhow::Result;

pub mod lexer;

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("examples/test.cqo")?;
    let mut lexer = lexer::Lexer::new(&contents);

    while let Some(tok) = lexer.next_token() {
        println!("{tok:?}");
    }

    Ok(())
}
