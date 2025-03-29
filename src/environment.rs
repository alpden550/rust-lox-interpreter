use crate::errors::RuntimeError;
use crate::models::literals::Literal;
use crate::models::tokens::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.values.get(&token.lexeme) {
            return Ok(value.clone());
        }

        match self.enclosing.as_ref() {
            Some(enclosing) => enclosing.borrow().get(token),
            None => Err(RuntimeError::UndefinedVariable(token.line, token.clone())),
        }
    }

    pub fn assign(&mut self, token: &Token, value: Literal) -> Result<Literal, RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.clone(), value.clone());
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        Err(RuntimeError::UndefinedVariable(token.line, token.clone()))
    }
}
