use crate::models::literals::Literal;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", left, operator.lexeme, right)
            }
            Expr::Unary(operator, right) => write!(f, "({} {})", operator.lexeme, right),
            Expr::Grouping(expression) => write!(f, "(group {})", expression),
        }
    }
}
