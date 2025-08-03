use std::io::{self, BufRead};

fn main() {
    println!("Enter lines of text (Ctrl+D to finish):");
    
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        match line {
            Ok(text) => {
                if text.trim().is_empty() {
                    continue;
                }
                println!("Echo: {}", text);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
    
    println!("Done processing input.");
}
