use std::io;

mod lexer;
mod interpreter;

fn main() {
    let mut code: String = String::new();
    let _ = io::stdin().read_line(&mut code);
    lexer::Lexer(code);
    //interpreter::Interpreter(5);
}



