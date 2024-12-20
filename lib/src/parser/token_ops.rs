use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.token() {
            Some(token) if token == &expected => Ok(()),
            Some(_) => Err(ParseError::UnexpectedToken(self.start_pos())),
            None => Err(ParseError::UnexpectedEOF(self.start_pos())),
        }
    }

    pub(crate) fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        self.advance();
        self.expect_token(token)
    }

    pub(crate) fn skip_comments(&mut self) {
        loop {
            if let Some(token) = self.token() {
                if token == &Token::LineComment || token == &Token::BlockComment {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    pub(crate) fn skip_comma(&mut self) {
        loop {
            if let Some(token) = self.token() {
                if token == &Token::Comma {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }
}
