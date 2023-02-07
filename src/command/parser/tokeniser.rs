use std::cmp::{Ord, Ordering};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, Sub};

use lazy_static;

use crate::command::parser::matchers::Matcher;
use crate::command::parser::syntax_err::SyntaxError;

#[derive(Copy, Clone, Debug)]
pub enum PipeType {
    Stdout,
    Stderr,
    Both,
}

#[derive(Copy, Clone, Debug)]
pub enum OperatorType {
    Pipe(PipeType),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
    Not,
    Assign,
}

#[derive(Copy, Clone, Debug)]
pub enum KeywordType {
    If,
    Else,
    For,
    Function,
    Return,
    Import,
}

#[derive(Copy, Clone, Debug)]
pub enum BracketType {
    Parenthesis,
    Brace,
    Bracket,
    Angle,
}

#[derive(Clone, Debug)]
pub enum TokenType {
    Symbol(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Operator(OperatorType),
    Keyword(KeywordType),
    OpenBracket(BracketType),
    CloseBracket(BracketType),
    Colon,
    Semicolon,
    Comma,
    Dot,
    Lambda,
    Whitespace(String),
    Comment(String),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i64,
    pub column: i64,
    pub index: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", match self.token_type {
            TokenType::Symbol(_) => "Symbol",
            TokenType::String(_) => "String",
            TokenType::Number(_) => "Number",
            TokenType::Boolean(_) => "Boolean",
            TokenType::Operator(_) => "Operator",
            TokenType::Keyword(_) => "Keyword",
            TokenType::OpenBracket(_) => "OpenBracket",
            TokenType::CloseBracket(_) => "CloseBracket",
            TokenType::Colon => "Colon",
            TokenType::Semicolon => "Semicolon",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Lambda => "Lambda",
            TokenType::Whitespace(_) => "Whitespace",
            TokenType::Comment(_) => "Comment",
        }, self.lexeme)
    }
}

pub fn tokenise(input: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();

    let matcher = Matcher::new();

    let mut index: usize = 0;

    while index < input.len() {
        if let Some((lexeme, r#type)) = matcher.match_all(&input[index..]) {
            match r#type {
                TokenType::Whitespace(_) => {}
                TokenType::Comment(_) => {}
                _ => tokens.push(Token {
                    token_type: r#type,
                    lexeme: lexeme.to_owned(),
                    column: input[..=index].split('\n').last().unwrap().len() as i64,
                    line: input[..=index].split('\n').count() as i64,
                    index,
                })
            };

            index += lexeme.len();
        } else {
            dbg!(tokens, index);

            return Err(SyntaxError::UnexpectedToken(
                input[..=index].split('\n').last().unwrap().split_whitespace().last().unwrap().to_owned(),
                input[..=index].split('\n').last().unwrap().len() as i64,
                input[..=index].split('\n').count() as i64,
            ))
        }
    }

    Ok(tokens)
}
