use std::env;
use std::io::{self, Write};
use lox::lexer::{Lexer};

fn main() {
    //some comments here //
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file = &args[1];
            run_file(file);
        }
        1 => {
            run_prompt();
        }
        _ => {
            panic!("Usage: lox [script]");
        }
    }
}

fn run_file(path: &str) {
    println!("We got a file path {}", path);
}

fn run_prompt() {
    print!("> ");
    let mut input = String::new();
    let _ = std::io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let _lexer = Lexer::new(&input);
}
