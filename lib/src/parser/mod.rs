mod error;
mod factory;
mod location_ops;
mod parse_definitions;
mod parse_types;
mod parse_values;
mod token_ops;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use logos::Logos;

#[derive(Debug, Clone)]
pub struct ParserToken<'a> {
    pub text: &'a str,
    pub span: logos::Span,
    pub token: Token,
    pub extras: (usize, usize),
}

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    cur_token: Option<ParserToken<'a>>,
    next_token: Option<ParserToken<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        Parser {
            lexer,
            next_token: None,
            cur_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut members = Vec::new();

        loop {
            self.advance();
            self.skip_comments();
            if let Some(token) = self.token() {
                match token {
                    Token::Include => {
                        members.push(DocumentMembers::Include(self.parse_include()?));
                    }
                    Token::Namespace => {
                        members.push(DocumentMembers::Namespace(self.parse_namespace()?));
                    }
                    Token::Const => members.push(DocumentMembers::Const(self.parse_const()?)),
                    Token::Typedef => members.push(DocumentMembers::Typedef(self.parse_typedef()?)),
                    Token::Enum => members.push(DocumentMembers::Enum(self.parse_enum()?)),
                    _ => {
                        return Err(ParseError::UnexpectedToken(self.start_pos()));
                    }
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
        self.bump_block_token_loc();

        // 如果没有下一个 token 且当前 token 存在，清空当前 token
        if self.next_token.is_none() && self.cur_token.is_some() {
            self.cur_token = None;
            return None;
        }

        // 更新当前 token
        if let Some(ref mut token) = self.next_token {
            if let Some(ref cur) = self.cur_token {
                token.extras.0 = cur.extras.0.max(token.extras.0);
            }
            self.cur_token = self.next_token.take(); // Move next_token to cur_token
        }

        // 获取当前 token
        if self.cur_token.is_none() {
            if let Some(Ok(token)) = self.lexer.next() {
                self.cur_token = Some(ParserToken {
                    text: self.lexer.slice(),
                    span: self.lexer.span(),
                    token,
                    extras: self.lexer.extras,
                });
            }
        }

        // 获取下一个 token
        self.next_token = self
            .lexer
            .next()
            .map(|result| {
                result.map(|token| ParserToken {
                    text: self.lexer.slice(),
                    span: self.lexer.span(),
                    token,
                    extras: self.lexer.extras,
                })
            })
            .transpose()
            .unwrap_or(None);

        self.cur_token.as_ref().map(|token| &token.token)
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(token) = &self.next_token {
            return Some(&token.token);
        }

        return None;
    }

    fn token(&self) -> Option<&Token> {
        if let Some(token) = &self.cur_token {
            return Some(&token.token);
        }

        return None;
    }

    fn text(&self) -> &str {
        if let Some(token) = &self.cur_token {
            return token.text;
        }
        return "";
    }

    fn bump_block_token_loc(&mut self) {
        if let Some(token) = self.token() {
            if token == &Token::StringLiteral || token == &Token::BlockComment {
                let count = self.text().matches('\n').count();
                if count > 0 {
                    if let Some(ref mut token) = self.cur_token {
                        token.extras.0 += count;
                        token.extras.1 = self.lexer.span().end;
                    }
                }
            }
        }
    }
}
