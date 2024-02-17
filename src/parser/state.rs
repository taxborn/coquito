use crate::lexer::{state::Lexer, tokens::Token};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { tokens: lexer.collect() }
    }

    pub fn parse(&self) {
        for token in &self.tokens {
            println!("{token:?}");
        }
    }
}
