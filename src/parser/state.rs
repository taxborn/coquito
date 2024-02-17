use crate::lexer::{state::Lexer, tokens::Token};
use crate::parser::ast::ASTNode;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    /// Construct a new [`Parser`]. This consumes the [`Lexer`] (since
    /// we shouldn't need it after initial lexing)
    // TODO: We could make lexing lazy and not collect here, and instead
    // make the lexer a part of the parser struct.
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            tokens: lexer.collect(),
        }
    }

    pub fn parse(&self) {
        for token in &self.tokens {
            println!("{token:?}");
        }
    }

    fn parse_term(&mut self) -> Option<ASTNode> {
        None
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ASTNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_term()
    }
}
