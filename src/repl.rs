use crate::{
    compiler::Compiler,
    interpreter::{environment::Environment, eval_program},
    lexer::Lexer,
    object::Inspectable,
    parser::Parser,
    vm::VM,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    process::exit,
    rc::Rc,
};

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

    let mut vm = VM::new();
    let mut compiler = Compiler::new();

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
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(errors) => {
                println!(
                    "The following parsing errors occurred:\n{}",
                    errors.join("\n")
                );
                continue;
            }
        };

        if let Err(err) = compiler.compile(program) {
            println!("The following compiler errors occurred:\n{}", err);
            continue;
        }

        match vm.run(compiler.bytecode()) {
            Ok(obj) => println!("{}", obj.inspect()),
            Err(err) => println!("A runtime error occurred:\n{}", err),
        }
    }
}
