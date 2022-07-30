mod interpreter;
mod lexer;
mod parser;
mod repl;
mod virtualmachine;

fn main() {
    repl::start();
}
