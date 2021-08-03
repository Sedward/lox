use std::env;
use std::io::{self, Write};

fn main() {
    //some comments here //
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: lox [script]")
    } else if args.len() == 2 {
        let file = &args[1];
        run_file(file);
    
    } else {
        run_prompt();
    }
}

fn run_file(path: &String){
    print!("We got a file path {}\n", path);
}

fn run_prompt(){
    print!("> ");
    let mut input = String::new();
    let _= std::io::stdout().flush();
    io::stdin().read_line(&mut input).expect("failed to read input");
    print!("We got the input {}\n", input);
}


#[derive(Debug)]
struct Scanner {
    source: String
}

impl Scanner {
    pub fn scan_tokens(&self) -> Vec<&str>{
        self.source.split(" ").collect()
    }
}