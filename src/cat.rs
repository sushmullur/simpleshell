pub fn process_command(args: core::str::SplitWhitespace) {
    let args: Vec<&str> = args.collect();
    println!("cat command called with args: {:?}", args);
    if args.len() < 1 {
        panic!("Error: cat command requires at least one argument.");
    }
    let contents = read_file(args[0]);
    println!("{}", contents.unwrap());
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    if !std::path::Path::new(file_name).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
    }
    std::fs::read_to_string(file_name)
}