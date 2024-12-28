# Rico-scan

A high-performance CLI tool for parsing and validating Thrift IDL files, built on top of the [Rico](https://crates.io/crates/rico) parser.

## Features

- 🚀 Fast parallel processing of Thrift files
- 🎯 Detailed error reporting with source context using [miette](https://crates.io/crates/miette)
- 📝 Optional JSON AST output
- 📊 Progress indication with ETA
- 🎨 Colorful and informative terminal output

## Installation

```bash
cargo install rico-scan
```

## Usage

### Basic Validation

To validate Thrift files without generating output:

```bash
rico-scan -p /path/to/thrift/files
```

### Generate JSON AST

To parse files and output JSON AST:

```bash
rico-scan -p /path/to/thrift/files -o /path/to/output
```

Each Thrift file will be parsed into a corresponding JSON file containing its AST.

## Features

- `json-output` (enabled by default): Enables JSON AST output functionality

## Example Output

```
[=========================>   ] 495/522 (28s)
Done! • ✅ succeeded: 521 • ❌ failed: 1 • ⚡ threads: 8 • ⏱ time: 1.23s
```

If errors are found, they will be displayed with source context:

```
Error details:
→ /path/to/file.thrift
<detailed error with source context>
```

## License

This project is licensed under the same terms as Rico.
