mod ast;
mod lexer;
mod lexer_test;
mod parser;
mod parser_test;
mod repl;
mod token;

fn main() {
    repl::start();
}
