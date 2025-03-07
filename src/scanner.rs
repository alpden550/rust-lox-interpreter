use crate::models::literals::Literal;
use crate::models::token_type::{TokenType, KEYWORDS};
use crate::models::tokens::Token;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
    chars: Vec<char>,
    length: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().collect();
        let length = source.len();
        Self {
            source,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            chars,
            length,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Nil,
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.length
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token_type(TokenType::LeftParen),
            ')' => self.add_token_type(TokenType::RightParen),
            '{' => self.add_token_type(TokenType::LeftBrace),
            '}' => self.add_token_type(TokenType::RightBrace),
            ',' => self.add_token_type(TokenType::Comma),
            '.' => self.add_token_type(TokenType::Dot),
            '-' => self.add_token_type(TokenType::Minus),
            '+' => self.add_token_type(TokenType::Plus),
            ';' => self.add_token_type(TokenType::Semicolon),
            '*' => self.add_token_type(TokenType::Star),
            '!' => {
                let token_type = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token_type(token_type);
            }
            '=' => {
                let token_type = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token_type(token_type);
            }
            '<' => {
                let token_type = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token_type(token_type);
            }
            '>' => {
                let token_type = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token_type(token_type);
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_type(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),

            _ => self.errors.push(format!("line {}: Unexpected character: {}", self.line, c)),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }

        self.chars[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.length {
            return '\0'
        }

        self.chars[self.current + 1]
    }

    fn add_token_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::Nil);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            literal,
            self.line,
        ));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(format!("line {}: Unterminated string.", self.line));
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn number(&mut self) {
        self.consume_digits();

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();
            // Consume fractional digits
            self.consume_digits();
        }

        let value = &self.source[self.start..self.current];
        if let Ok(num) = value.parse::<f64>() {
            self.add_token(TokenType::Number, Literal::Number(num));
        } else {
            self.errors.push(format!("line {}: Invalid number: {}", self.line, value));
        }
    }

    fn consume_digits(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);
        self.add_token_type(token_type);
    }
}