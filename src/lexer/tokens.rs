#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// {
    LBrace,
    /// }
    RBrace,
    /// !
    Bang,
    /// =
    Eq,
    /// :
    Colon,
    /// ;
    Semi,
    /// $
    Dollar,
    /// ,
    Comma,
    /// .
    Period,
    /// ~
    Tilde,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Astrisk,
    /// /
    Slash,
    /// %
    Percent,
    /// &
    Ampsersand,
    /// |
    Pipe,
    /// ^
    Caret,
    /// >
    GreaterThan,
    /// <
    LessThan,
    
    Identifier(&'a str),
}
