use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{Stream, StreamExt};

use crate::command::parser::{ASTNode, LiteralToken, OperatorType, OpOrExpr, SyntaxError};
use crate::command::proc::ProcessOptions;

#[derive(Debug, Clone)]
pub struct ByteStream {
    from_string: String,
}

impl Stream for ByteStream {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.from_string.is_empty() {
            let c = self.from_string.clone();
            self.from_string = String::new();
            Poll::Ready(Some(c.as_bytes().to_vec()))
        } else {
            Poll::Ready(None)
        }
    }
}

impl ByteStream {
    pub async fn merge(mut self) -> Vec<u8> {
        let mut vec = vec![];

        while let Some(i) = self.next().await {
            vec.extend(i)
        }

        vec
    }

    pub fn from_string(s: &str) -> Self {
        ByteStream {
            from_string: s.to_owned()
        }
    }
}

pub fn eval(ast: Box<ASTNode>, options: ProcessOptions) -> Pin<Box<dyn Future<Output=Result<ByteStream, SyntaxError>>>> {
    Box::pin(async move {
        match *ast.clone() {
            ASTNode::Call(function, args) => {
                let executable = eval(function, ProcessOptions { resolve_names_to_executables: true, ..options.clone() }).await?;

                let data = executable
                    .merge()
                    .await;
                let data = String::from_utf8_lossy(&data);

                println!("Data: {:?}", data);
            }
            ASTNode::Expression(expr) => {
                // Implement the Shunting-Yard algorithm to correctly evaluate operations

                let mut opstack = VecDeque::<OpOrExpr>::new();
                let mut output = VecDeque::<OpOrExpr>::new();

                for token in expr {
                    match token {
                        OpOrExpr::Operator(op) => {
                            while let Some(OpOrExpr::Operator(top)) = opstack.back() {
                                if top.precedence() > op.precedence() {
                                    break;
                                }

                                output.push_back(OpOrExpr::Operator(*top));
                            }

                            opstack.push_back(OpOrExpr::Operator(op));
                        }
                        OpOrExpr::Expr(expr) => output.push_back(OpOrExpr::Expr(expr)),
                        OpOrExpr::Literal(lit) => output.push_back(OpOrExpr::Literal(lit))
                    }
                }

                while let Some(token) = opstack.pop_front() {
                    output.push_back(token);
                }

                if output.len() > 1 {
                    while let Some(op) = output.pop_back() {
                        if let OpOrExpr::Operator(op) = op {
                            match op {
                                OperatorType::Add => {}
                                OperatorType::Subtract => {}
                                _ => return Err(SyntaxError::UnsupportedExpression(*ast.clone()))
                            }
                        } else {
                            return Err(SyntaxError::UnsupportedExpression(*ast.clone()));
                        }
                    }
                } else {
                    if let Some(OpOrExpr::Literal(val)) = output.pop_back() {
                        return match val {
                            LiteralToken::Symbol(name) => if options.resolve_names_to_executables {
                                match locate_binary(&name) {
                                    Some(binary) => Ok(ByteStream::from_string(&binary)),
                                    None => Err(SyntaxError::NoValue(name))
                                }
                            } else {
                                std::env::var(&name)
                                    .map(|i| ByteStream::from_string(&i))
                                    .map_err(|_| SyntaxError::NoValue(name))
                            },
                            LiteralToken::String(str) => Ok(ByteStream::from_string(&str)),
                            LiteralToken::Boolean(bool) => Ok(ByteStream::from_string(if bool { "true" } else { "false" })),
                            LiteralToken::Number(num) => Ok(ByteStream::from_string(&format!("{}", num))),
                        };
                    }

                    return Err(SyntaxError::UnsupportedExpression(*ast.clone()));
                }
            }
            _ => todo!()
        };

        Err(SyntaxError::UnsupportedExpression(*ast.clone()))
        // ByteStream {}
    })
}

pub fn locate_binary(hint: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_else(|_| "/usr/local/bin:/usr/bin:/bin".to_string());
    let paths = path.split(':');

    for path in paths {
        let path = format!("{}/{}", path, hint);
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}
