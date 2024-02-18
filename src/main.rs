use std::io::{self, Write};
mod cat;
mod echo;

fn main() {
    loop {
        print!("osh> "); 
        // Flush to make sure prompt is printed before input.
        io::stdout().flush().unwrap(); 

        // Read input.
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let command = line.trim();
        // Exit conditions
        if command == "exit" || command == "quit" {
            break;
        }
        process_command(command);
    }
}

fn process_command(command: &str) {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    if command == "cat"{
        cat::process_command(args);
    }
    else if command == "echo" {
        echo::process_command(args);
    }
}
