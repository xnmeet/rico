use std::fmt;

#[derive(Debug)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]

pub enum ParseError {
    UnexpectedToken(Loc),
    UnexpectedEOF(Loc),
    InvalidInteger(Loc),
    InvalidFloat(Loc),
    LexerError(Loc),
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
