use crate::lexer::{state::Lexer, tokens::Token};
use crate::parser::ast::ASTNode;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    /// Construct a new [`Parser`]. This consumes the [`Lexer`] (since
    /// we shouldn't need it after initial lexing)
    // TODO: We could make lexing lazy and not collect here, and instead
    // make the lexer a part of the parser struct.
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut toks: Vec<Token<'a>> = lexer.collect();
        // TODO: Bounds checks
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

    pub fn parse_term(&self) -> Option<ASTNode> {
        None
    }

    fn advance(&mut self) {
        self.current = if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }

    fn eat(&mut self, _expected: Token<'a>) -> Option<Token<'a>> {
        if let Some(tok) = self.tokens.first() {
            if matches!(tok, _expected) {
                self.advance();
                return Some(_expected);
            }
        }

        None
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
        let remaining = vec![Token::Identifier("_"), Token::Eq, Token::Number("5"), Token::Semi];
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
