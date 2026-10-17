#[derive(Debug, PartialEq, Clone)]

pub enum TokenType{
    Number(f64),
    Bool(bool),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    LBrace,
    RBrace,
    Semicolon,
    Let,
    If,
    Else,
    While,
    Print,
    Eof,
}

#[derive (Debug, PartialEq, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub line: usize,
}

impl Token{
    pub fn new(token_type: TokenType, line:usize)->Self{
        Token{ token_type, line}
    }
}