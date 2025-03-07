use crate::models::literals::Literal;
use crate::models::token_type::{TokenType, KEYWORDS};
use crate::models::tokens::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Scanner {
    source: Vec<char>,
    source_text: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
    length: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let length = source.len();
        Self {
            source: source.chars().collect::<Vec<char>>(),
            source_text: source,
            tokens: vec![],
            errors: vec![],
            start: 0,
            current: 0,
            line: 1,
            length,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
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
        self.tokens.clone()
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
                let token_type = match self.matches('=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                };
                self.add_token_type(token_type);
            }
            '=' => {
                let token_type = match self.matches('=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                };
                self.add_token_type(token_type);
            }
            '<' => {
                let token_type = match self.matches('=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };
                self.add_token_type(token_type);
            }
            '>' => {
                let token_type = match self.matches('=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
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
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            _ => self
                .errors
                .push(format!("line {}: Unexpected character: {}", self.line, c)),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.length {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn add_token_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::Nil);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source_text[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push(format!("line {}: Unterminated string.", self.line));
            return;
        }

        self.advance();

        let value = &self.source_text[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Literal::String(value.to_string()));
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source_text[self.start..self.current];
        if let Ok(num) = value.parse::<f64>() {
            self.add_token(TokenType::Number, Literal::Number(num));
        }
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source_text[self.start..self.current];
        let token_type = KEYWORDS.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token_type(token_type.clone());
    }
}
