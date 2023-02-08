use crate::command::parser::syntax_err::SyntaxError;
use crate::command::parser::tokeniser::{BracketType, OperatorType, PipeType, Token, TokenType};
pub use crate::command::parser::tokeniser::tokenise;

#[derive(Debug)]
pub enum OpOrExpr {
    Operator(OperatorType),
    Expr(Box<ASTNode>),
    Literal(LiteralToken),
}

#[derive(Debug)]
pub enum KeyOrNoKey {
    Key(String, Box<ASTNode>),
    NoKey(Box<ASTNode>),
}

#[derive(Debug)]
pub enum LiteralToken {
    Symbol(String),
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub enum DictKey {
    Key(Box<ASTNode>, Box<ASTNode>),
    NoKey(Box<ASTNode>),
}

#[derive(Debug)]
pub enum ASTNode {
    Call(Box<ASTNode>, Vec<KeyOrNoKey>), //
    Lambda(Vec<String>, Box<ASTNode>), //
    Expression(Vec<OpOrExpr>), //
    Dict(Vec<DictKey>), //
    Index(Vec<Box<ASTNode>>),
    // TODO: Define control-flow
    Nothing,
}

pub fn get_enclosed_tokens(tokens: &[Token]) -> Result<&[Token], SyntaxError> {
    let mut bracket_count: (isize, isize, isize, isize) = (0, 0, 0, 0);

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

        if bracket_count.0 < 0 || bracket_count.1 < 0 || bracket_count.2 < 0 || bracket_count.3 < 0 {
            return Err(SyntaxError::BracketMismatch(tokens[0].line, tokens[0].column));
        }

        if bracket_count.0 + bracket_count.1 + bracket_count.2 + bracket_count.3 == 0 {
            return Ok(&tokens[1..a]);
        }
    }

    Err(SyntaxError::BracketMismatch(tokens[0].line, tokens[0].column))
}

pub fn top_level_split<F>(tokens: &[Token], predicate: F, keep_delimiter: bool) -> Result<Vec<&[Token]>, SyntaxError> where F: Fn(&Token) -> bool {
    let mut sections: Vec<&[Token]> = Vec::new();

    let mut start = 0;
    let mut bracket_count: (isize, isize, isize, isize) = (0, 0, 0, 0);

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
            _ => {}
        }

        if bracket_count.0 < 0 || bracket_count.1 < 0 || bracket_count.2 < 0 || bracket_count.3 < 0 {
            return Err(SyntaxError::BracketMismatch(tokens[0].line, tokens[0].column));
        }

        if bracket_count.0 + bracket_count.1 + bracket_count.2 + bracket_count.3 == 0 && predicate(i) {
            let s = start;
            start = a + 1;

            sections.push(if keep_delimiter {
                &tokens[s..start]
            } else {
                &tokens[s..a]
            });
        }
    }

    if start < tokens.len() {
        sections.push(&tokens[start..]);
    }

    Ok(sections)
}

fn parse_call(tokens: &[Token]) -> Result<ASTNode, SyntaxError> {
    if tokens.len() < 3 || !matches!(tokens.last().unwrap().token_type, TokenType::CloseBracket(BracketType::Parenthesis)) {
        return Err(SyntaxError::UnexpectedEOF());
        // return Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column));
    }

    // find last top-level parenthesis
    let mut bracket_count = 0isize;
    let mut index = 0usize;
    for (a, i) in tokens.iter().enumerate() {
        if matches!(i.token_type, TokenType::OpenBracket(BracketType::Parenthesis)) {
            if bracket_count == 0 {
                index = a;
            }
            bracket_count += 1;
        } else if matches!(i.token_type, TokenType::CloseBracket(BracketType::Parenthesis)) {
            bracket_count -= 1;
        }
    }
    let (function, enclosed) = tokens.split_at(index);

    if let Some(last) = function.last() {
        if matches!(last.token_type, TokenType::Dot) {
            return Err(SyntaxError::InvalidSyntax(last.line, last.column));
        }
    }

    let enclosed_tokens = top_level_split(get_enclosed_tokens(enclosed)?, |t| matches!(t.token_type, TokenType::Comma), false)?;

    let args: Vec<Result<KeyOrNoKey, SyntaxError>> = enclosed_tokens.iter()
        .map(|i| if i.len() > 1 && matches!(i[1].token_type, TokenType::Colon) {
            match parse(&i[2..]) {
                Ok(node) => Ok(KeyOrNoKey::Key(i[0].to_string(), node)),
                Err(e) => Err(e)
            }
        } else {
            match parse(i) {
                Ok(node) => Ok(KeyOrNoKey::NoKey(node)),
                Err(e) => Err(e)
            }
        }).collect();

    if let Some(Err(e)) = args.iter().find(|i| i.is_err()) {
        return Err(e.clone());
    }

    let args: Vec<KeyOrNoKey> = args.into_iter().map(|i| i.unwrap()).collect();

    Ok(ASTNode::Call(parse(function)?, args))
}

fn parse_expr(tokens: &[Token]) -> Result<ASTNode, SyntaxError> {
    // If no operators, then it's a single value, so continue
    if !tokens.iter().any(|i| matches!(i.token_type, TokenType::Operator(_))) {
        if tokens.len() == 1 {
            return match &tokens[0].token_type {
                TokenType::Symbol(symbol) => Ok(ASTNode::Expression(vec![OpOrExpr::Literal(LiteralToken::Symbol(symbol.clone()))])),
                TokenType::String(string) => Ok(ASTNode::Expression(vec![OpOrExpr::Literal(LiteralToken::String(string.clone()))])),
                TokenType::Number(number) => Ok(ASTNode::Expression(vec![OpOrExpr::Literal(LiteralToken::Number(*number))])),
                TokenType::Boolean(boolean) => Ok(ASTNode::Expression(vec![OpOrExpr::Literal(LiteralToken::Boolean(*boolean))])),
                _ => Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column))
            }
        }
        return Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column));
    }

    let operands = top_level_split(tokens, |t| matches!(t.token_type, TokenType::Operator(_)), true)?;

    let (last, rest) = operands.split_last().unwrap();

    Ok(ASTNode::Expression(rest.iter()
        .map(|i| i.split_last().unwrap())
        .map(|(op, i)| match parse(i) {
            Ok(expr) => {
                match op.token_type {
                    TokenType::Operator(op) => Ok(vec![OpOrExpr::Expr(expr), OpOrExpr::Operator(op)]),
                    _ => Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column))
                }
            },
            Err(err) => Err(err)
        })
        .chain(std::iter::once(match parse(last) {
            Ok(expr) => Ok(vec![OpOrExpr::Expr(expr)]),
            Err(err) => Err(err)
        }))
        .into_iter()
        .flatten()
        .flatten()
        .collect::<Vec<OpOrExpr>>()))
}

fn parse_dict(tokens: &[Token]) -> Result<ASTNode, SyntaxError> {
    // Dict: { expr: expr... }
    if let Some(token) = tokens.get(0) {
        if !matches!(token.token_type, TokenType::OpenBracket(BracketType::Brace)) {
            return Err(SyntaxError::InvalidSyntax(token.line, token.column));
        }
    }

    let enclosed_tokens = top_level_split(get_enclosed_tokens(tokens)?, |t| matches!(t.token_type, TokenType::Comma), false)?;

    let dict = enclosed_tokens.into_iter().map(|i| -> Result<DictKey, SyntaxError> {
        if let Some(pos) = i.iter().position(|i| matches!(i.token_type, TokenType::Colon)) {
            let (key, value) = i.split_at(pos);

            Ok(DictKey::Key(parse(key)?, parse(&value[1..])?))
        } else {
            Ok(DictKey::NoKey(parse(i)?))
        }
    }).collect::<Vec<_>>();

    if let Some(Err(e)) = dict.iter().find(|i| i.is_err()) {
        return Err(e.clone());
    }

    Ok(ASTNode::Dict(dict.into_iter().map(|i| i.unwrap()).collect()))
}

fn parse_index(tokens: &[Token]) -> Result<ASTNode, SyntaxError> {
    // Index: expr1.expr2.expr3
    if tokens.len() <= 1 || !tokens.iter().any(|i| matches!(i.token_type, TokenType::Dot)) {
        return Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column));
    }

    let indices = top_level_split(tokens, |t| matches!(t.token_type, TokenType::Dot), false)?;

    let mut indices = indices.into_iter().map(parse);

    if indices.any(|i| i.is_err()) {
        return Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column));
    }

    Ok(ASTNode::Index(indices.map(|i| i.unwrap()).collect()))
}

fn parse_lambda(tokens: &[Token]) -> Result<ASTNode, SyntaxError> {
    // Lambdas: arg1; arg2; ...; argn -> body

    if let Some(lambda) = tokens.iter().position(|i| matches!(i.token_type, TokenType::Lambda)) {
        let (args, body) = tokens.split_at(lambda);

        let args = top_level_split(args, |t| matches!(t.token_type, TokenType::Semicolon), false)?;

        if args.iter().any(|i| i.len() != 1 || !matches!(i[0].token_type, TokenType::Symbol(_))) {
            return Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column));
        }

        let args: Vec<String> = args.iter().map(|i| i[0].lexeme.to_owned()).collect();

        let body = parse(&body[1..])?;

        Ok(ASTNode::Lambda(args, body))
    } else {
        Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column))
    }
}

pub fn parse(tokens: &[Token]) -> Result<Box<ASTNode>, SyntaxError> {
    if let Some(token) = tokens.get(0) {
        if matches!(token.token_type, TokenType::OpenBracket(BracketType::Parenthesis)) {
            if let Ok(node) = get_enclosed_tokens(tokens) {
                return parse(node);
            }
        }
    }

    if let Ok(lambda) = parse_lambda(tokens) {
        return Ok(Box::new(lambda));
    }

    if let Ok(dict) = parse_dict(tokens) {
        return Ok(Box::new(dict));
    }

    if let Ok(index) = parse_index(tokens) {
        return Ok(Box::new(index));
    }

    if let Ok(call) = parse_call(tokens) {
        return Ok(Box::new(call));
    }

    if let Ok(expr) = parse_expr(tokens) {
        return Ok(Box::new(expr));
    }

    Err(SyntaxError::InvalidSyntax(tokens[0].line, tokens[0].column))
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_get_enclosed_content() {
        use super::*;
        use crate::command::parser::tokeniser::tokenise;

        let tokens = tokenise("(\"Hi\", \"World\"), \"hello\"").unwrap();
        let enclosed = get_enclosed_tokens(&tokens[0..]).unwrap();

        dbg!(&enclosed);

        assert_eq!(enclosed.len(), 3);
    }

    #[test]
    pub fn test_top_level_split() {
        use super::*;
        use crate::command::parser::tokeniser::tokenise;

        let tokens = tokenise("1, (1 + 2, 3), 2, 3").unwrap();
        let sections = top_level_split(&tokens, |t| matches!(t.token_type, TokenType::Comma), false)
            .unwrap();

        dbg!(&sections);

        assert_eq!(sections.len(), 4);
    }

    #[test]
    pub fn test_top_level_split_keep_delimiter() {
        use super::*;
        use crate::command::parser::tokeniser::tokenise;

        let tokens = tokenise("1, (1 + 2, 3), 2, 3").unwrap();
        let sections = top_level_split(&tokens, |t| matches!(t.token_type, TokenType::Comma), true)
            .unwrap();

        dbg!(&sections);

        assert_eq!(sections.len(), 4);
    }

    #[test]
    pub fn test_top_level_split_empty() {
        use super::*;
        use crate::command::parser::tokeniser::tokenise;

        let tokens = tokenise("1").unwrap();
        let sections = top_level_split(&tokens, |t| matches!(t.token_type, TokenType::Comma), false)
            .unwrap();

        dbg!(&sections);

        assert_eq!(sections.len(), 1);
    }

}
