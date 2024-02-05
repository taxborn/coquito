use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // Symbols
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Semi,
    Comma,
    Colon,
    Underscore,

    // Math
    Add,
    Sub,
    Mul,
    Div,
    Eq,

    Identifier(String),
    Number(String),

    // Keywords
    If,
    Else,
    Return,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        if input.len() == 0 {
            panic!("Nothing to lex.");
        }

        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        return match self.input.peek() {
            Some('(') => self.lex_token(Token::LParen),
            Some(')') => self.lex_token(Token::RParen),
            Some('{') => self.lex_token(Token::LBracket),
            Some('}') => self.lex_token(Token::RBracket),
            Some('[') => self.lex_token(Token::LBrace),
            Some(']') => self.lex_token(Token::RBrace),
            Some('=') => self.lex_token(Token::Eq),
            Some(';') => self.lex_token(Token::Semi),
            Some(',') => self.lex_token(Token::Comma),
            Some(':') => self.lex_token(Token::Colon),
            Some('_') => {
                self.input.next();
                return match self.input.peek() {
                    Some(chr) if chr.is_alphanumeric() => {
                        return self.parse_identifier(true);
                    }
                    Some(_) | None => self.lex_token(Token::Underscore),
                };
            }
            Some('+') => self.lex_token(Token::Add),
            Some('-') => self.lex_token(Token::Sub),
            Some('*') => self.lex_token(Token::Mul),
            Some('/') => {
                self.input.next();

                return match self.input.peek() {
                    Some('/') => return self.parse_comment(),
                    Some(_) | None => self.lex_token(Token::Div),
                };
            }
            Some(chr) if chr.is_whitespace() => {
                self.input.next();
                return self.next_token();
            }
            Some(chr) if chr.is_alphabetic() => return self.parse_identifier(false),
            Some(chr) if chr.is_numeric() => return self.parse_number(),
            // Unknown tokens
            Some(_) | None => None,
        };
    }

    #[inline]
    fn lex_token(&mut self, token: Token) -> Option<Token> {
        let _ = self.input.next();
        return Some(token);
    }

    fn parse_identifier(&mut self, private: bool) -> Option<Token> {
        let mut ident = if private {
            String::from("_")
        } else {
            String::new()
        };

        while let Some(chr) = self.input.peek() {
            if !chr.is_alphanumeric() && !matches!(chr, '_') {
                break;
            }

            ident.push(self.input.next()?);
        }

        // TODO: Should these be hard tokens?
        if ident.eq_ignore_ascii_case("if") {
            return Some(Token::If);
        } else if ident.eq_ignore_ascii_case("else") {
            return Some(Token::Else);
        } else if ident.eq_ignore_ascii_case("return") {
            return Some(Token::Return);
        }

        return Some(Token::Identifier(ident));
    }

    fn parse_number(&mut self) -> Option<Token> {
        let mut number = String::new();
        let mut is_float = false;

        // TODO: There has gotta be a better way than this
        while let Some(chr) = self.input.peek() {
            if matches!(chr, '.') && is_float {
                panic!("Can't have two '.' in a numeric");
            }

            if !chr.is_numeric() && !matches!(chr, '.') {
                break;
            }

            if matches!(chr, '.') {
                is_float = true;
            }

            number.push(self.input.next()?);
        }

        return Some(Token::Number(number));
    }

    fn parse_comment(&mut self) -> Option<Token> {
        while let Some(chr) = self.input.peek() {
            if *chr == '\n' {
                break;
            }
            self.input.next();
        }

        return self.next_token();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return self.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        let lx = Lexer::new("a//this is a test\na");
        let toks: Vec<Token> = lx.collect();
        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Identifier("a".to_string()),
        ];

        assert_eq!(toks, expected);
    }

    #[test]
    fn test_identifier_parsing() {
        let lx = Lexer::new("func f_unc func_ _func");
        let toks: Vec<Token> = lx.collect();
        let expected = vec![
            Token::Identifier("func".to_string()),
            Token::Identifier("f_unc".to_string()),
            Token::Identifier("func_".to_string()),
            Token::Identifier("_func".to_string()),
        ];

        assert_eq!(toks, expected);
    }
}
