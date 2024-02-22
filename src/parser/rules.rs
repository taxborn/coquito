use core::panic;

use clap_verbosity_flag::Level;

use crate::{lexer::tokens::Token, parser::state::Parser, utils::log};

use super::ast::ASTNode;

impl<'a> Parser<'a> {
    pub fn parse_let_statement(&mut self) -> Option<ASTNode> {
        // Extract the identifier name
        let id = if let Some(Token::Identifier(name)) = self.current.take() {
            // Use `take` to replace `self.current` with `None` and take its value
            self.advance(); // Move to the next token after the identifier
            name
        } else {
            panic!("Named identifier expected after `let`");
        };

        // Ensure the next token is an equals sign and advance
        assert!(
            self.eat(Token::Colon).is_some(),
            "Expected type assignment after identifier (e.g. ... : int = ...)"
        );

        let typ = if let Some(Token::Identifier(ty)) = self.current.take() {
            // Use `take` to replace `self.current` with `None` and take its value
            self.advance(); // Move to the next token after the identifier
            ty
        } else {
            panic!("type expected after `:`");
        };

        log(
            format!("  [-] type found \"{typ}\""),
            &self.verbose,
            Level::Debug,
        );

        assert!(
            self.eat(Token::Eq).is_some(),
            "Expected equals after type assignment"
        );

        // Parse the value expression
        let val = if let Some(token) = self.current.take() {
            self.advance(); // Move past the value token
            match token {
                Token::Identifier(_) | Token::Number(_) => token,
                Token::Semi => panic!("let assignment needs an assignment"),
                _ => panic!("Unexpected token type for let assignment"),
            }
        } else {
            panic!("Expected a value after '=' in let statement");
        };

        log(
            format!("  [-] parsed value \"{val:?}\""),
            &self.verbose,
            Level::Debug,
        );

        // Ensure the statement ends with a semicolon and advance
        assert!(
            self.eat(Token::Semi).is_some(),
            "Expected ';' at the end of let statement"
        );

        // Return the AST node with the identifier and value
        Some(ASTNode::Let(id, typ, val))
    }
}
