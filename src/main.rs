mod interpreter;
mod lexer;
mod parser;
mod repl;
mod virtualmachine;
mod compiler;

fn main() {
    repl::start();
}
