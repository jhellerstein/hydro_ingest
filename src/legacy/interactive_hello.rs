use std::io::{self, BufRead};

fn main() {
    println!("What's your name?");
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut name = String::new();
    
    match handle.read_line(&mut name) {
        Ok(_) => {
            let name = name.trim();
            println!("Hello, {}!", name);
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
