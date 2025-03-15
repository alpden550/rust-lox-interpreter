use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::models::exr::{Expr, ExprVisitor};
use crate::models::literals::Literal;
use crate::models::stmt::{Stmt, StmtVisitor};
use crate::models::token_type::TokenType;
use crate::models::tokens::Token;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub errors: Vec<String>,
    env: Rc<RefCell<Environment>>,
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left_value = self.evaluate(left)?;

        match operator.token_type {
            TokenType::Or if self.is_truthy(&left_value) => Ok(left_value),
            TokenType::And if !self.is_truthy(&left_value) => Ok(left_value),
            TokenType::Or | TokenType::And => self.evaluate(right),
            _ => Err(RuntimeError::UndefinedOperation(
                operator.line,
                format!("Unknown logical operator: {}", operator.token_type),
            )),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::Slash => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => {
                    if *r == 0.0 {
                        Err(RuntimeError::DivisionByZero(
                            operator.line,
                            format!("{l} / {r}"),
                        ))
                    } else {
                        Ok(Literal::Number(l / r))
                    }
                }
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}"),
                )),
            },
            TokenType::Star => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::Plus => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers or strings, got {left} and {right}",),
                )),
            },
            TokenType::Greater => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l > r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l >= r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::Less => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l < r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::LessEqual => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(l <= r)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operands must be numbers, got {left} and {right}",),
                )),
            },
            TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
            _ => Err(RuntimeError::UndefinedOperation(
                operator.line,
                format!("Unknown operator: {}", operator.token_type),
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(
        &mut self,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(RuntimeError::TypeError(
                    operator.line,
                    format!("Operand must be a number, got {:?}", right),
                )),
            },
            TokenType::Bang => Ok(Literal::Boolean(!self.is_truthy(&right))),
            _ => Err(RuntimeError::UndefinedOperation(
                operator.line,
                format!("Unknown operator: {:?}", operator.token_type),
            )),
        }
    }

    fn visit_variable_expr(&mut self, token: &Token) -> Result<Literal, RuntimeError> {
        self.env.borrow().get(token)
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(expr)?;
        self.env.borrow_mut().assign(token, value)
    }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        cond: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), RuntimeError> {
        let condition = self.evaluate(cond)?;

        if self.is_truthy(&condition) {
            self.execute(then_branch)
        } else {
            match else_branch {
                Some(stmt) => self.execute(stmt),
                None => Ok(()),
            }
        }
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, cond: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        loop {
            let condition = self.evaluate(cond)?;
            if !self.is_truthy(&condition) {
                break;
            }
            self.execute(body)?;
        }
        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        lexeme: String,
        initializer: &Option<Expr>,
    ) -> Result<(), RuntimeError> {
        let value = initializer
            .as_ref()
            .map_or_else(|| Ok(Literal::Nil), |expr| self.evaluate(expr))?;

        self.env.borrow_mut().define(lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        let new_env = Rc::new(RefCell::new(Environment::new_with_enclosing(&self.env)));
        self.execute_block(stmts, new_env)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(e) => self.log_error(e),
            }
        }
    }

    fn log_error(&mut self, error: RuntimeError) {
        self.errors.push(error.to_string());
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Literal, RuntimeError> {
        expression.accept(self)
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Nil => false,
            Literal::Boolean(b) => *b,
            Literal::Number(n) => *n != 0.0,
            Literal::String(s) => !s.is_empty(),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        stmts: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous = self.env.clone();
        self.env = env;

        let result: Result<(), RuntimeError> = (|| {
            for stmt in stmts {
                stmt.accept(self)?;
            }
            Ok(())
        })();

        self.env = previous;

        if let Err(ref e) = result {
            self.log_error(e.clone());
        }
        result
    }
}
