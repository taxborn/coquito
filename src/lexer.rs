use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

#[derive(Debug)]
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
            Some('(') => {
                self.input.next();
                return Some(Token::LParen);
            }
            Some(')') => {
                self.input.next();
                return Some(Token::RParen);
            }
            Some('{') => {
                self.input.next();
                return Some(Token::LBracket);
            }
            Some('}') => {
                self.input.next();
                return Some(Token::RBracket);
            }
            Some('[') => {
                self.input.next();
                return Some(Token::LBrace);
            }
            Some(']') => {
                self.input.next();
                return Some(Token::RBrace);
            }
            Some('=') => {
                self.input.next();
                return Some(Token::Eq);
            }
            Some(';') => {
                self.input.next();
                return Some(Token::Semi);
            }
            Some('+') => {
                self.input.next();
                return Some(Token::Add);
            }
            Some('-') => {
                self.input.next();
                return Some(Token::Sub);
            }
            Some('*') => {
                self.input.next();
                return Some(Token::Mul);
            }
            Some('/') => {
                self.input.next();

                return match self.input.peek() {
                    Some('/') => return self.parse_comment(),
                    Some(_) | None => Some(Token::Div)
                }
            }
            Some(',') => {
                self.input.next();
                return Some(Token::Comma);
            }
            Some(':') => {
                self.input.next();
                return Some(Token::Colon);
            }
            Some(chr) if chr.is_whitespace() => {
                self.input.next();
                return self.next_token();
            }
            Some(chr) if chr.is_alphabetic() => return self.parse_identifier(),
            Some(chr) if chr.is_numeric() => return self.parse_number(),
            // Unknown tokens
            Some(_) | None => None,
        };
    }

    fn parse_identifier(&mut self) -> Option<Token> {
        let mut ident = String::new();
        while let Some(chr) = self.input.peek() {
            if !chr.is_alphanumeric() {
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
