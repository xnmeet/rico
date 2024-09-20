use std::fmt;

use super::Span;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Span),
    UnexpectedEOF(Span),
    InvalidInteger(Span),
    InvalidFloat(Span),
    LexerError(Span),
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
        }
    }
}

impl std::error::Error for ParseError {}
