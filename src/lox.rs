use std::process::exit;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::{models::constants::ExitCode, scanner::Scanner};

#[derive(Debug, Clone, Copy)]
pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, location, message);
    }

    pub fn run(&self, source_code: &str) {
        let mut scanner = Scanner::new(source_code.to_string());
        scanner.scan_tokens();

        if !scanner.errors.is_empty() {
            println!("Errors:");
            for error in scanner.errors {
                eprintln!("{}", error);
            }
            exit(ExitCode::DataError as i32);
        }

        let mut parser = Parser::new(scanner.tokens);
        parser.parse();

        if !parser.errors.is_empty() {
            println!("Errors:");
            for error in parser.errors {
                eprintln!("{}", error);
            }
            exit(ExitCode::DataError as i32);
        }

        let mut interpreter = Interpreter::new();
        interpreter.interpret(&parser.stmts);
    }
}
