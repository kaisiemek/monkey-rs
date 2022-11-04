mod code;
mod compiler;
mod interpreter;
mod lexer;
mod parser;
mod repl;

fn main() {
    repl::start();
}
