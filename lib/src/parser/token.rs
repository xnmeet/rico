use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

use super::Comment;

impl<'a> Parser<'a> {
    pub(crate) fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.token() {
            Some(token) if token == &expected => Ok(()),
            Some(_) | None => Err(ParseError::UnexpectedToken(self.start_pos())),
        }
    }

    pub(crate) fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        self.advance();
        self.expect_token(token)
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
                        if token == &Token::LineComment || token == &Token::BlockComment {
                            self.parser_comments();
                        }
                        self.advance();
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
