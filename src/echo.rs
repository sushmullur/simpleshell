pub fn process_command(args: core::str::SplitWhitespace) {
    let args: Vec<&str> = args.collect();

    let mut print_newline = true;
    let mut interpret_escapes = false;

    let mut processed_args = Vec::new();

    for arg in &args {
        match *arg {
            "-n" => print_newline = false,
            "-e" => interpret_escapes = true,
            "-E" => interpret_escapes = false,
            _ => processed_args.push(*arg),
        }
    }

    let mut output = processed_args.join(" ");
    if interpret_escapes {
        output = output.replace("\\n", "\n")
                       .replace("\\t", "\t")
                       .replace("\\\\", "\\")
                       ;
    }

    print!("{}", output);

    if print_newline {
        println!();
    }
}
