use std::process::exit;

use crate::parser::Parser;
use crate::{models::constants::ExitCode, scanner::Scanner};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Lox {
    pub has_error: bool,
}

#[allow(dead_code)]
impl Lox {
    pub fn new() -> Self {
        Self { has_error: false }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
        self.has_error = true;
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error: {}: {}", line, location, message);
    }

    pub fn run(&self, source_code: &str) {
        let mut scanner = Scanner::new(source_code.to_string());
        scanner.scan_tokens();

        if !scanner.errors.is_empty() {
            println!("Errors:");
            for error in scanner.errors {
                println!("{}", error);
            }
            exit(ExitCode::DataError as i32);
        }

        for token in &scanner.tokens {
            println!("{}", token);
        }

        let mut parser = Parser::new(scanner.tokens);
        parser.parse();

        if !parser.errors.is_empty() {
            println!("Errors:");
            for error in parser.errors {
                println!("{}", error);
            }
            exit(ExitCode::DataError as i32);
        }
    }
}
