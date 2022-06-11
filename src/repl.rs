use std::cell::RefCell;
use std::io::{self, Write};
use std::process::exit;
use std::rc::Rc;

use crate::interpreter::environment::Environment;
use crate::interpreter::eval_program;
use crate::interpreter::object::Inspectable;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn start() -> ! {
    let mut line_str = String::new();
    let in_stream = io::stdin();
    println!(
        "Welcome {}! This is the monkey-rs programming language.",
        whoami::username()
    );
    io::stdout().flush().expect("Error flushing stdout");
    let repl_environment = Rc::new(RefCell::new(Environment::new()));

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        in_stream
            .read_line(&mut line_str)
            .expect("Error reading from stdin");

        if line_str.trim() == "exit" {
            exit(0);
        }

        let lexer = Lexer::new(&line_str);
        let mut parser = Parser::new(lexer);
        match parser.parse_program() {
            Ok(program) => {
                let result = eval_program(program, repl_environment.clone());
                match result {
                    Ok(object) => println!("{}", object.inspect()),
                    Err(msg) => println!("ERROR! {}", msg),
                }
            }
            Err(err_vec) => {
                println!("The following errors occurred:\n{}", err_vec.join("\n"));
            }
        }
        line_str.clear();
    }
}
