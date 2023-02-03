mod command;

fn main() {
    // let tokens = command::parser::tokenise("http(url: 'https://api.example.com/v1/users', headers: { accept: 'text/json5' }) | json | .users | map(user -> user + { age: Date(user.dob).elapsed().years } - { id: r'.*' })").unwrap();
    let tokens = command::parser::tokenise("echo(\"hi\")").unwrap();
    let ast = command::parser::parse(&tokens).unwrap();

    println!("{:#?}", ast);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse___basic_call() -> Result<(), command::parser::SyntaxError> {
        {
            let tokens = command::parser::tokenise("echo()")?;
            let ast = command::parser::parse(&tokens)?;

            println!("{:#?}", ast);
        }

        {
            let tokens = command::parser::tokenise("echo(\"Hi\")").unwrap();
            let ast = command::parser::parse(&tokens).unwrap();

            println!("{:#?}", ast);
        }

        Ok(())
    }

    #[test]
    pub fn test_parse___basic_expr() -> Result<(), command::parser::SyntaxError> {
        {
            let tokens = command::parser::tokenise("1 + 2 * 3")?;
            let ast = command::parser::parse(&tokens)?;

            println!("{:#?}", ast);
        }

        {
            let tokens = command::parser::tokenise("1 + 2 * 3 + echo(\"hi\")")?;
            let ast = command::parser::parse(&tokens)?;

            println!("{:#?}", ast);
        }

        Ok(())
    }
}
