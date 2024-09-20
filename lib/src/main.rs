use serde_json;
use std::fs;
use thrift_parser::parser::Parser; // Add this line to import the fs module

fn main() {
    let input = fs::read_to_string("tests/header.thrift").expect("Unable to read file"); // Handle potential errors

    let mut temp = Parser::new(&input);
    let result = temp.parse();
    match result {
        Ok(value) => {
            let json_output = serde_json::to_string(&value).expect("Failed to convert to JSON");
            println!("{}", json_output);
        }
        Err(e) => println!("Error: {}", e),
    }
}
