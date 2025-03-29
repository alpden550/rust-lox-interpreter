use crate::models::literals::Literal;
use crate::models::token_type::TokenType;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type {} lexeme {} literal {}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
