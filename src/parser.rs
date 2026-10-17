// src/parser.rs

use crate::token::{Token, TokenType};
use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }
    fn consume(&mut self, expected: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Line {}: {} (found {:?})",
                self.peek().line,
                message,
                self.peek().token_type
            ))
        }
    }
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }
    pub fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek().token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Print => self.parse_print_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::LBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.advance();
        let name = match &self.peek().token_type {
            TokenType::Ident(name) => name.clone(),
            _ => {
                return Err(format!(
                    "Line {}: Expected variable name after 'let'",
                    self.peek().line
                ))
            }
        };
        self.advance();

        self.consume(&TokenType::Equal, "Expected '=' after variable name")?;
        let initializer = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Stmt::Let { name, initializer })
    }
    fn parse_print_statement(&mut self) -> Result<Stmt, String> {
        self.advance();
        let value = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after print value")?;
        Ok(Stmt::Print(value))
    }
    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.consume(&TokenType::LParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after if condition")?;

        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if matches!(self.peek().token_type, TokenType::Else) {
            self.advance(); // consume 'else'
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    fn parse_while_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'
        self.consume(&TokenType::LParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expression()?;
        self.consume(&TokenType::RParen, "Expected ')' after while condition")?;

        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::While { condition, body })
    }
    fn parse_block_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume '{'
        let mut statements = Vec::new();

        while !matches!(self.peek().token_type, TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(&TokenType::RBrace, "Expected '}' after block")?;
        Ok(Stmt::Block(statements))
    }
    fn parse_expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or()?;

        if matches!(self.peek().token_type, TokenType::Equal) {
            self.advance(); // consume '='
            let value = self.parse_assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            return Err("Invalid assignment target".to_string());
        }

        Ok(expr)
    }
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        while matches!(self.peek().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;

        while matches!(self.peek().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while matches!(
            self.peek().token_type,
            TokenType::EqualEqual | TokenType::BangEqual
        ) {
            let op = match self.advance().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while matches!(
            self.peek().token_type,
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual
        ) {
            let op = match self.advance().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while matches!(self.peek().token_type, TokenType::Plus | TokenType::Minus) {
            let op = match self.advance().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        while matches!(self.peek().token_type, TokenType::Star | TokenType::Slash) {
            let op = match self.advance().token_type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek().token_type, TokenType::Bang | TokenType::Minus) {
            let op = match self.advance().token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            return Ok(Expr::Unary {
                op,
                right: Box::new(right),
            });
        }

        self.parse_primary()
    }
    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.advance();
        match &token.token_type {
            TokenType::Number(n) => Ok(Expr::Number(*n)),
            TokenType::Bool(b) => Ok(Expr::Bool(*b)),
            TokenType::Ident(name) => Ok(Expr::Variable(name.clone())),
            TokenType::LParen => {
                let expr = self.parse_expression()?;
                self.consume(&TokenType::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!(
                "Line {}: Expected expression, found {:?}",
                token.line, token.token_type
            )),
        }
    }
}