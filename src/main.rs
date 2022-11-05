mod code;
mod compiler;
mod interpreter;
mod lexer;
mod parser;
mod repl;
mod vm;

fn main() {
    repl::start();
}
