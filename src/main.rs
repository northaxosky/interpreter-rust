mod expr;
mod parser;
mod scanner;
mod token;
use parser::Parser;
use scanner::Scanner;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let (tokens, had_error) = Scanner::new(&file_contents).scan_tokens();
            for tok in tokens {
                println!("{tok}")
            }

            if had_error {
                std::process::exit(65);
            }
        }
        "parse" => {
            let (tokens, had_error) = Scanner::new(&file_contents).scan_tokens();
            if had_error {
                std::process::exit(65);
            }
            let mut parser = Parser::new(tokens);
            let expr = parser.parse();
            println!("{expr}")
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
