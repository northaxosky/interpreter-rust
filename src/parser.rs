use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let token = &self.tokens[self.current];
        self.current += 1;
        match token.token_type {
            TokenType::True => Expr::Literal(LiteralValue::Bool(true)),
            TokenType::False => Expr::Literal(LiteralValue::Bool(false)),
            TokenType::Nil => Expr::Literal(LiteralValue::Nil),
            _ => todo!(),
        }
    }
}
