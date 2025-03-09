use crate::errors::RuntimeError;
use crate::models::exr::{Expr, ExprVisitor};
use crate::models::literals::Literal;
use crate::models::token_type::TokenType;
use crate::models::tokens::Token;

pub struct Interpreter {
    pub errors: Vec<String>,
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal_expr(&self, literal: &Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_binary_expr(
        &self,
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

    fn visit_grouping_expr(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<Literal, RuntimeError> {
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
}

impl Interpreter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn interpret(&mut self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(e) => {
                println!("{}", e);
            }
            Err(e) => self.errors.push(e.to_string()),
        }
    }

    fn evaluate(&self, expression: &Expr) -> Result<Literal, RuntimeError> {
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
}
