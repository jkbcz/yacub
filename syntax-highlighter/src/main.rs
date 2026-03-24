mod scanner;
mod token;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    println!("Coloring syntax in file {}", filename);

    let mut scanner = scanner::Scanner::new(filename.to_string()).expect("Failed to read file");
    let tokens = scanner.scan_all().expect("Failed to scan tokens");

    for token in tokens {
        println!("{}", token);
    }
}
