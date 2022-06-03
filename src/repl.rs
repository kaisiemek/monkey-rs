use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start() {
    let mut line_str = String::new();
    let in_stream = io::stdin();

    println!(
        "Welcome {}! This is the monkey-rs programming language.",
        whoami::username()
    );

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        in_stream
            .read_line(&mut line_str)
            .expect("Error reading from stdin");

        if line_str.trim() == "exit" {
            return;
        }

        let mut lexer = Lexer::new(&line_str);
        let mut cur_token = lexer.next_token();
        while cur_token.tok_type != TokenType::EOF {
            println!("{:?}", cur_token);
            cur_token = lexer.next_token();
        }
        line_str.clear();
    }
}
