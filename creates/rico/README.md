# Thrift Parser

A high-performance Apache Thrift IDL parser written in Rust that converts Thrift IDL files to JSON AST.

## Features

- 🚀 Fast and efficient parsing
- 🎯 Practical Thrift IDL support with a stable JSON AST
- 🔄 JSON AST output
- 📝 Comment preservation
- 🎨 Detailed source location tracking
- ⚡ Parallel processing support
- 📊 Built-in benchmarking

## Installation

```bash
[dependencies]
rico = { version = "*" }
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

## Supported Thrift Syntax

Rico focuses on the mainstream Thrift IDL surface used by JS and Rust tooling while keeping the JSON `Document` AST stable.

### Types

- Base types: `bool`, `byte`, `i8`, `i16`, `i32`, `i64`, `double`, `string`, `binary`
- Identifier types: user-defined structs, enums, exceptions, unions, typedefs, and qualified names such as `shared.User`
- Containers: `list<T>`, `set<T>`, `map<K, V>`, including nested containers

### Definitions

- Headers: `include`, `cpp_include`, `namespace`, and `namespace *`
- `const`, `typedef`, `enum`, `struct`, `union`, `exception`, `service`
- Service `extends`, `throws`, and `oneway`
- Field ids, `required`, `optional`, and default-required fields
- Const/default values: strings, integers, hex integers, doubles, scientific notation, booleans, identifiers, lists, and maps
- Annotations on fields, enum members, definitions, functions, and services
- Comments: `//`, `#`, and `/* ... */`

### Intentional Non-Goals

Rare or legacy Thrift extensions are not currently parsed, including `senum`, `uuid`, container `cpp_type`, and XSD-specific options such as `xsd_all`, `xsd_optional`, `xsd_nillable`, and `xsd_attrs`. Keeping these out preserves a smaller parser surface and a stable AST for JS ecosystem consumers.

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
