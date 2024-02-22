use crate::lexer::tokens::Token;

#[derive(Debug)]
pub enum ASTNode<'a> {
    //  name   , type   , token
    Let(&'a str, &'a str, Token<'a>),
}
