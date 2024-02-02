use std::{iter::Peekable, str::Chars};

use anyhow::Result;

#[derive(Debug)]
struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

#[derive(Debug)]
enum Token {
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
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        if input.len() == 0 {
            panic!("Nothing to lex.");
        }

        Self {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if let None = self.input.peek() {
            return None;
        }

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
                return Some(Token::Div);
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
        let mut output = String::new();
        while let Some(chr) = self.input.peek() {
            if !chr.is_alphanumeric() {
                break;
            }

            output.push(self.input.next()?);
        }

        // TODO: Should these be hard tokens?
        if output.eq_ignore_ascii_case("if") {
            return Some(Token::If);
        } else if output.eq_ignore_ascii_case("else") {
            return Some(Token::Else);
        }

        return Some(Token::Identifier(output));
    }

    fn parse_number(&mut self) -> Option<Token> {
        let mut output = String::new();
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

            output.push(self.input.next()?);
        }

        return Some(Token::Number(output));
    }
}

fn main() -> Result<()> {
    let contents = std::fs::read_to_string("examples/test.cqo")?;
    let mut lexer = Lexer::new(&contents);

    while let Some(tok) = lexer.next_token() {
        println!("{tok:?}");
    }

    Ok(())
}
