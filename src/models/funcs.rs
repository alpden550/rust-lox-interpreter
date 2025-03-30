use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::interpreter::Interpreter;
use crate::models::literals::Literal;
use crate::models::stmt::Stmt;
use crate::models::tokens::Token;
use std::cell::RefCell;
use std::fmt::Display;
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
        // closure: Rc<RefCell<Environment>>
    },
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}", self.to_string())
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
            Function::Lox { params, body, .. } => self.call_lox(interpreter, params, body, args),
        }
    }

    fn call_lox(
        &self,
        interpreter: &mut Interpreter,
        params: &[Token],
        body: &[Stmt],
        args: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        if args.len() != params.len() {
            return Err(RuntimeError::TypeError(
                0,
                format!("Expected {} args, got {}", params.len(), args.len()),
            ));
        }

        let env = Rc::new(RefCell::new(Environment::new_with_enclosing(
            &interpreter.globals,
        )));

        params.iter().zip(args.iter()).for_each(|(param, arg)| {
            env.borrow_mut().define(param.lexeme.clone(), arg.clone());
        });

        match interpreter.execute_block(body, env) {
            Err(RuntimeError::Return(_, value)) => Ok(value),
            Err(e) => Err(RuntimeError::UndefinedOperation(
                0,
                format!("Error in function call: {}", e),
            )),
            Ok(..) => Ok(Literal::Nil),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Function::Native { name, .. } => format!("<native fn {}>", name),
            Function::Lox { name, .. } => format!("<lox fn {}>", name),
        }
    }
}
