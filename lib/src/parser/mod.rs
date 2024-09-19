mod error;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use logos::Logos;

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input);
        Parser {
            lexer,
            current_token: None,
            next_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut members = Vec::new();

        self.advance();
        loop {
            self.skip_comments();
            self.skip_newline();
            self.skip_whitespace();
            if let Some(token) = &self.current_token {
                match token {
                    Token::Include => {
                        members.push(DocumentMembers::Include(self.parse_include()?));
                    }
                    // Token::Namespace => {
                    //     members.push(DocumentMembers::Namespace(self.parse_namespace()?));
                    // }
                    _ => return Err(ParseError::UnexpectedToken(self.current_loc())),
                }
                self.advance();
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
        self.try_update_prev_string_pos();
        self.current_token = self.next_token.to_owned();

        if let None = &self.next_token {
            let current_token = self.lexer.next();
            if let Some(Ok(token)) = current_token {
                self.current_token = Some(token);
            }
        }
        println!("{:?}", self.current_token);

        return self.current_token.as_ref();
    }

    fn peek(&mut self) -> Option<&Token> {
        if let Some(_) = &self.next_token {
            return self.next_token.as_ref();
        }

        let next_token = self.lexer.next();
        if let Some(Ok(token)) = next_token {
            self.next_token = Some(token);
        } else {
            self.next_token = None;
        }
        return self.next_token.as_ref();
    }

    fn current_loc(&mut self) -> (usize, usize) {
        let line = self.lexer.extras.0;
        let column = self.lexer.span().start - self.lexer.extras.1;

        if let Some(token) = self.peek() {
            if let token = &Token::Whitespace {
                return (
                    self.lexer.extras.0,
                    self.lexer.span().start - self.lexer.extras.1,
                );
            }
        }

        (line, column)
    }

    fn start_loc(&self) -> Span {
        let line = self.lexer.extras.0;
        let column = self.lexer.span().start - self.lexer.extras.1;
        Span {
            column,
            line,
            index: self.lexer.span().end,
        }
    }

    fn try_update_prev_string_pos(&mut self) {
        if let Some(token) = &self.current_token {
            if token == &Token::String {
                // 处理换行字符串
                let count = self.lexer.slice().matches('\n').count();
                self.lexer.extras.0 += count;
                self.lexer.extras.1 = self.lexer.span().end;
            }
        }
    }

    fn parse_include(&mut self) -> Result<Include, ParseError> {
        let start_span = self.lexer.span();
        let (start_line, start_column) = self.current_loc();
        self.advance();
        self.skip_whitespace();
        self.expect_token(Token::StringLiteral)?;
        let (value_line, value_column) = self.current_loc();
        let value = self.lexer.slice();

        Ok(Include {
            name: Common {
                kind: NodeType::IncludeKeyword,
                value: value.to_string(),
                loc: LOC {
                    start: Span {
                        column: value_column,
                        line: value_line,
                        index: start_span.start,
                    },
                    end: Span {
                        column: value_column + value.len(),
                        line: value_line,
                        index: start_span.end,
                    },
                },
            },
            kind: NodeType::IncludeDefinition,
            loc: LOC {
                start: Span {
                    column: start_column,
                    line: start_line,
                    index: start_span.start,
                },
                end: Span {
                    column: value_column + value.len(),
                    line: value_line,
                    index: self.lexer.span().end,
                },
            },
        })
    }

    // fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
    //     let start_span = self.lexer.span();

    //     self.advance();
    //     self.expect_token(Token::Identifier)?;
    //     // for example: namespace go a.b.c
    //     let indent_scope = self.lexer.slice(); // result go

    //     self.advance();
    //     self.expect_token(Token::ChainIdentifier)?;
    //     let space = self.lexer.slice(); // result a.b.c
    //     let combined = format!("{}{}", indent_scope, space);

    //     Ok(Namespace {
    //         kind: NodeType::NamespaceDefinition,
    //         name: Common {
    //             kind: NodeType::Identifier,
    //             value: space.to_string(),
    //             loc: LOC {
    //                 start: Span {
    //                     column: value_column,
    //                     line: value_line,
    //                     index: start_span.start,
    //                 },
    //                 end: Span {
    //                     column: value_column + value.len(),
    //                     line: value_line,
    //                     index: start_span.end,
    //                 },
    //             },
    //         },
    //         loc: LOC {
    //             start: Span {
    //                 index: start_span.start,
    //             },
    //             end: Span {
    //                 index: self.lexer.span().end,
    //             },
    //         },
    //         scope: Common {
    //             kind: NodeType::Identifier,
    //             value: indent_scope.to_string(),
    //             loc: LOC {
    //                 start: Span {
    //                     column: value_column,
    //                     line: value_line,
    //                     index: start_span.start,
    //                 },
    //                 end: Span {
    //                     column: value_column + value.len(),
    //                     line: value_line,
    //                     index: start_span.end,
    //                 },
    //             },
    //         },
    //     })
    // }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match &self.current_token {
            Some(token) if token == &expected => Ok(()),
            Some(_) => Err(ParseError::UnexpectedToken(self.current_loc())),
            None => Err(ParseError::UnexpectedEOF(self.current_loc())),
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

    fn skip_newline(&mut self) {
        loop {
            if let Some(token) = &self.current_token {
                if token == &Token::Newline {
                    self.advance();
                }
            }
            break;
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(token) = &self.current_token {
                if token == &Token::Whitespace {
                    self.advance();
                }
            }
            break;
        }
    }
}
