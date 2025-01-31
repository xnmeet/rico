<div align="center">
    <img src="./apps/web/public/logo.svg" width="80px" height="80px"></img>
    <h1>Rico</h1>
</div>
<div align="center">
A high-performance Apache Thrift IDL parser written in Rust that converts Thrift IDL files to JSON AST.
</div>
<br/>

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

## Performance vs thrift-parser

The program takes `0.03` seconds, which is nearly a 10x performance improvement compared to the JavaScript implementation.

> The test file contains approximately 15,000 lines of code.

![performance](./performance.png)

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

You can also use the `rico-scan` command to scan multiple files and output the JSON AST to a folder.

```bash
cargo install rico-scan
```

Now you can use the `rico-scan` CLI tool for processing multiple Thrift files:

```bash
rico-scan --path ./thrift/files --output ./output
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

### Setup

```bash
# Install insta
curl -LsSf https://insta.rs/install.sh | sh

# Install pnpm
npm install -g pnpm bun

# Install dependencies
pnpm install
```

### Building

*build crates*

```bash
cargo build --workspace
```

*build docs*

```bash
cargo doc --workspace
```

*build wasm*

```bash
cd wasm/rico && npm run build

// debug use bun
bun examples/basic.ts
```

### Updating Snapshots

You can for instance first run the tests and not write and new snapshots, and if you like them run the tests again and update them:

```bash
INSTA_UPDATE=no cargo test
INSTA_UPDATE=always cargo test
```

For more information see [insta](https://insta.rs/docs/quickstart/)

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
