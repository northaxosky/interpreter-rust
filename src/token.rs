use std::fmt;

#[derive(Debug, Clone)]
// Alternatively: #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    // 1 Char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 2 Chars
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    String,
    Number,

    // Keywords & Identifier
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Identifier,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS",
            TokenType::Plus => "PLUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Slash => "SLASH",
            TokenType::Star => "STAR",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Eof => "EOF",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::Fun => "FUN",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
            TokenType::Identifier => "IDENTIFIER",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Str(n) => write!(f, "{n}"),
            Literal::Num(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{n:.1}") // Integer: force one decimal
                } else {
                    write!(f, "{n}")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal: None,
        }
    }

    pub fn with_literal(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<Literal>,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.literal {
            Some(lit) => write!(f, "{} {} {}", self.token_type, self.lexeme, lit),
            None => write!(f, "{} {} null", self.token_type, self.lexeme),
        }
    }
}
