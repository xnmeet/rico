use crate::ast::{Span, LOC};
use crate::lexer::Token;
use crate::parser::Parser;

pub(crate) struct LocationTracker {
    start_pos: Span,
}

impl LocationTracker {
    pub fn new(start_pos: Span) -> Self {
        Self { start_pos }
    }

    pub fn to_parent_loc(&self, end_loc: &LOC) -> LOC {
        LOC {
            start: self.start_pos,
            end: end_loc.end,
        }
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn start_pos(&self) -> Span {
        let mut line = 0;
        let mut column = 0;
        let mut index = 0;
        if let Some(token) = &self.cur_token {
            line = token.extras.0 + 1;
            column = token.span.start + 1 - token.extras.1;
            index = token.span.start
        }

        Span {
            line,
            column,
            index,
        }
    }

    pub(crate) fn end_pos(&self) -> Span {
        let (extras_0, extras_1) = self.cur_token.as_ref().map_or((0, 0), |t| t.extras);
        let span_end = self.cur_token.as_ref().map_or(0, |t| t.span.end);

        if let Some(token) = self.token() {
            if token == &Token::StringLiteral || token == &Token::BlockComment {
                let split_vec: Vec<&str> = self.text().split('\n').collect();
                if split_vec.len() > 1 {
                    return Span {
                        line: extras_0 + split_vec.len(),
                        column: split_vec.last().unwrap().len() + 1,
                        index: span_end,
                    };
                }
            }
        }

        let line = extras_0 + 1;
        let column = span_end + 1 - extras_1;
        Span {
            line,
            column,
            index: span_end,
        }
    }

    pub(crate) fn get_token_loc(&self) -> LOC {
        LOC {
            start: self.start_pos(),
            end: self.end_pos(),
        }
    }

    pub(crate) fn get_token_parent_loc(&self, start: Span, end: Span) -> LOC {
        LOC { start, end }
    }
}
