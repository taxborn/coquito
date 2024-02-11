pub mod state;
pub mod tokens;

use crate::lexer::state::Lexer;
use crate::lexer::tokens::Token;

/// Checking if a given character is a whitespace character. Currently this
/// this only checks '\r' and '\t', however there is a more exhaustive list
/// in the Rust lexer, which I might be able to update this to base off of.
fn is_whitespace(chr: char) -> bool {
    matches!(chr, ' ' | '\r' | '\t')
}

/// Checking if a given character is valid for identifiers. Currently according
/// to the grammar, this is all letters lowercase and uppercase, numbers, and
/// underscores.
fn is_valid_id(chr: char) -> bool {
    chr.is_alphanumeric() || matches!(chr, '_')
}

/// Checking if a given character is valid for the *start* of identifiers.
/// Currently according to the grammar, this is all letters lowercase and
/// uppercase, and underscores. We do not allow for numbers to be the start
/// because that can cause issues with parsing of actual numbers.
fn is_valid_id_start(chr: char) -> bool {
    chr.is_alphabetic() || matches!(chr, '_')
}

impl<'a> Lexer<'a> {
    pub fn lex_token(&mut self) -> Option<Token<'a>> {
        let chr = self.lookahead.peek()?;

        return match chr {
            '(' => self.single_token(Token::LParen),
            ')' => self.single_token(Token::RParen),
            '[' => self.single_token(Token::LBracket),
            ']' => self.single_token(Token::RBracket),
            '{' => self.single_token(Token::LBrace),
            '}' => self.single_token(Token::RBrace),
            '=' => self.single_token(Token::Eq),
            ':' => self.single_token(Token::Colon),
            ';' => self.single_token(Token::Semi),
            '$' => self.single_token(Token::Dollar),
            '.' => self.single_token(Token::Period),
            ',' => self.single_token(Token::Comma),
            '~' => self.single_token(Token::Tilde),
            '+' => self.single_token(Token::Plus),
            '-' => self.single_token(Token::Minus),
            '*' => self.single_token(Token::Astrisk),
            '/' => self.single_token(Token::Slash),
            '%' => self.single_token(Token::Percent),
            '&' => self.single_token(Token::Ampsersand),
            '|' => self.single_token(Token::Pipe),
            '^' => self.single_token(Token::Caret),
            '>' => self.single_token(Token::GreaterThan),
            '<' => self.single_token(Token::LessThan),
            c if is_valid_id_start(*c) => {
                Some(Token::Identifier(self.accumulate_while(&is_valid_id)))
            }
            c if is_whitespace(*c) => {
                self.accumulate_while(&is_whitespace);
                self.lex_token()
            }
            '\n' => {
                self.accumulate_while(&|x| matches!(x, '\n' | '\r'));
                self.lex_token()
            }
            _ => None,
        };
    }

    fn single_token(&mut self, token: Token<'a>) -> Option<Token<'a>> {
        self.next_char();
        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_characters() {
        let input = "+-=";
        let mut lx = Lexer::new(input);
        let token = lx.lex_token().unwrap();

        assert_eq!(token, Token::Plus);
        assert_eq!(lx.input, "-=");
        assert_eq!(lx.position, 1);
    }

    #[test]
    fn test_lexing_identifiers() {
        let input = "func main()";
        let lx = Lexer::new(input);

        let toks: Vec<Token> = lx.collect();
        let expected = vec![Token::Identifier("func"), Token::Identifier("main"), Token::LParen, Token::RParen];
        assert_eq!(toks, expected);
    }
}
