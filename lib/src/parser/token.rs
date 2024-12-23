use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

use super::{Comment, NodeType};

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
                    let comment = Comment {
                        kind: NodeType::from_token(token).unwrap(),
                        value: self.text().to_string(),
                        loc: self.get_token_loc(),
                    };
                    self.pending_comments.push(comment);
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }

    pub(crate) fn take_pending_comments(&mut self) -> Vec<Comment> {
        std::mem::take(&mut self.pending_comments)
    }

    pub(crate) fn clear_pending_comments(&mut self) {
        self.pending_comments.clear();
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
}
