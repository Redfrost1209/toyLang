mod token;
mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;
use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn run(source: &str, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("Lexer Error: {}", e); return; }
    };
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse_program() {
        Ok(s) => s,
        Err(e) => { eprintln!("Parser Error: {}", e); return; }
    };
    if let Err(e) = interpreter.interpret(&statements) {
        eprintln!("Runtime Error: {}", e);
    }
}

fn main() {
    let mut interpreter = Interpreter::new();   // created once so variables persist

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 { break; }  // Ctrl+D / Ctrl+Z
        if line.trim() == "exit" { break; }

        run(&line, &mut interpreter);
    }
}