#![allow(non_snake_case)]

mod token;
mod lexer;

fn main() {
    let text = String::from("12 + 34 / (6 + 7);");

    let mut lexer = lexer::Lexer::new(text);
    let tokens =  lexer.analyze();

    for token in tokens {
        println!("Type=[{}]: Value=[{}]", token::TokenTypeToStr(token.getType()), token.getValue());
    }
}