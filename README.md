# Thrift Parser

A high-performance Apache Thrift IDL parser written in Rust that converts Thrift IDL files to JSON AST.

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/rico.svg)](https://crates.io/crates/rico)
[![Documentation](https://docs.rs/rico/badge.svg)](https://docs.rs/rico)
[![License](https://img.shields.io/github/license/xnmeet/rico)](LICENSE)
[![Build Status](https://github.com/xnmeet/rico/workflows/Build%20and%20Test/badge.svg)](https://github.com/xnmeet/rico/actions)
[![Crates.io Downloads](https://img.shields.io/crates/d/rico)](https://crates.io/crates/rico)

</div>

## Features

- 🚀 Fast and efficient parsing
- 🎯 Complete Thrift IDL support
- 🔄 JSON AST output
- 📝 Comment preservation
- 🎨 Detailed source location tracking
- ⚡ Parallel processing support
- 📊 Built-in benchmarking

## Installation

```bash
[dependencies]
rico = "*"
```

## Usage

### Basic Parsing

```rust
use rico::Parser;

fn main() {
    let input = r#"
        namespace rs demo

        struct User {
            1: string name
            2: i32 age
        }
    "#;

    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(ast) => println!("{}", serde_json::to_string_pretty(&ast).unwrap()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Command Line Tools

#### Scan Multiple Files

The project includes a CLI tool for processing multiple Thrift files:

```bash
cargo run -p rico-scan -- --path ./thrift/files --output ./json/output
```

#### Benchmark Parser Performance

Run benchmarks on Thrift files:

```bash
cargo run -p benchmark
```

## Project Structure

- `creates/rico/`: Core parser library
- `apps/scan/`: CLI tool for batch processing
- `apps/benchmark/`: Performance benchmarking tool

## Supported Thrift Features

- Base types (i32, i64, string, etc.)
- Collections (list, set, map)
- Structs and Exceptions
- Services and Functions
- Enums
- Constants
- Typedefs
- Namespaces
- Includes
- Comments and Annotations

## Development

### Building

```bash
cargo build --workspace
```

### Running Tests

```bash
cargo test --workspace
```

### Code Structure

- Lexer: Tokenizes input using Logos
- Parser: Recursive descent parser
- AST: Strongly typed syntax tree
- Location Tracking: Preserves source positions

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Logos](https://github.com/maciejhirsz/logos) for lexer generation
- [Serde](https://github.com/serde-rs/serde) for JSON serialization
