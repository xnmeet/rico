use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken((usize, usize)),
    UnexpectedEOF((usize, usize)),
    InvalidInteger((usize, usize)),
    InvalidFloat((usize, usize)),
    LexerError((usize, usize)),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(loc) => {
                write!(f, "Unexpected token:{}:{}", loc.0, loc.1)
            }
            ParseError::UnexpectedEOF(loc) => {
                write!(f, "Unexpected end of file:{}:{}", loc.0, loc.1)
            }
            ParseError::InvalidInteger(loc) => {
                write!(f, "Invalid integer:{}:{}", loc.0, loc.1)
            }
            ParseError::InvalidFloat(loc) => {
                write!(f, "Invalid float:{}:{}", loc.0, loc.1)
            }
            ParseError::LexerError(loc) => {
                write!(f, "Lexer error:{}:{}", loc.0, loc.1)
            }
        }
    }
}

impl std::error::Error for ParseError {}
