pub fn process_command(args: core::str::SplitWhitespace) {
    let args: Vec<&str> = args.collect();
    println!("cat command called with args: {:?}", args);
}