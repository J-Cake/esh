use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::command::parser::ASTNode;

#[derive(Clone)]
pub enum SyntaxError {
    BracketMismatch(i64, i64),
    UnexpectedToken(String, i64, i64),
    InvalidSyntax(i64, i64),
    UnexpectedEOF(),
    UnsupportedExpression(ASTNode),
    NoValue(String)
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
            SyntaxError::InvalidSyntax(line, col) => write!(f, "SyntaxError: Invalid Syntax at {}:{}", line, col),
            SyntaxError::UnexpectedEOF() => write!(f, "SyntaxError: Unexpected EOF"),
            SyntaxError::UnsupportedExpression(node) => write!(f, "SyntaxError: Unsupported Expression: {:?}", node),
            SyntaxError::NoValue(val) => write!(f, "SyntaxError: Value '{}' does not exist in scope.", val)
        }
    }
}

impl Error for SyntaxError {}
