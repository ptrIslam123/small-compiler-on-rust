#![allow(non_snake_case)]

mod token;
mod lexer;

fn main() {
    let text = String::from("def func myFunc = () {}");

    let mut lexer = lexer::Lexer::new(text);
    let tokens =  lexer.analyze();

    for token in tokens {
        println!("Type=[{}]: Value=[{}]", token::TokenTypeToStr(token.getType()), token.getValue());
    }
}