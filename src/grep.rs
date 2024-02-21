use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn process_command(args: core::str::SplitWhitespace) {
    let args: Vec<&str> = args.collect();

    if args.len() < 2 {
        eprintln!("Usage: grep <pattern> <file>...");
        return;
    }

    // Remove leading and trailing quotation marks from the pattern
    let pattern = args[0].trim_matches('"');

    // Take multiple files as arguments
    let files = &args[1..];
    for file_name in files {
        match grep_file(file_name, pattern) {
            Ok(_) => {},
            Err(e) => eprintln!("Error processing '{}': {}", file_name, e),
        }
    }
}



fn grep_file(file_name: &str, pattern: &str) -> Result<(), io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    // Read each line and print if pattern matches
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}:{}:{}", file_name, index + 1, line);
        }
    }

    Ok(())
}
