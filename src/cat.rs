pub fn process_command(args: core::str::SplitWhitespace) {
    // Convert args to a Vec<&str> so we can access by index.
    let args: Vec<&str> = args.collect();

    // Check if we have at least one argument.
    if args.is_empty() {
        println!("Error: cat command requires at least one argument.");
        return;
    }

    // Iterate over each argument and attempt to read and print the file contents.
    for file_name in args {
        let contents = read_file(file_name);
        match contents {
            Ok(content) => println!("{}", content),
            Err(e) => println!("Error reading '{}': {}", file_name, e),
        }
    }
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    // Read the file and return its contents.
    std::fs::read_to_string(file_name)
}