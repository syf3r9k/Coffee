

mod lexer;
mod interpreter;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut tokens = Vec::new();
    let filename = "main.clf";
    let lines = read_lines(filename)?;
    for line in lines {
        tokens.push(lexer::Lexer(line))
    }

    interpreter::Interpreter(tokens);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}


