use crate::lexer::tokens::Token;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse(&self) {
        for token in &self.tokens {
            println!("{token:?}");
        }
    }
}
