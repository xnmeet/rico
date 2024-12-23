use crate::ast::{Span, LOC};
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
        if let Some(token) = &self.cur_token {
            return token.start;
        }

        Span::new(1, 1, 0)
    }

    pub(crate) fn end_pos(&self) -> Span {
        if let Some(token) = &self.cur_token {
            return token.end;
        }
        Span::new(1, 1, 0)
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
