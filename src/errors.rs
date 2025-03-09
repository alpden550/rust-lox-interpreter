use crate::models::token_type::TokenType;
use crate::models::tokens::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(f, "line {} at end: {}", self.token.line, self.message)
        } else {
            write!(
                f,
                "line {} at '{}': {}",
                self.token.line, self.token.lexeme, self.message
            )
        }
    }
}

pub enum RuntimeError {
    TypeError(usize, String),
    DivisionByZero(usize, String),
    UndefinedOperation(usize, String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::TypeError(line, msg) => write!(f, "line {line}: {msg}"),
            RuntimeError::DivisionByZero(line, msg) => {
                write!(f, "line {line}: Division by zero: {msg}")
            }
            RuntimeError::UndefinedOperation(line, msg) => write!(f, "line {line}: {msg}"),
        }
    }
}
