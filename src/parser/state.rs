use crate::lexer::{state::Lexer, tokens::Token};
use crate::parser::ast::ASTNode;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pub current: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    /// Construct a new [`Parser`]. This consumes the [`Lexer`] (since
    /// we shouldn't need it after initial lexing)
    // TODO: We could make lexing lazy and not collect here, and instead
    // make the lexer a part of the parser struct.
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut toks: Vec<Token<'a>> = lexer.collect();

        let first = if toks.is_empty() {
            None
        } else {
            Some(toks.remove(0))
        };

        Self {
            tokens: toks,
            current: first,
        }
    }

    /// Parse a term according to the `grammar.ebnf`.
    pub fn parse_term(&mut self) -> Option<ASTNode> {
        if self.eat(Token::Identifier("let")).is_some() {
            // println!("parsing let statement");
            return self.parse_let_statement();
        }

        // if self.eat(Token::Identifier("func")).is_some() {
        //     self.parse_function();
        // }

        None
    }

    /// Eat the next token that we expect the input to be.
    pub fn eat(&mut self, expected: Token<'a>) -> Option<Token<'a>> {
        if matches!(self.current.as_ref(), Some(tok) if tok == &expected) {
            self.advance();
            return Some(expected);
        }

        None
    }

    /// Helper function for the `eat()` method to ensure we advance the state
    pub fn advance(&mut self) {
        self.current = if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_parser() -> Parser<'static> {
        let inp = "let _ = 5;";
        let lx = Lexer::new(inp);

        Parser::new(lx)
    }

    #[test]
    fn test_parser_loads_correctly() {
        let parser = create_parser();

        assert_eq!(parser.current, Some(Token::Identifier("let")));
        let remaining = vec![
            Token::Identifier("_"),
            Token::Eq,
            Token::Number("5"),
            Token::Semi,
        ];
        assert_eq!(parser.tokens, remaining);
    }

    #[test]
    fn test_eating_advances() {
        let inp = "let _ = 5;";
        let lx = Lexer::new(inp);
        let mut parser = Parser::new(lx);

        let expected = Token::Identifier("let");
        parser.eat(expected);

        assert_eq!(parser.current, Some(Token::Identifier("_")));
        let remaining = vec![Token::Eq, Token::Number("5"), Token::Semi];
        assert_eq!(parser.tokens, remaining);
    }
}
