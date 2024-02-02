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

    // Math
    Plus,
    Minus,
    Star,
    Div,
    Eq,

    Identifier(String),

    // Keywords
    If,
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
            Some(' ' | '\n' | '\t') =>  {
                self.input.next();
                return self.next_token();
            }
            Some('a'..='z') => return self.parse_identifier(),
            // Unknown tokens
            Some(_) | None => None,
        };
    }

    fn parse_identifier(&mut self) -> Option<Token> {
        let mut output = String::new();
        while let Some(chr) = self.input.peek() {
            if !chr.is_alphanumeric() { break; }

            output.push(self.input.next()?);
        }

        if output.eq_ignore_ascii_case("if") {
            return Some(Token::If);
        }

        return Some(Token::Identifier(output));
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
