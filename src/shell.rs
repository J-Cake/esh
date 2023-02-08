use std::io::Write;
use crate::command::eval::eval;
use crate::command::parser;

pub async fn shell_main() {
    loop {
        std::io::stdout().write_all(b"> ").unwrap();
        std::io::stdout().flush().unwrap();
        let mut cmd = String::new();
        if std::io::stdin().read_line(&mut cmd).is_ok() && !cmd.is_empty(){
            if let Ok(tokens) = parser::tokenise(&cmd) {
                if tokens.is_empty() {
                    continue;
                }

                if let Ok(ast) = parser::parse(&tokens) {
                    // dbg!(&ast);
                    let res = eval(ast, Default::default()).await;
                    println!("{:#?}", res);
                    continue;
                }
            }

            eprintln!("Error: {}", cmd);
        }
    }
}
