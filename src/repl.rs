use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::parser::print::program_to_string;
use crate::parser::Parser;

pub fn start() {
    let mut line_str = String::new();
    let in_stream = io::stdin();
    println!(
        "Welcome {}! This is the monkey-rs programming language.",
        whoami::username()
    );
    io::stdout().flush().expect("Error flushing stdout");
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        in_stream
            .read_line(&mut line_str)
            .expect("Error reading from stdin");

        if line_str.trim() == "exit" {
            return;
        }

        let lexer = Lexer::new(&line_str);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        match program {
            Ok(prog) => println!("{}", program_to_string(prog)),
            Err(err_vec) => {
                println!("The following errors occurred:\n{}", err_vec.join("\n"));
            }
        }
        line_str.clear();
    }
}
