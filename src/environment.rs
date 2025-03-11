use crate::errors::RuntimeError;
use crate::models::literals::Literal;
use crate::models::tokens::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, RuntimeError> {
        self.values
            .get(&token.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedVariable(token.line, token.clone()))
    }

    pub fn assign(&mut self, token: &Token, value: Literal) -> Result<Literal, RuntimeError> {
        match self.values.get_mut(&token.lexeme) {
            Some(val) => {
                *val = value.clone();
                Ok(value)
            }
            None => Err(RuntimeError::UndefinedVariable(token.line, token.clone())),
        }
    }
}
