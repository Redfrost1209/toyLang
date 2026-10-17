
mod token;
mod lexer;
mod ast;
mod parser;
use lexer::Lexer;
use ast::{Expr, BinaryOp, Stmt};

fn main() {
    println!("=== 1. Testing Lexer ===");
    let source_code = r#"
        let x = 10;
        let y = 20;
        if (x < y) {
            print x + y;
        }
    "#;

    let mut lexer = Lexer::new(source_code);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Successfully tokenized {} tokens:\n", tokens.len());
            for token in &tokens {
                println!("  Line {:2} | {:?}", token.line, token.token_type);
            }
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    }

    println!("\n=== 2. Previewing AST Node Structure ===");
    let sample_ast = Stmt::Let {
        name: "x".to_string(),
        initializer: Expr::Binary {
            left: Box::new(Expr::Number(5.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(10.0)),
        },
    };

    println!("{:#?}", sample_ast);
}