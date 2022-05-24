mod token;
mod lexer_test;
mod lexer;

fn main() {
    let tok = token::Token { literal: "lol".to_string(), tok_type: token::TokenType::SEMICOLON };
    println!("{}", tok.tok_type);
    println!("Hello, world!");
}
