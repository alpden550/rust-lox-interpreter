use crate::models::exr::Expr;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    While(Expr, Box<Stmt>),
    Return(Token, Option<Expr>),
    Var(String, Option<Expr>),
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Stmt::Expr(e) => write!(f, "expr {}", e),
            Stmt::Function(token, params, body) => {
                write!(f, "function {}({:?}, {:?})", token, params, body)
            }
            Stmt::If(c, t, e) => write!(f, "if {} then {} else {:?}", c, t, e),
            Stmt::Print(e) => write!(f, "print {}", e),
            Stmt::While(condition, body) => write!(f, "while loop {} do {}", condition, body),
            Stmt::Var(token, expr) => {
                if let Some(expr) = expr {
                    write!(f, "var {} = {}", token, expr)
                } else {
                    write!(f, "var {}", token)
                }
            }
            Stmt::Return(_token, expr) => {
                write!(f, "return {:?}", expr)
            }
            Stmt::Block(stmts) => {
                write!(f, "block {:?}", stmts)
            }
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visit_if_stmt(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_print_stmt(&mut self, expr: &Expr) -> T;
    fn visit_while_stmt(&mut self, cond: &Expr, body: &Stmt) -> T;
    fn visit_return_stmt(&mut self, token: &Token, expr: &Option<Expr>) -> T;
    fn visit_var_stmt(&mut self, lexeme: String, expr: &Option<Expr>) -> T;
    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
            Stmt::Function(name, params, body) => visitor.visit_function_stmt(name, params, body),
            Stmt::If(cond, then_b, else_b) => visitor.visit_if_stmt(cond, then_b, else_b),
            Stmt::Print(expr) => visitor.visit_print_stmt(expr),
            Stmt::While(cond, body) => visitor.visit_while_stmt(cond, body),
            Stmt::Return(token, expr) => visitor.visit_return_stmt(token, expr),
            Stmt::Var(lexeme, expr) => visitor.visit_var_stmt(lexeme.clone(), expr),
            Stmt::Block(stmts) => visitor.visit_block_stmt(stmts),
        }
    }
}
