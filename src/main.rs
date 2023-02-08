mod command;

fn main() {
    // let tokens = command::parser::tokenise("http(url: 'https://api.example.com/v1/users', headers: { accept: 'text/json5' }) | json | .users | map(user -> user + { age: Date(user.dob).elapsed().years } - { id: r'.*' })").unwrap();
    let tokens = command::parser::tokenise("readdir(file: 'file:/home/user') | keys").unwrap();
    // let tokens = command::parser::tokenise("map(i -> i + 'Oi')").unwrap();
    // let tokens = command::parser::tokenise("index.(1 + 2).hi").unwrap();
    let ast = command::parser::parse(&tokens).unwrap();

    println!("{:#?}", ast);
}
