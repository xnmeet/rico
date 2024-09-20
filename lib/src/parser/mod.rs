mod error;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use logos::Logos;

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        Parser {
            lexer,
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut members = Vec::new();

        loop {
            self.advance();
            self.skip_comments();
            if let Some(token) = &self.current_token {
                match token {
                    Token::Include => {
                        members.push(DocumentMembers::Include(self.parse_include()?));
                    }
                    Token::Namespace => {
                        members.push(DocumentMembers::Namespace(self.parse_namespace()?));
                    }
                    _ => return Err(ParseError::UnexpectedToken(self.start_pos())),
                }
            } else {
                break;
            }
        }

        Ok(Document {
            kind: NodeType::ThriftDocument,
            members,
        })
    }

    fn advance(&mut self) -> Option<&Token> {
        // 处理换行字符串
        self.bump_block_string_loc();

        let current_token = self.lexer.next();
        if let Some(Ok(token)) = current_token {
            self.current_token = Some(token);
        } else {
            self.current_token = None;
        }
        return self.current_token.as_ref();
    }

    fn bump_block_string_loc(&mut self) {
        if let Some(token) = &self.current_token {
            if token == &Token::StringLiteral {
                let count = self.lexer.slice().matches('\n').count();
                if count > 0 {
                    self.lexer.extras.0 += count;
                    self.lexer.extras.1 = self.lexer.span().end;
                }
            }
        }
    }

    fn start_pos(&mut self) -> Span {
        let line = self.lexer.extras.0 + 1;
        // The addition of 1 here is on the one hand to make the column start at 1
        // and on the other hand to avoid newline value overflow
        let column = self.lexer.span().start + 1 - self.lexer.extras.1;

        Span {
            line,
            column,
            index: self.lexer.span().start,
        }
    }

    fn end_pos(&mut self) -> Span {
        if let Some(token) = &self.current_token {
            if token == &Token::StringLiteral {
                // Handling newline strings
                let split_vec: Vec<&str> = self.lexer.slice().split('\n').collect();
                if split_vec.len() > 1 {
                    return Span {
                        line: self.lexer.extras.0 + split_vec.len(),
                        column: split_vec.last().unwrap().len() + 1,
                        index: self.lexer.span().end,
                    };
                }
            }
        }

        let line = self.lexer.extras.0 + 1;
        let column = self.lexer.span().end + 1 - self.lexer.extras.1;
        Span {
            line,
            column,
            index: self.lexer.span().end,
        }
    }

    fn get_token_loc(&mut self) -> LOC {
        LOC {
            start: self.start_pos(),
            end: self.end_pos(),
        }
    }

    fn get_token_parent_loc(&self, start: Span, end: Span) -> LOC {
        LOC { start, end }
    }

    fn parse_include(&mut self) -> Result<Include, ParseError> {
        let include_start_pos = self.start_pos();

        self.advance();
        let value_loc = self.get_token_loc();
        self.expect_token(Token::StringLiteral)?;
        let value = self.lexer.slice();

        Ok(Include {
            name: Common {
                kind: NodeType::Identifier,
                value: value.to_string(),
                loc: value_loc,
            },
            kind: NodeType::IncludeDefinition,
            loc: self.get_token_parent_loc(include_start_pos, value_loc.end),
        })
    }

    fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        let namespace_start_pos = self.start_pos();

        self.advance();
        let scope_loc = self.get_token_loc();
        self.expect_token(Token::Identifier)?;
        // for example: namespace go a.b.c
        let indent_scope = self.lexer.slice(); // result go

        self.advance();
        let value_loc = self.get_token_loc();
        self.expect_token(Token::ChainIdentifier)?;
        let value = self.lexer.slice(); // result a.b.c

        Ok(Namespace {
            kind: NodeType::NamespaceDefinition,
            name: Common {
                kind: NodeType::Identifier,
                value: value.to_string(),
                loc: value_loc,
            },
            scope: Common {
                kind: NodeType::Identifier,
                value: indent_scope.to_string(),
                loc: scope_loc,
            },
            loc: self.get_token_parent_loc(namespace_start_pos, value_loc.end),
        })
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match &self.current_token {
            Some(token) if token == &expected => Ok(()),
            Some(_) => Err(ParseError::UnexpectedToken(self.start_pos())),
            None => Err(ParseError::UnexpectedEOF(self.start_pos())),
        }
    }

    fn skip_comments(&mut self) {
        loop {
            if let Some(token) = &self.current_token {
                if token == &Token::LineComment || token == &Token::BlockComment {
                    self.advance();
                }
            }
            break;
        }
    }
}
