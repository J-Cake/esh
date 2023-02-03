use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum SyntaxError {
    BracketMismatch(i64, i64),
    UnexpectedToken(String, i64, i64),
    InvalidSyntax(i64, i64),
    UnexpectedEOF(),
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::BracketMismatch(line, col) => write!(f, "SyntaxError: Bracket Mismatch at {}:{}", line, col),
            SyntaxError::UnexpectedToken(lexeme, line, col) => write!(f, "SyntaxError: Unexpected Token '{}' at {}:{}", lexeme, line, col),
            SyntaxError::UnexpectedEOF() => write!(f, "SyntaxError: Unexpected EOF"),
            SyntaxError::InvalidSyntax(line, col) => write!(f, "SyntaxError: Invalid Syntax at {}:{}", line, col),
        }
    }
}

impl Error for SyntaxError {}
