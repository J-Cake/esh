use crate::command::parser::tokeniser::{BracketType, OperatorType, PipeType, Token, TokenType};
pub use crate::command::parser::tokeniser::tokenise;

pub enum OpOrExpr {
    Operator(OperatorType),
    Expr(Box<ASTNode>)
}

pub enum KeyOrNoKey {
    Key(String, Box<ASTNode>),
    NoKey(Box<ASTNode>)
}

pub enum ASTNode {
    Call(String, Vec<KeyOrNoKey>),
    Pipe(PipeType, Box<ASTNode>),
    Lambda(Vec<String>, Box<ASTNode>),
    Expression(Vec<OpOrExpr>),
    Dict(Vec<KeyOrNoKey>),
    Index(Vec<ASTNode>),
    // TODO: Define control-flow
    Nothing
}

pub fn get_enclosed_tokens(tokens: &[Token]) -> Result<&[Token], String> {
    let mut bracket_count: (usize, usize, usize, usize) = (0, 0, 0, 0);

    for (a, i) in tokens.iter().enumerate() {
        match i.token_type {
            TokenType::OpenBracket(bracket) => match bracket {
                BracketType::Parenthesis => bracket_count.0 += 1,
                BracketType::Brace => bracket_count.1 += 1,
                BracketType::Bracket => bracket_count.2 += 1,
                BracketType::Angle => bracket_count.3 += 1,
            },
            TokenType::CloseBracket(bracket) => match bracket {
                BracketType::Parenthesis => bracket_count.0 -= 1,
                BracketType::Brace => bracket_count.1 -= 1,
                BracketType::Bracket => bracket_count.2 -= 1,
                BracketType::Angle => bracket_count.3 -= 1,
            },
            _ => continue
        }

        if bracket_count.0 + bracket_count.1 + bracket_count.2 + bracket_count.3 == 0 {
            return Ok(&tokens[1..a]);
        }
    }

    Err(format!("SyntaxError: Bracket Mismatch at {}:{}", tokens[0].line, tokens[0].column))
}

pub fn parse(tokens: Vec<Token>) -> Box<ASTNode> {
    Box::new(ASTNode::Nothing)
}
