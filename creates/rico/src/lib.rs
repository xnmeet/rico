//! Rico is a high-performance Apache Thrift IDL parser that converts Thrift IDL files to JSON AST.
//!
//! # Features
//!
//! - Fast and efficient parsing
//! - Complete Thrift IDL support
//! - JSON AST output
//! - Comment preservation
//! - Detailed source location tracking
//! - Parallel processing support
//!
//! # Example
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
//! # Supported Thrift Features
//!
//! - Base types (i32, i64, string, etc.)
//! - Collections (list, set, map)
//! - Structs and Exceptions
//! - Services and Functions
//! - Enums
//! - Constants
//! - Typedefs
//! - Namespaces
//! - Includes
//! - Comments and Annotations

pub mod ast;
pub mod lexer;
pub mod parser;

pub use ast::*;
pub use parser::Parser;
