mod command;

use std::env::args;

fn main() {
    println!("Tokens: {:#?}", command::parser::tokenise("http(url: 'https://api.example.com/v1/users') | json | .users | map(user -> user + { age: Date(user.dob).elapsed().years } - { id: r'.*' })").unwrap());
}
