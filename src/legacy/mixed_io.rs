use std::io::{self, Write};

fn main() {
    // Write to stdout
    print!("Processing");
    io::stdout().flush().unwrap();
    
    // Simulate work with output to stderr
    for i in 1..=3 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        eprint!(".");
        io::stderr().flush().unwrap();
        
        print!(".");
        io::stdout().flush().unwrap();
    }
    
    println!("\nDone!");
    eprintln!("Process completed successfully");
}
