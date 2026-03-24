mod formatter;
mod scanner;
mod token;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_filename = &args[1];
    let output_filename = &args[2];

    println!(
        "Syntax highlighting: {} -> {}",
        input_filename, output_filename
    );

    let mut scanner =
        scanner::Scanner::new(input_filename.to_string()).expect("Failed to read input file");
    let tokens = scanner.scan_all().expect("Failed to scan tokens");
    let source = scanner.source();

    // Generate HTML
    let html = formatter::to_html(&tokens, source);

    // Write to output file
    std::fs::write(output_filename, html).expect("Failed to write output file");

    println!(
        "Syntax highlighting complete! Output written to: {}",
        output_filename
    );
}
