mod lox;
mod models;
mod parser;
mod scanner;

use lox::Lox;
use models::constants::ExitCode;
use std::{env, fs, process::exit};

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rlox.sh [script]");
        exit(ExitCode::IncorrectCommand as i32);
    });

    let source_code = fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("Error reading file '{}'", filename);
        exit(ExitCode::NoInputFile as i32);
    });

    let lox = Lox::new();
    lox.run(&source_code);
}
