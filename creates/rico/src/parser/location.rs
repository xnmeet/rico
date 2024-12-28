use crate::ast::{Span, LOC};
use crate::parser::Parser;

/// Tracks source code locations during parsing
pub(crate) struct LocationTracker {
    /// Starting position of the current node
    start_pos: Span,
}

impl LocationTracker {
    /// Creates a new LocationTracker starting at the given position
    pub fn new(start_pos: Span) -> Self {
        Self { start_pos }
    }

    /// Creates a location that spans from this tracker's start position to the given end location
    pub fn to_parent_loc(&self, end_loc: &LOC) -> LOC {
        LOC {
            start: self.start_pos,
            end: end_loc.end,
        }
    }
}

impl<'a> Parser<'a> {
    /// Gets the current position in the source
    pub(crate) fn start_pos(&self) -> Span {
        if let Some(token) = &self.cur_token {
            return token.start;
        }
        Span::new(1, 1, 0)
    }

    /// Gets the end position of the current token
    pub(crate) fn end_pos(&self) -> Span {
        if let Some(token) = &self.cur_token {
            return token.end;
        }
        Span::new(1, 1, 0)
    }

    /// Gets the location information for the current token
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
