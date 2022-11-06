use std::cell::RefCell;
use std::io::{self, Write};
use std::process::exit;
use std::rc::Rc;

use crate::compiler::Compiler;
use crate::interpreter::environment::Environment;
use crate::interpreter::eval_program;
use crate::interpreter::object::Inspectable;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::VM;

pub fn start_interpreter() -> ! {
    let mut line_str = String::new();
    let in_stream = io::stdin();
    println!(
        "Welcome {}! This is the monkey-rs programming language (interpreted).",
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
                    Err(msg) => println!("ERROR: {}", msg),
                }
            }
            Err(err_vec) => {
                println!("The following errors occurred:\n{}", err_vec.join("\n"));
            }
        }
        line_str.clear();
    }
}

pub fn start_vm() -> ! {
    let mut line_str = String::new();
    let in_stream = io::stdin();
    println!(
        "Welcome {}! This is the monkey-rs programming language (compiled).",
        whoami::username()
    );
    io::stdout().flush().expect("Error flushing stdout");

    loop {
        line_str.clear();
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

        let parse_result = parser.parse_program();
        if parse_result.is_err() {
            println!(
                "The following parsing errors occurred:\n{}",
                parse_result.unwrap_err().join("\n")
            );
            continue;
        }

        let program = parse_result.unwrap();
        let mut compiler = Compiler::new();
        let compile_result = compiler.compile(program);

        if compile_result.is_err() {
            println!(
                "The following compiler errors occurred:\n{}",
                compile_result.unwrap_err()
            );
            continue;
        }

        let mut vm = VM::new(compiler.bytecode());
        let run_result = vm.run();

        if run_result.is_err() {
            println!(
                "The following runtime errors occurred:\n{}",
                run_result.unwrap_err()
            );
            continue;
        }

        let stacktop = vm.last_popped_stack_elem();
        println!("{}", stacktop.inspect());
    }
}
