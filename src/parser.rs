use crate::models::exr::Expr;
use crate::models::literals::Literal;
use crate::models::token_type::TokenType;
use crate::models::tokens::Token;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    length: usize,
    pub errors: Vec<String>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let length = tokens.len();
        Self {
            tokens,
            current: 0,
            length,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.expression() {
                Ok(expr) => {
                    println!("{}", expr);
                }
                Err(_) => {
                    self.synchronize();
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn match_tokens(&mut self, token_types: Vec<&TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(vec![&TokenType::BangEqual, &TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(vec![
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(vec![&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(vec![&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(vec![&TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_tokens(vec![&TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_tokens(vec![&TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_tokens(vec![&TokenType::Number, &TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_tokens(vec![&TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let error = "Expect expression.".to_string();
        self.error(self.peek().clone(), &error);
        Err(error)
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if !self.check(token_type) {
            self.error(self.peek().clone(), message);
            return Err(message.to_string());
        }
        Ok(self.advance())
    }

    fn error(&mut self, token: Token, message: &str) {
        let mut err = String::new();

        if token.token_type == TokenType::Eof {
            let msg = format!("line {} at end: {}", token.line, message);
            err.push_str(&msg);
        } else {
            let msg = format!("{} at '{}': {}", token.line, token.lexeme, message);
            err.push_str(&msg);
        }

        self.errors.push(err);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}
