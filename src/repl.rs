use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start() {
    let mut line_str = String::new();
    let in_stream = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let read_res = in_stream.read_line(&mut line_str);
        match read_res {
            Err(_) => return,
            Ok(0) => return,
            Ok(_) => {}
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
