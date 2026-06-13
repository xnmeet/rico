//! Parser module for converting Thrift IDL text into AST.
//!
//! This module implements a recursive descent parser that converts tokenized
//! Thrift IDL into an Abstract Syntax Tree (AST). The parser provides:
//!
//! - Complete Thrift IDL support
//! - Detailed error reporting
//! - Source location tracking
//! - Comment preservation
//!
//! # Parsing Process
//!
//! The parser operates in several stages:
//!
//! 1. Tokenization (via lexer)
//! 2. Recursive descent parsing
//! 3. AST construction
//! 4. Error handling and recovery
//!
//! # Error Handling
//!
//! The parser provides detailed error information:
//!
//! - Source location (line and column)
//! - Expected vs actual tokens
//! - Context about the construct being parsed
//! - Suggestions for common mistakes
//!
//! # Implementation Details
//!
//! ## Parser Design
//!
//! - Recursive descent parsing for clarity
//! - Look-ahead tokens for better error handling
//! - State tracking for context-sensitive parsing
//! - Comment attachment to AST nodes
//!
//! ## Performance Considerations
//!
//! - Efficient token handling
//! - Minimal memory allocation
//! - Fast error recovery
//! - Optimized AST construction
//!
//! # Usage Examples
//!
//! ## Basic Parsing
//!
//! ```rust
//! use rico::Parser;
//!
//! let input = r#"
//!     namespace rs example
//!     struct User {
//!         1: string name
//!     }
//! "#;
//!
//! let mut parser = Parser::new(input);
//! match parser.parse() {
//!     Ok(ast) => println!("Parsing successful"),
//!     Err(e) => eprintln!("Parse error: {}", e),
//! }
//! ```
//!
//! ## Error Handling
//!
//! ```rust
//! use rico::Parser;
//!
//! let input = r#"
//!     struct User {
//!         1: invalid_type name  // Error: invalid type
//!     }
//! "#;
//!
//! let mut parser = Parser::new(input);
//! match parser.parse() {
//!     Ok(_) => println!("Parsing successful"),
//!     Err(e) => {
//!         // Handle specific error cases
//!     }
//! }
//! ```

mod definitions;
mod error;
mod factory;
mod location;
mod token;
mod types;
mod values;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use error::ParseErrorKind;
use logos::Logos;

#[derive(Debug, Clone)]
pub struct ParserToken<'a> {
    pub text: &'a str,
    pub span: logos::Span,
    pub token: Token,
    pub start: Span,
    pub end: Span,
}

/// A Thrift IDL parser that produces a JSON AST representation.
///
/// The parser supports all standard Thrift features including:
/// - Base types and collections
/// - Structs, unions, and exceptions
/// - Services and functions
/// - Enums and constants
/// - Namespaces and includes
/// - Comments and annotations
///
/// # Example
///
/// ```rust
/// use rico::Parser;
///
/// let input = r#"
///     namespace rs demo
///     
///     struct User {
///         1: string name
///         2: i32 age
///     }
/// "#;
///
/// let mut parser = Parser::new(input);
/// match parser.parse() {
///     Ok(ast) => println!("{}", serde_json::to_string_pretty(&ast).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    cur_token: Option<ParserToken<'a>>,
    next_token: Option<ParserToken<'a>>,
    pending_comments: Vec<Comment>,
    last_span: logos::Span,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance for the given Thrift IDL input string.
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        let last_span = lexer.span().clone();
        Parser {
            lexer,
            next_token: None,
            cur_token: None,
            pending_comments: Vec::with_capacity(4),
            last_span,
        }
    }

    /// Parses the Thrift IDL input and returns a Document AST.
    ///
    /// The Document contains all parsed definitions including:
    /// - Namespaces
    /// - Includes
    /// - Constants
    /// - Typedefs
    /// - Enums
    /// - Structs
    /// - Unions
    /// - Exceptions
    /// - Services
    ///
    /// # Errors
    ///
    /// Returns a ParseError if the input contains syntax errors or unsupported features.
    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut members = Vec::with_capacity(self.lexer.source().len() / 128);

        loop {
            self.advance();
            self.skip_separator();
            self.skip_comments();
            self.skip_separator();
            if let Some(token) = self.token() {
                match token {
                    Token::Include => {
                        members.push(DocumentMembers::Include(self.parse_include()?));
                    }
                    Token::CppInclude => {
                        members.push(DocumentMembers::CppInclude(self.parse_cpp_include()?));
                    }
                    Token::Namespace => {
                        members.push(DocumentMembers::Namespace(self.parse_namespace()?));
                    }
                    Token::Const => members.push(DocumentMembers::Const(self.parse_const()?)),
                    Token::Typedef => members.push(DocumentMembers::Typedef(self.parse_typedef()?)),
                    Token::Enum => members.push(DocumentMembers::Enum(self.parse_enum()?)),
                    Token::Struct => members.push(DocumentMembers::Struct(self.parse_struct()?)),
                    Token::Union => members.push(DocumentMembers::Union(self.parse_union()?)),
                    Token::Exception => {
                        members.push(DocumentMembers::Exception(self.parse_exception()?))
                    }
                    Token::Service => members.push(DocumentMembers::Service(self.parse_service()?)),
                    _ => {
                        return Err(self.error(ParseErrorKind::UnexpectedToken));
                    }
                }
            } else {
                // if last_span.end == 1, it means the first token is unrecognized
                if self.last_span.end == 1 {
                    return Err(self.error(ParseErrorKind::UnrecognizedToken));
                }
                self.clear_pending_comments();
                break;
            }
        }

        Ok(Document { members })
    }

    fn create_parser_token(&mut self) -> Option<ParserToken<'a>> {
        let result = self.lexer.next()?;
        match result {
            Ok(token) => {
                let span = self.lexer.span();
                let text = self.lexer.slice();
                let start = self.bind_start_position(&span);
                let end = self.bind_end_position(&span, token);

                Some(ParserToken {
                    text,
                    span,
                    token,
                    start,
                    end,
                })
            }
            Err(_) => {
                self.last_span = self.lexer.span();
                None
            }
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        // If there's no next token and current token exists, clear the current token
        if self.next_token.is_none() && self.cur_token.is_some() {
            self.last_span = self.cur_token.as_ref().unwrap().span.clone();
            self.cur_token = None;
            return None;
        }

        // Get current token (happens for the first token)
        if self.cur_token.is_none() {
            self.cur_token = self.create_parser_token();
        } else {
            // Update current token
            self.cur_token = self.next_token.take(); // Move next_token to cur_token
        }

        // Get next token
        self.next_token = self.create_parser_token();

        self.cur_token.as_ref().map(|token| &token.token)
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(token) = &self.next_token {
            return Some(&token.token);
        }

        return None;
    }

    fn token(&self) -> Option<&Token> {
        if let Some(token) = &self.cur_token {
            return Some(&token.token);
        }

        return None;
    }

    fn text(&self) -> &'a str {
        if let Some(token) = &self.cur_token {
            return token.text;
        }
        return "";
    }

    fn bind_start_position(&self, span: &logos::Span) -> Span {
        let line = self.lexer.extras.0 + 1;
        let column = span.start.saturating_sub(self.lexer.extras.1) + 1;
        Span::new(line, column, span.start)
    }

    fn bind_end_position(&mut self, span: &logos::Span, token: Token) -> Span {
        let mut line = self.lexer.extras.0 + 1;
        let mut line_start = self.lexer.extras.1;

        if matches!(token, Token::BlockComment) {
            let mut newline_count = 0;
            for (offset, byte) in self.lexer.slice().bytes().enumerate() {
                if byte == b'\n' {
                    newline_count += 1;
                    line_start = span.start + offset + 1;
                }
            }
            self.lexer.extras.0 += newline_count;
            self.lexer.extras.1 = line_start;
            line = self.lexer.extras.0 + 1;
        }

        let column = span.end.saturating_sub(line_start) + 1;

        Span::new(line, column, span.end)
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        match &self.cur_token {
            Some(token) => ParseError::from_loc(token.span.clone(), kind),
            None => {
                if self.last_span.end == self.lexer.source().bytes().len() {
                    return ParseError::from_loc(
                        self.last_span.clone(),
                        ParseErrorKind::UnexpectedEOF,
                    );
                }
                ParseError::from_loc(self.last_span.clone(), ParseErrorKind::UnrecognizedToken)
            }
        }
    }
}
