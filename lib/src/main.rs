use std::fs;
use thrift_parser::parser::Parser; // Add this line to import the fs module

fn main() {
    let input = fs::read_to_string("tests/header.thrift").expect("Unable to read file"); // Handle potential errors

    let mut temp = Parser::new(&input);
    let result = temp.parse();

    match result {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("Error: {}", e),
    }
}
