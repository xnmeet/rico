use std::fmt;

use super::Span;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Span),
    UnexpectedEOF(Span),
    InvalidInteger(Span),
    InvalidFloat(Span),
    LexerError(Span),
    NestedComplexType(Span),
    UnsupportedType(Span),
    MissingTypeDeclaration(Span),
    InvalidValueDeclaration(Span),
    InvalidReturnType(Span),
    InvalidFieldName(Span),
    InvalidFieldId(Span),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(loc) => {
                write!(f, "Unexpected token:{}:{}", loc.line, loc.column)
            }
            ParseError::UnexpectedEOF(loc) => {
                write!(f, "Unexpected end of file:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidInteger(loc) => {
                write!(f, "Invalid integer:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidFloat(loc) => {
                write!(f, "Invalid float:{}:{}", loc.line, loc.column)
            }
            ParseError::LexerError(loc) => {
                write!(f, "Lexer error:{}:{}", loc.line, loc.column)
            }
            ParseError::NestedComplexType(loc) => {
                write!(
                    f,
                    "Nested complex types are not allowed:{}:{}",
                    loc.line, loc.column
                )
            }
            ParseError::UnsupportedType(loc) => {
                write!(f, "Unsupported type format:{}:{}", loc.line, loc.column)
            }
            ParseError::MissingTypeDeclaration(loc) => {
                write!(f, "Missing type declaration:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidValueDeclaration(loc) => {
                write!(f, "Invalid value declaration:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidReturnType(loc) => {
                write!(f, "Invalid return type:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidFieldName(loc) => {
                write!(f, "Invalid field name:{}:{}", loc.line, loc.column)
            }
            ParseError::InvalidFieldId(loc) => {
                write!(f, "Invalid field ID:{}:{}", loc.line, loc.column)
            }
        }
    }
}

impl std::error::Error for ParseError {}
