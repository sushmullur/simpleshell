pub fn process_command(args: core::str::SplitWhitespace) {
    // Convert args to a Vec<&str> so we can access by index.
    let args: Vec<&str> = args.collect();
    println!("{}", args.join(" "));
}