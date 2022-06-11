use std::hash::Hash;

mod interpreter;
mod lexer;
mod parser;
mod repl;

fn main() {
    repl::start();
}
