mod code;
mod compiler;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod vm;
mod object;

const COMPILED: bool = true;

fn main() {
    if COMPILED {
        repl::start_vm();
    } else {
        repl::start_interpreter();
    }
}
