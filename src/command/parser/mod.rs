mod parser;
mod tokeniser;
mod matchers;
mod syntax_err;

pub use parser::*;
pub use tokeniser::*;
pub use matchers::*;
pub use syntax_err::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_basic_call() -> Result<(), SyntaxError> {
        {
            let tokens = tokenise("echo()")?;
            let ast = parse(&tokens)?;

            println!("{:#?}", ast);
        }

        {
            let tokens = tokenise("echo(\"Hi\")").unwrap();
            let ast = parse(&tokens).unwrap();

            println!("{:#?}", ast);
        }

        Ok(())
    }

    #[test]
    pub fn test_parse_basic_expr() -> Result<(), SyntaxError> {
        {
            let tokens = tokenise("1 + 2 * 3")?;
            let ast = parse(&tokens)?;

            println!("{:#?}", ast);
        }

        {
            let tokens = tokenise("1 + 2 * 3 + echo(\"hi\")")?;
            let ast = parse(&tokens)?;

            println!("{:#?}", ast);
        }

        Ok(())
    }
}
