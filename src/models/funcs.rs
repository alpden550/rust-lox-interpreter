use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::interpreter::Interpreter;
use crate::models::literals::Literal;
use crate::models::stmt::Stmt;
use crate::models::tokens::Token;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Native {
        name: String,
        arity: usize,
        body: fn(Vec<Literal>) -> Result<Literal, RuntimeError>,
    },
    Lox {
        name: String,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native { name, .. } => write!(f, "<native fn {}>", name),
            Function::Lox { name, .. } => write!(f, "<lox fn {}>", name),
        }
    }
}

impl Function {
    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
            Function::Lox { params, .. } => params.len(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        match self {
            Function::Native { body, .. } => body(args),
            Function::Lox {
                params,
                body,
                closure,
                ..
            } => {
                if args.len() != params.len() {
                    return Err(RuntimeError::TypeError(
                        0,
                        format!("Expected {} args, got {}", params.len(), args.len()),
                    ));
                }

                let env = Rc::new(RefCell::new(Environment::new_with_enclosing(closure)));

                {
                    let mut env_borrow = env.borrow_mut();
                    for (param, arg) in params.iter().zip(args) {
                        env_borrow.define(param.lexeme.clone(), arg);
                    }
                }

                match interpreter.execute_block(body, env) {
                    Ok(()) => Ok(Literal::Nil),
                    Err(RuntimeError::Return(_, value)) => Ok(value),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
