use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidInteger(String),
    InvalidFloat(String),
    LexerError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseError::InvalidInteger(s) => write!(f, "Invalid integer: {}", s),
            ParseError::InvalidFloat(s) => write!(f, "Invalid float: {}", s),
            ParseError::LexerError => write!(f, "Lexer error"),
        }
    }
}

impl std::error::Error for ParseError {}
