
use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::environment::{Environment, Value};

pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }

            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val);
                Ok(())
            }

            Stmt::Let { name, initializer } => {
                let val = self.eval_expr(initializer)?;
                self.env.define(name.clone(), val);
                Ok(())
            }

            Stmt::Block(statements) => {
                self.env.enter_scope();

                // Execute all statements inside the block
                let mut block_result = Ok(());
                for inner_stmt in statements {
                    if let Err(e) = self.execute(inner_stmt) {
                        block_result = Err(e);
                        break;
                    }
                }
                self.env.exit_scope();
                block_result
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(condition)?;

                if self.is_truthy(&cond_val) {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                let mut cond_val = self.eval_expr(condition)?;

                while self.is_truthy(&cond_val) {
                    self.execute(body)?;
                    cond_val = self.eval_expr(condition)?; // Re-evaluate condition after each iteration
                }

                Ok(())
            }
        }
    }
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Variable(name) => self
                .env
                .get(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),

            Expr::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.assign(name, val.clone())?;
                Ok(val)
            }

            Expr::Unary { op, right } => {
                let right_val = self.eval_expr(right)?;
                match (op, right_val) {
                    (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOp::Not, val) => Ok(Value::Bool(!self.is_truthy(&val))),
                    _ => Err("Invalid operand type for unary operation".to_string()),
                }
            }

            Expr::Binary { left, op, right } => {
                if *op == BinaryOp::And {
                    let left_val = self.eval_expr(left)?;
                    if !self.is_truthy(&left_val) {
                        return Ok(Value::Bool(false)); // Short-circuit!
                    }
                    let right_val = self.eval_expr(right)?;
                    return Ok(Value::Bool(self.is_truthy(&right_val)));
                }

                if *op == BinaryOp::Or {
                    let left_val = self.eval_expr(left)?;
                    if self.is_truthy(&left_val) {
                        return Ok(Value::Bool(true)); // Short-circuit!
                    }
                    let right_val = self.eval_expr(right)?;
                    return Ok(Value::Bool(self.is_truthy(&right_val)));
                }
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match (op, left_val, right_val) {
                    // Arithmetic
                    (BinaryOp::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (BinaryOp::Sub, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    (BinaryOp::Mul, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    (BinaryOp::Div, Value::Number(a), Value::Number(b)) => {
                        if b == 0.0 {
                            Err("Division by zero error".to_string())
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }
                    (BinaryOp::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                    (BinaryOp::LessEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (BinaryOp::Greater, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a > b))
                    }
                    (BinaryOp::GreaterEqual, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (BinaryOp::Equal, a, b) => Ok(Value::Bool(a == b)),
                    (BinaryOp::NotEqual, a, b) => Ok(Value::Bool(a != b)),

                    _ => Err("Invalid operand types for binary operation".to_string()),
                }
            }
        }
    }
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Nil => false,
            Value::Number(_) => true,
        }
    }
}