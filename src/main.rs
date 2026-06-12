#![allow(unused_variables)]
use std::env;
use std::fs;

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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut had_error = false;
            for c in file_contents.chars() {
                match c {
                    '(' => println!("LEFT_PAREN ( null"),
                    ')' => println!("RIGHT_PAREN ) null"),
                    '{' => println!("LEFT_BRACE {{ null"),
                    '}' => println!("RIGHT_BRACE }} null"),
                    ',' => println!("COMMA , null"),
                    '.' => println!("DOT . null"),
                    '-' => println!("MINUS - null"),
                    '+' => println!("PLUS + null"),
                    ';' => println!("SEMICOLON ; null"),
                    '/' => println!("SLASH / null"),
                    '*' => println!("STAR * null"),
                    _ => {
                        eprintln!("[line 1] Error: Unexpected character: {c}");
                        had_error = true;
                    }
                }
            }
            println!("EOF  null");

            if had_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
