mod command;

fn main() {
    let tokens = command::parser::tokenise("http(url: 'https://api.example.com/v1/users') | json | .users | map(user -> user + { age: Date(user.dob).elapsed().years } - { id: r'.*' })").unwrap();
    // println!("Tokens: {:#?}", &tokens);
    // println!("Enclosed {:#?}", &tokens[1..]);
    println!("Enclosed {:#?}", command::parser::get_enclosed_tokens(&tokens[1..]))
}
