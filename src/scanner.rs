use crate::token::{Literal, Token, TokenType};

#[derive(Debug, Clone)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

// Minor helper free functions
fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(), // iterator -> Vec<char>
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(mut self) -> (Vec<Token>, bool) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));
        (self.tokens, self.had_error)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // 1 Char: simple case
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // 2 Chars: read forward
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // Values: literal string & numbers, identifers & keywords
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if is_alpha(c) => self.identifier(),

            // Whitespace: skip or advance line
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,

            // Unexpected token
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {c}", self.line);
                self.had_error = true;
            }
        }
    }

    // Literal constructors
    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.had_error = true;
            return;
        }
        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, Literal::Str(value));
    }

    fn number(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        // fractional part: a '.' followed by a digit
        if self.peek() == Some('.') && matches!(self.peek_next(), Some(c) if c.is_ascii_digit()) {
            self.advance();
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = lexeme.parse().unwrap();
        self.add_token_literal(TokenType::Number, Literal::Num(value));
    }

    fn identifier(&mut self) {
        while matches!(self.peek(), Some(c) if is_alpha_numeric(c)) {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier, // not a keyword -> plain identifier
        };
        self.add_token(token_type);
    }

    // Minor helper functions
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ty, lexeme, self.line));
    }

    fn add_token_literal(&mut self, ty: TokenType, literal: Literal) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::with_literal(ty, lexeme, self.line, Some(literal)));
    }
}
