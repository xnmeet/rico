//! Lexical analyzer for Thrift IDL.
//!
//! This module provides tokenization of Thrift IDL source text. It breaks down
//! the input into a sequence of tokens that can be processed by the parser.
//!
//! # Token Types
//!
//! The lexer recognizes several categories of tokens:
//!
//! ## Keywords
//! - Language constructs (namespace, struct, enum, etc.)
//! - Type names (i32, string, bool, etc.)
//! - Modifiers (required, optional, oneway)
//!
//! ## Identifiers
//! - Names of types, functions, fields, etc.
//! - Namespace scopes
//!
//! ## Literals
//! - Integer literals (decimal and hex)
//! - Floating point literals
//! - String literals (with escaping)
//! - Boolean literals (true/false)
//!
//! ## Punctuation
//! - Brackets and braces
//! - Operators and separators
//! - Field IDs and type parameters
//!
//! ## Comments
//! - Single-line comments (//)
//! - Multi-line comments (/* */)
//!
//! # Features
//!
//! - Fast tokenization using logos
//! - Detailed source location tracking
//! - Error recovery and reporting
//! - Comment preservation
//! - Support for Unicode identifiers
//!
//! # Implementation Details
//!
//! The lexer uses the logos crate for efficient tokenization and provides:
//! - Token type definitions
//! - Source location tracking
//! - Error handling and recovery
//! - Iterator interface for token stream

mod token;

pub use self::token::Token;
