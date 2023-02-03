use regex::bytes::Regex;
use crate::command::parser::tokeniser::{BracketType, KeywordType, OperatorType, PipeType, TokenType};

pub struct Matcher {
    symbol: Regex,
    string: Regex,
    number: Regex,
    boolean: Regex,
    operator: Regex,
    keyword: Regex,
    open_bracket: Regex,
    close_bracket: Regex,
    colon: Regex,
    semicolon: Regex,
    comma: Regex,
    dot: Regex,
    lambda: Regex,
    whitespace: Regex,
    comment: Regex,
}

impl Matcher {
    pub(crate) fn new() -> Self {
        Self {
            symbol: Regex::new(r"(^[~.#]?(?:/\S+)+)|^([~.#]?/)|^([a-zA-Z0-9][a-zA-Z0-9@#$^]+)").unwrap(),
            string: Regex::new(r#"^[a-z]?"([^"\\]|\\.)*"|^[a-z]?'([^'\\]|\\.)*'"#).unwrap(),
            number: Regex::new(r"(^-?[0-9]+(?:\.[0-9]+)?(?:[xX][+-]?[0-9]+)?)|(^-?0x[0-9a-fA-F]+(?:\.[0-9a-fA-F]+)?(?:[xX][+-]?[0-9]+)?)|(^-?0b[01]+(?:\.[01]+)?(?:[xX][+-]?[0-9]+)?)").unwrap(),
            boolean: Regex::new(r"^true|false").unwrap(),
            operator: Regex::new(r"^\|([oO]?[eE]?)|\|([eE]?[oO]?)|^\+|^-|^\*|^/|^%|^\^|^==|^!=|^>|^<|^>=|^<=|^&&|^\|\||^!|^=").unwrap(),
            keyword: Regex::new(r"^if|^else|^for|^function|^return|^import").unwrap(),
            open_bracket: Regex::new(r"^\(|^\{|^\[|^<").unwrap(),
            close_bracket: Regex::new(r"^\)|^}|^]|^>").unwrap(),
            colon: Regex::new(r"^:").unwrap(),
            semicolon: Regex::new(r"^;").unwrap(),
            comma: Regex::new(r"^,").unwrap(),
            dot: Regex::new(r"^\.").unwrap(),
            lambda: Regex::new(r"^->").unwrap(),
            whitespace: Regex::new(r"^\s+").unwrap(),
            comment: Regex::new(r"^(//[^\n]*|/\*.*\*/)").unwrap(),
        }
    }

    pub fn match_all(&self, str: &str) -> Option<(String, TokenType)> {
        let vec: Vec<(String, TokenType)> = vec![
            self.symbol.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Symbol(m))),

            self.string.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::String(m))),

            self.number.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Number(m.parse::<f64>().unwrap()))),

            self.boolean.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Boolean(m == "true"))),

            self.operator.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Operator(match m.as_str() {
                    "|" => OperatorType::Pipe(PipeType::Stdout),
                    "|o" => OperatorType::Pipe(PipeType::Stdout),
                    "|O" => OperatorType::Pipe(PipeType::Stdout),
                    "|e" => OperatorType::Pipe(PipeType::Stderr),
                    "|E" => OperatorType::Pipe(PipeType::Stderr),
                    "|oe" => OperatorType::Pipe(PipeType::Both),
                    "|OE" => OperatorType::Pipe(PipeType::Both),
                    "|eo" => OperatorType::Pipe(PipeType::Both),
                    "|EO" => OperatorType::Pipe(PipeType::Both),
                    "|oE" => OperatorType::Pipe(PipeType::Both),
                    "|Oe" => OperatorType::Pipe(PipeType::Both),
                    "|eO" => OperatorType::Pipe(PipeType::Both),
                    "|Eo" => OperatorType::Pipe(PipeType::Both),
                    "+" => OperatorType::Add,
                    "-" => OperatorType::Subtract,
                    "*" => OperatorType::Multiply,
                    "/" => OperatorType::Divide,
                    "%" => OperatorType::Modulo,
                    "^" => OperatorType::Exponent,
                    "==" => OperatorType::Equal,
                    "!=" => OperatorType::NotEqual,
                    ">" => OperatorType::GreaterThan,
                    "<" => OperatorType::LessThan,
                    ">=" => OperatorType::GreaterThanOrEqual,
                    "<=" => OperatorType::LessThanOrEqual,
                    "&&" => OperatorType::And,
                    "||" => OperatorType::Or,
                    "!" => OperatorType::Not,
                    "=" => OperatorType::Assign,
                    _ => panic!("Unknown operator: {}", m),
                }))),

            self.keyword.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Keyword(match m.as_str() {
                    "if" => KeywordType::If,
                    "else" => KeywordType::Else,
                    "for" => KeywordType::For,
                    "function" => KeywordType::Function,
                    "return" => KeywordType::Return,
                    "import" => KeywordType::Import,
                    _ => panic!("Unknown keyword: {}", m),
                }))),

            self.open_bracket.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::OpenBracket(match m.as_str() {
                    "(" => BracketType::Parenthesis,
                    "{" => BracketType::Brace,
                    "[" => BracketType::Bracket,
                    "<" => BracketType::Angle,
                    _ => panic!("Unknown bracket: {}", m),
                }))),

            self.close_bracket.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::CloseBracket(match m.as_str() {
                    ")" => BracketType::Parenthesis,
                    "}" => BracketType::Brace,
                    "]" => BracketType::Bracket,
                    ">" => BracketType::Angle,
                    _ => panic!("Unknown bracket: {}", m),
                }))),

            self.colon.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Colon)),

            self.semicolon.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Semicolon)),

            self.comma.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Comma)),

            self.dot.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Dot)),

            self.lambda.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Lambda)),

            self.whitespace.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Whitespace(m))),

            self.comment.find(str.as_ref())
                .map(|m| String::from_utf8_lossy(m.as_bytes()).to_string())
                .map(|m| (m.clone(), TokenType::Comment(m))),
        ].into_iter()
            .flatten()
            .collect();

        if vec.is_empty() {
            return None;
        }

        Some(vec.iter().reduce(|a, i| if a.0.len() > i.0.len() { a } else { i }).unwrap().clone())
    }
}
