use std::collections::HashMap;
use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn exit_scope(&mut self) {
        if self.scopes.len() <= 1 {
            panic!("Cannot pop the global scope!");
        }
        self.scopes.pop();
    }
    pub fn define(&mut self, name: String, value: Value) {
        let current_scope = self.scopes.last_mut().expect("No active scope found");
        current_scope.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Cannot assign to undefined variable '{}'", name))
    }
}