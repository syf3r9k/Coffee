mod lexer;
mod interpreter;
mod reader;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut tokens = Vec::new();
    let filename = "main.clf";
    let lines = reader::read_lines(filename)?;
    for line in lines {
        tokens.push(lexer::Lexer(line))
    }

    interpreter::Interpreter(tokens);

    Ok(())
}