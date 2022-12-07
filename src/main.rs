mod code;
mod compiler;
mod interpreter;
mod lexer;
mod object;
mod parser;
mod repl;
mod vm;

const COMPILED: bool = false;

fn main() {
    if COMPILED {
        repl::start_vm();
    } else {
        repl::start_interpreter();
    }
}
