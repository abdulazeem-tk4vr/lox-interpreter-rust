use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // // You can use print statements as follows for debugging, they'll be visible when running tests.
            // writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();
            // adding code to figure out what is happening

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let file_contents_chars = file_contents.chars();
                let _ = file_contents_chars.for_each(|char| match char {
                    '(' => println!("LEFT_PAREN ( null"),
                    ')' => println!("RIGHT_PAREN ) null"),
                    _ => {}
                });
                println!("EOF  null");
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
