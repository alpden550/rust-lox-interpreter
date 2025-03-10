use crate::models::exr::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Stmt::Expr(e) => write!(f, "expr {}", e),
            Stmt::Print(e) => write!(f, "print {}", e),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
        }
    }
}
