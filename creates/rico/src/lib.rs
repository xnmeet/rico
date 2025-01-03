//! Rico is a high-performance Apache Thrift IDL parser and writer library.
//!
//! Rico provides functionality to parse Thrift IDL files into an AST (Abstract Syntax Tree)
//! and write the AST back to Thrift IDL format. It aims to be a complete solution for
//! working with Thrift IDL files in Rust.
//!
//! # Features
//!
//! - Fast and efficient parsing with detailed error reporting
//! - Complete Thrift IDL support including all standard constructs
//! - Bidirectional conversion between Thrift IDL and AST
//! - Preservation of comments and formatting
//! - Detailed source location tracking for debugging
//! - Support for annotations and custom metadata
//! - Clean and well-documented API
//!
//! # Modules
//!
//! - [`ast`]: Abstract Syntax Tree definitions and types. Provides the core data structures
//!   that represent Thrift IDL constructs in memory.
//!
//! - [`lexer`]: Tokenization of Thrift IDL input. Breaks down source text into a sequence
//!   of tokens for the parser to process.
//!
//! - [`parser`]: Parsing of tokens into AST. Implements recursive descent parsing with
//!   detailed error reporting and recovery.
//!
//! - [`writer`]: Converting AST back to Thrift IDL text. Handles proper formatting,
//!   indentation, and comment preservation.
//!
//! # Getting Started
//!
//! Rico can be used in two main ways:
//!
//! 1. Parsing Thrift IDL into AST for analysis or transformation
//! 2. Writing AST back to Thrift IDL text
//!
//! ## Parsing Example
//!
//! ```rust
//! use rico::Parser;
//!
//! fn main() {
//!     let input = r#"
//!         namespace rs demo
//!
//!         struct User {
//!             1: string name
//!             2: i32 age
//!         }
//!     "#;
//!
//!     let mut parser = Parser::new(input);
//!     match parser.parse() {
//!         Ok(ast) => println!("{}", serde_json::to_string_pretty(&ast).unwrap()),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## Writing Example
//!
//! ```rust
//! use rico::{Parser, Writer};
//!
//! fn main() {
//!     let input = r#"
//!         struct User {
//!             1: string name
//!             2: i32 age
//!         }
//!     "#;
//!
//!     // Parse the input into an AST
//!     let mut parser = Parser::new(input);
//!     if let Ok(ast) = parser.parse() {
//!         // Create a writer and convert AST back to Thrift IDL
//!         let mut writer = Writer::new();
//!         let output = writer.write(&ast);
//!         println!("{}", output);
//!     }
//! }
//! ```
//!
//! # Supported Thrift Features
//!
//! ## Types
//! - Base types (bool, byte, i16, i32, i64, double, string, binary)
//! - Container types (list, set, map)
//! - User-defined types (struct, union, exception, enum)
//!
//! ## Definitions
//! - Constants with complex values (numbers, strings, lists, maps)
//! - Typedefs for type aliases and documentation
//! - Enums with optional values and annotations
//! - Structs with required/optional fields and defaults
//! - Unions for variant types with field IDs
//! - Exceptions for error handling with inheritance
//! - Services with inheritance and function modifiers
//!
//! ## Other Features
//! - Namespaces for different target languages
//! - Include statements for modular IDL organization
//! - Field IDs and requiredness specifiers
//! - Default values for fields
//! - Oneway function modifiers
//! - Throws clauses for error handling
//! - Single and multi-line comments
//! - Annotations for metadata and custom attributes
//!
//! # Error Handling
//!
//! The library provides detailed error reporting for both parsing and writing operations.
//! Errors include:
//! - Source location information (line and column)
//! - Descriptive error messages
//! - Expected vs actual token information
//! - Context about the construct being processed
//! - Suggestions for fixing common issues
//!
//! # Best Practices
//!
//! 1. Always check for errors when parsing
//! 2. Use the writer to maintain consistent formatting
//! 3. Preserve comments and annotations when modifying AST
//! 4. Handle all error cases appropriately
//! 5. Validate AST modifications before writing

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod writer;

pub use ast::*;
pub use parser::Parser;
pub use writer::Writer;
