#[derive(Debug)]

enum Token{
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

fn tokenize(input: &str)-> Vec<Token>{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek(){
        match c{
            ' ' | '\t' | '\n' => {chars.next();} 
            '+' => {tokens.push(Token::Plus); chars.next();}
            '-' => {tokens.push(Token::Minus); chars.next();}
            '*' => {tokens.push(Token::Star); chars.next();}
            '/' => {tokens.push(Token::Slash); chars.next();}
            '(' => {tokens.push(Token::LParen); chars.next();}
            ')' => {tokens.push(Token::RParen); chars.next();}
            '0'..='9'| '.'=>{
                let mut num = String::new();
                while let Some(&d) = chars.peek(){
                    if d.is_ascii_digit() || d == '.'{
                        num.push(d);
                        chars.next();
                    }
                    else{
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            _=> panic!("Unexpected character {}", c),
        }
    }
        
    tokens
}

fn token() {
    let tokens = tokenize("12 + 6(3 - 1)");
    println!("{:?}", tokens);
} 

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn simple_addition(){
        assert_eq!(
            tokenize("1 + 2"),
            vec![Token::Number(1.0), Token::Plus, Token::Number(2.0 )]
        );  
    }
}
