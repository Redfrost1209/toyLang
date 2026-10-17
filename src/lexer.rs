use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
            line: 1,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    self.line += 1;
                    self.chars.next();
                }
                '+' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::Plus, self.line));
                }
                '-' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::Minus, self.line));
                }
                '*' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::Star, self.line));
                }
                '/' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'/') {
                        self.chars.next();
                        while let Some(&comment_char) = self.chars.peek() {
                            if comment_char == '\n' {
                                break;
                            }
                            self.chars.next();
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Slash, self.line));
                    }
                }
                '(' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::LParen, self.line));
                }
                ')' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::RParen, self.line));
                }
                '{' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::LBrace, self.line));
                }
                '}' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::RBrace, self.line));
                }
                ';' => {
                    self.chars.next();
                    tokens.push(Token::new(TokenType::Semicolon, self.line));
                }
                '=' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::EqualEqual, self.line));
                    } else {
                        tokens.push(Token::new(TokenType::Equal, self.line));
                    }
                }
                '!' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::BangEqual, self.line));
                    } else {
                        tokens.push(Token::new(TokenType::Bang, self.line));
                    }
                }
                '<' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::LessEqual, self.line));
                    } else {
                        tokens.push(Token::new(TokenType::Less, self.line));
                    }
                }
                '>' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'=') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, self.line));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, self.line));
                    }
                }
                '&' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'&') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::And, self.line));
                    } else {
                        return Err(format!(
                            "Line {}: Unexpected character '&', did you mean '&&'?",
                            self.line
                        ));
                    }
                }
                '|' => {
                    self.chars.next();
                    if self.chars.peek() == Some(&'|') {
                        self.chars.next();
                        tokens.push(Token::new(TokenType::Or, self.line));
                    } else {
                        return Err(format!(
                            "Line {}: Unexpected character '|', did you mean '||'?",
                            self.line
                        ));
                    }
                }
                '0'..='9' => {
                    let num = self.read_number()?;
                    tokens.push(Token::new(TokenType::Number(num), self.line));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    let token_type = match ident.as_str() {
                        "let" => TokenType::Let,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "print" => TokenType::Print,
                        "true" => TokenType::Bool(true),
                        "false" => TokenType::Bool(false),
                        _ => TokenType::Ident(ident),
                    };
                    tokens.push(Token::new(token_type, self.line));
                }
                _ => {
                    return Err(format!(
                        "Line {}: Unexpected character '{}'",
                        self.line, c
                    ));
                }
            }
        }
        tokens.push(Token::new(TokenType::Eof, self.line));
        Ok(tokens)
    }
    fn read_number(&mut self) -> Result<f64, String> {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.chars.next();
            } else if c == '.' && !has_dot {
                has_dot = true;
                num_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        num_str.parse::<f64>().map_err(|_| {
            format!("Line {}: Failed to parse float '{}'", self.line, num_str)
        })
    }
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        ident
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_and_precedence_tokens() {
        let mut lexer = Lexer::new("12 + 6 * (3 - 1)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Number(12.0), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Number(6.0), 1),
                Token::new(TokenType::Star, 1),
                Token::new(TokenType::LParen, 1),
                Token::new(TokenType::Number(3.0), 1),
                Token::new(TokenType::Minus, 1),
                Token::new(TokenType::Number(1.0), 1),
                Token::new(TokenType::RParen, 1),
                Token::new(TokenType::Eof, 1),
            ]
        );
    }

    #[test]
    fn test_keywords_and_logic() {
        let code = "let x = 10;\nif (x >= 5 && true) {\n  print x;\n}";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::new(TokenType::Let, 1));
        assert_eq!(tokens[1], Token::new(TokenType::Ident("x".into()), 1));
        assert_eq!(tokens[2], Token::new(TokenType::Equal, 1));
        assert_eq!(tokens[3], Token::new(TokenType::Number(10.0), 1));
        assert_eq!(tokens[4], Token::new(TokenType::Semicolon, 1));

        // Line 2
        assert_eq!(tokens[5], Token::new(TokenType::If, 2));
        assert_eq!(tokens[6], Token::new(TokenType::LParen, 2));
        assert_eq!(tokens[7], Token::new(TokenType::Ident("x".into()), 2));
        assert_eq!(tokens[8], Token::new(TokenType::GreaterEqual, 2));
        assert_eq!(tokens[9], Token::new(TokenType::Number(5.0), 2));
        assert_eq!(tokens[10], Token::new(TokenType::And, 2));
        assert_eq!(tokens[11], Token::new(TokenType::Bool(true), 2));
        assert_eq!(tokens[12], Token::new(TokenType::RParen, 2));
        assert_eq!(tokens[13], Token::new(TokenType::LBrace, 2));

        // Line 3
        assert_eq!(tokens[14], Token::new(TokenType::Print, 3));
        assert_eq!(tokens[15], Token::new(TokenType::Ident("x".into()), 3));
        assert_eq!(tokens[16], Token::new(TokenType::Semicolon, 3));

        // Line 4
        assert_eq!(tokens[17], Token::new(TokenType::RBrace, 4));
        assert_eq!(tokens[18], Token::new(TokenType::Eof, 4));
    }

    #[test]
    fn test_comments_ignored() {
        let code = "let a = 1; // this is a comment\nlet b = 2;";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::new(TokenType::Let, 1));
        assert_eq!(tokens[4], Token::new(TokenType::Semicolon, 1));
        // Next token should jump straight to 'let' on line 2
        assert_eq!(tokens[5], Token::new(TokenType::Let, 2));
    }
}