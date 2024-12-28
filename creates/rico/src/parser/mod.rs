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
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance for the given Thrift IDL input string.
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        Parser {
            lexer,
            next_token: None,
            cur_token: None,
            pending_comments: Vec::new(),
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
        let mut members = Vec::new();

        loop {
            self.advance();
            self.skip_comments();
            if let Some(token) = self.token() {
                match token {
                    Token::Include => {
                        members.push(DocumentMembers::Include(self.parse_include()?));
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
                self.clear_pending_comments();
                break;
            }
        }

        Ok(Document {
            kind: NodeType::ThriftDocument,
            members,
        })
    }

    fn advance(&mut self) -> Option<&Token> {
        // 如果没有下一个 token 且当前 token 存在，清空当前 token
        if self.next_token.is_none() && self.cur_token.is_some() {
            self.cur_token = None;
            return None;
        }

        // 更新当前 token
        self.cur_token = self.next_token.take(); // Move next_token to cur_token

        // 获取当前 token，发生于首 token
        if self.cur_token.is_none() {
            if let Some(Ok(token)) = self.lexer.next() {
                self.cur_token = Some(ParserToken {
                    text: self.lexer.slice(),
                    span: self.lexer.span(),
                    token,
                    start: self.bind_start_position(),
                    end: self.bind_end_position(),
                });
            }
        }

        // 获取下一个 token
        self.next_token = self
            .lexer
            .next()
            .map(|result| {
                result.map(|token| ParserToken {
                    text: self.lexer.slice(),
                    span: self.lexer.span(),
                    token,
                    start: self.bind_start_position(),
                    end: self.bind_end_position(),
                })
            })
            .transpose()
            .unwrap_or(None);

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

    fn text(&self) -> &str {
        if let Some(token) = &self.cur_token {
            return token.text;
        }
        return "";
    }

    fn bind_start_position(&mut self) -> Span {
        let source = self.lexer.source();
        let span = self.lexer.span();
        let start_index = source[..span.start].len();
        let column = source[self.lexer.extras.1..span.start].len() + 1;
        let line = self.lexer.extras.0 + 1;
        let index = start_index;
        Span::new(line, column, index)
    }

    fn bind_end_position(&mut self) -> Span {
        let span = self.lexer.span();
        let source = self.lexer.source();
        // handle inner multiple content
        let newline_count = self.lexer.slice().matches('\n').count();

        if newline_count > 0 {
            self.lexer.extras.0 += newline_count;
            self.lexer.extras.1 = self.lexer.span().end;
        }

        let line = self.lexer.extras.0 + 1;
        let column = source[self.lexer.extras.1..span.end].len() + 1;
        let index = source[..span.end].len();

        Span::new(line, column, index)
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        ParseError::from_loc(self.cur_token.clone().unwrap().span, kind)
    }
}

#[cfg(test)]
mod tests;
