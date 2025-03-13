use crate::models::exr::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(String, Option<Expr>),
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Stmt::Expr(e) => write!(f, "expr {}", e),
            Stmt::Print(e) => write!(f, "print {}", e),
            Stmt::Var(token, expr) => {
                if let Some(expr) = expr {
                    write!(f, "var {} = {}", token, expr)
                } else {
                    write!(f, "var {}", token)
                }
            }
            Stmt::Block(stmts) => {
                write!(f, "block {:?}", stmts)
            }
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_var_stmt(&mut self, lexeme: String, expr: &Option<Expr>) -> T;
    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::Var(lexeme, expr) => visitor.visit_var_stmt(lexeme.clone(), expr),
            Stmt::Block(stmts) => visitor.visit_block_stmt(stmts),
        }
    }
}
