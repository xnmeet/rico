use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

use super::error::ParseErrorKind;
use super::Comment;

impl<'a> Parser<'a> {
    pub(crate) fn with_error_boundary<T>(
        &mut self,
        result: Result<T, ParseError>,
        error_kind: ParseErrorKind,
    ) -> Result<T, ParseError> {
        result.map_err(|_| self.error(error_kind))
    }

    pub(crate) fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.token() {
            Some(token) if token == &expected => Ok(()),
            Some(_) | None => Err(self.error(ParseErrorKind::UnexpectedToken)),
        }
    }
    pub(crate) fn expect_token_with_error(
        &mut self,
        expected: Token,
        error_kind: ParseErrorKind,
    ) -> Result<(), ParseError> {
        let result = self.expect_token(expected);
        self.with_error_boundary(result, error_kind)
    }

    pub(crate) fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        self.advance();
        self.expect_token(token)
    }

    pub(crate) fn consume_with_error(
        &mut self,
        token: Token,
        error_kind: ParseErrorKind,
    ) -> Result<(), ParseError> {
        let result = self.consume(token);
        self.with_error_boundary(result, error_kind)
    }

    pub(crate) fn take_pending_comments(&mut self) -> Vec<Comment> {
        std::mem::take(&mut self.pending_comments)
    }

    pub(crate) fn clear_pending_comments(&mut self) {
        self.pending_comments.clear();
    }

    pub(crate) fn skip_comments(&mut self) {
        loop {
            if let Some(token) = self.token() {
                if token == &Token::LineComment || token == &Token::BlockComment {
                    self.parser_comments();
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    pub(crate) fn skip_separator(&mut self) {
        loop {
            if let Some(token) = self.token() {
                if token == &Token::Comma || token == &Token::Semicolon {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    pub(crate) fn skip_trivia(&mut self) {
        loop {
            if let Some(token) = self.peek() {
                match token {
                    Token::Comma | Token::Semicolon | Token::LineComment | Token::BlockComment => {
                        self.advance();
                        self.parser_comments();
                        continue;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }
}
