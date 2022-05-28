mod lexer;
mod lexer_test;
mod repl;
mod token;

fn main() {
    println!("Hello, this is the Monkey programming language");
    repl::start();
}
