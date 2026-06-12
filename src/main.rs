mod token;
use std::{env, fs};
use token::{Token, TokenType};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let (tokens, had_error) = scan(&file_contents);
            for t in &tokens {
                println!("{t}");
            }

            if had_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn scan(source: &str) -> (Vec<Token>, bool) {
    let mut tokens = Vec::new();
    let mut had_error = false;
    let mut chars = source.chars().peekable();
    let line = 1;

    while let Some(c) = chars.next() {
        match c {
            // 1 Char: Simple case
            '(' => tokens.push(Token::new(TokenType::LeftParen, c.to_string(), line)),
            ')' => tokens.push(Token::new(TokenType::RightParen, c.to_string(), line)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, c.to_string(), line)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, c.to_string(), line)),
            ',' => tokens.push(Token::new(TokenType::Comma, c.to_string(), line)),
            '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), line)),
            '-' => tokens.push(Token::new(TokenType::Minus, c.to_string(), line)),
            '+' => tokens.push(Token::new(TokenType::Plus, c.to_string(), line)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string(), line)),
            '*' => tokens.push(Token::new(TokenType::Star, c.to_string(), line)),

            // 2 Chars: Forward peak next char using peek()
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), line));
                } else {
                    tokens.push(Token::new(TokenType::Equal, c.to_string(), line));
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), line));
                } else {
                    tokens.push(Token::new(TokenType::Bang, c.to_string(), line));
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), line));
                } else {
                    tokens.push(Token::new(TokenType::Less, c.to_string(), line));
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), line));
                } else {
                    tokens.push(Token::new(TokenType::Greater, c.to_string(), line));
                }
            }
            '/' => {
                if chars.peek() == Some(&'/') {
                    // It's a comment, skip this line
                    while let Some(&next) = chars.peek() {
                        if next == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    tokens.push(Token::new(TokenType::Slash, c.to_string(), line));
                }
            }
            // Unexpected Character
            _ => {
                eprintln!("[line {line}] Error: Unexpected character: {c}");
                had_error = true;
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, String::new(), line));
    (tokens, had_error)
}
