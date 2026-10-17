
mod token;
mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn run(source: &str) {
    println!("--- Program Code ---\n{}\n--------------------", source.trim());

    // 1. Lexing
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer Error: {}", e);
            return;
        }
    };
    let mut parser = Parser::new(tokens);
    let statements = match parser.parse_program() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parser Error: {}", e);
            return;
        }
    };
    println!("--- Output ---");
    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.interpret(&statements) {
        eprintln!("Runtime Error: {}", e);
    }
    println!("--------------\n");
}

fn main() {
    let test_program_1 = r#"
        let a = 15;
        let b = 20;

        if (a < b && !(a == b)) {
            print 1111;
        } else {
            print 9999;
        }
    "#;
    run(test_program_1);
    let test_program_2 = r#"
        let count = 0;
        let sum = 0;

        while (count < 5) {
            count = count + 1;
            sum = sum + count;
        }

        print sum;
    "#;
    run(test_program_2);
    let test_program_3 = r#"
        let x = 100;
        {
            let x = 200;
            print x; // Should print 200 (shadowed inside block)
        }
        print x; // Should print 100 (outer scope restored)
    "#;
    run(test_program_3);
}