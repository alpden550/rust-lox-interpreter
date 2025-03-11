use crate::errors::RuntimeError;
use crate::models::literals::Literal;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
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
            Expr::Variable(token) => write!(f, "variable {}", token.lexeme),
            Expr::Assign(token, expr) => write!(f, "assign {} = {}", token.lexeme, expr),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<T, RuntimeError>;
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<T, RuntimeError>;
    fn visit_variable_expr(&mut self, token: &Token) -> Result<T, RuntimeError>;
    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<T, RuntimeError>;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
            Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            Expr::Variable(token) => visitor.visit_variable_expr(token),
            Expr::Assign(token, expr) => visitor.visit_assign_expr(token, expr),
        }
    }
}
