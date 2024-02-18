use std::io::{self, Write};
mod cat;

fn main() {
    loop {
        print!("osh> "); 
        io::stdout().flush().unwrap(); // Flush to make sure osh> is printed before input.

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap(); // Read input.
        let command = line.trim(); // Remove trailing newline.
        if command == "exit" {
            break;
        }
        process_command(command);
    }
}

fn process_command(command: &str) {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    if command == "cat" {
        cat::process_command(args);
    }
}
