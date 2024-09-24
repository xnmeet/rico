mod error;
mod factory;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use factory::*;
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
                    Token::Const => members.push(DocumentMembers::Const(self.parse_const()?)),
                    Token::Typedef => members.push(DocumentMembers::Typedef(self.parse_typedef()?)),
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

        let current_token = self.lexer.next();
        if let Some(Ok(token)) = current_token {
            self.current_token = Some(token);
        } else {
            self.current_token = None;
        }
        return self.current_token.as_ref();
    }

    fn bump_block_token_loc(&mut self) {
        if let Some(token) = &self.current_token {
            if token == &Token::StringLiteral || token == &Token::BlockComment {
                let count = self.lexer.slice().matches('\n').count();
                if count > 0 {
                    self.lexer.extras.0 += count;
                    self.lexer.extras.1 = self.lexer.span().end;
                }
            }
        }
    }

    fn start_pos(&self) -> Span {
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

    fn end_pos(&self) -> Span {
        if let Some(token) = &self.current_token {
            if token == &Token::StringLiteral || token == &Token::BlockComment {
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

    fn get_token_loc(&self) -> LOC {
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

        self.consume(Token::StringLiteral)?;
        let value_loc = self.get_token_loc();
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

        self.consume(Token::Identifier)?;
        let scope_loc = self.get_token_loc();

        // for example: namespace go a.b.c
        let indent_scope = self.lexer.slice(); // result go

        self.consume(Token::Identifier)?;
        let value_loc = self.get_token_loc();

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

    fn parse_const(&mut self) -> Result<Const, ParseError> {
        let const_start_pos = self.start_pos();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name_loc = self.get_token_loc();
        let name = self.lexer.slice();

        self.consume(Token::Equals)?;

        let const_value = self.parse_field_value()?;

        Ok(Const {
            kind: NodeType::ConstDefinition,
            loc: self.get_token_parent_loc(const_start_pos, self.get_token_loc().end),
            name: Common {
                loc: name_loc,
                value: name.to_string(),
                kind: NodeType::Identifier,
            },
            value: const_value,
            field_type,
        })
    }

    fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        let typedef_start_pos = self.start_pos();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name_loc = self.get_token_loc();
        let name = self.lexer.slice();

        Ok(Typedef {
            kind: NodeType::TypedefDefinition,
            loc: self.get_token_parent_loc(typedef_start_pos, self.get_token_loc().end),
            name: Common {
                loc: name_loc,
                value: name.to_string(),
                kind: NodeType::Identifier,
            },
            field_type,
        })
    }

    fn is_list_token_end(&mut self) -> bool {
        self.expect_token(Token::RightBracket).is_ok()
    }

    fn is_map_token_end(&mut self) -> bool {
        self.expect_token(Token::RightBrace).is_ok()
    }

    fn parse_complex_type<F>(&mut self, create_field_type: F) -> Result<FieldType, ParseError>
    where
        F: Fn(LOC, &str, FieldType) -> FieldType,
    {
        let start_loc = self.start_pos();
        let slice = self.lexer.slice();

        self.consume(Token::LeftAngle)?;
        let filed_type = self.parse_field_type()?;

        self.consume(Token::RightAngle)?;
        let end_loc = self.get_token_loc();
        Ok(create_field_type(
            LOC {
                start: start_loc,
                end: end_loc.end,
            },
            slice,
            filed_type,
        ))
    }

    fn parse_list_type(&mut self) -> Result<FieldType, ParseError> {
        self.parse_complex_type(create_list_field_type)
    }

    fn parse_set_type(&mut self) -> Result<FieldType, ParseError> {
        self.parse_complex_type(create_set_field_type)
    }

    fn parse_map_type(&mut self) -> Result<FieldType, ParseError> {
        let start_loc = self.start_pos();
        let slice = self.lexer.slice();

        self.consume(Token::LeftAngle)?;
        let filed_key_type = self.parse_field_type()?;

        self.consume(Token::Comma)?;
        let filed_value_type = self.parse_field_type()?;

        self.consume(Token::RightAngle)?;

        let end_loc = self.get_token_loc();
        Ok(create_map_field_type(
            self.get_token_parent_loc(start_loc, end_loc.end),
            slice,
            filed_key_type,
            filed_value_type,
        ))
    }

    fn parse_field_type(&mut self) -> Result<FieldType, ParseError> {
        self.advance();
        match &self.current_token {
            Some(token) => match token {
                Token::Binary
                | Token::String
                | Token::Byte
                | Token::I16
                | Token::I32
                | Token::I64
                | Token::Double
                | Token::Bool => Ok(create_keyword_field_type(
                    token,
                    self.get_token_loc(),
                    self.lexer.slice(),
                )),
                Token::Identifier => Ok(create_identifier_field_type(
                    self.get_token_loc(),
                    self.lexer.slice(),
                )),
                Token::List => self.parse_list_type(),
                Token::Map => self.parse_map_type(),
                Token::Set => self.parse_set_type(),
                _ => Err(ParseError::UnsupportedType(self.start_pos())),
            },
            None => Err(ParseError::MissingTypeDeclaration(self.start_pos())),
        }
    }

    fn parse_list_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        let start_loc = self.get_token_loc();
        let mut elements: Vec<FieldInitialValue> = Vec::new();

        loop {
            let value = self.parse_field_value();

            if let Ok(val) = value {
                elements.push(val);
            } else if self.is_list_token_end() {
                break;
            } else {
                return value;
            }

            self.advance();
            if self.is_list_token_end() {
                break;
            }
            self.expect_token(Token::Comma)?;
        }

        Ok(create_const_list_value(
            self.get_token_parent_loc(start_loc.start, self.end_pos()),
            elements,
        ))
    }

    fn parse_map_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        let start_pos = self.start_pos();
        let mut properties: Vec<MapProperty> = Vec::new();

        loop {
            let property_start_pos = self.start_pos();
            let property_key = self.parse_field_value()?;

            self.consume(Token::Colon)?;
            let property_value = self.parse_field_value();

            if let Ok(value) = property_value {
                properties.push(MapProperty {
                    kind: NodeType::PropertyAssignment,
                    loc: self.get_token_parent_loc(property_start_pos, self.get_token_loc().end),
                    name: property_key,
                    value,
                });
            } else if self.is_map_token_end() {
                break;
            } else {
                return property_value;
            }

            self.advance();
            if self.is_map_token_end() {
                break;
            }
            self.expect_token(Token::Comma)?;
        }

        Ok(create_map_value(
            self.get_token_parent_loc(start_pos, self.end_pos()),
            properties,
        ))
    }

    fn parse_field_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        self.advance();
        match &self.current_token {
            Some(token) => match token {
                Token::StringLiteral
                | Token::IntegerLiteral
                | Token::DoubleLiteral
                | Token::BooleanLiteral => Ok(create_const_value(
                    token,
                    self.get_token_loc(),
                    self.lexer.slice(),
                )),
                Token::Identifier => Ok(create_identifier_value(
                    self.get_token_loc(),
                    self.lexer.slice(),
                )),
                Token::LeftBracket => self.parse_list_value(),
                Token::LeftBrace => self.parse_map_value(),
                _ => Err(ParseError::InvalidValueDeclaration(self.start_pos())),
            },
            None => Err(ParseError::UnexpectedEOF(self.start_pos())),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match &self.current_token {
            Some(token) if token == &expected => Ok(()),
            Some(_) => Err(ParseError::UnexpectedToken(self.start_pos())),
            None => Err(ParseError::UnexpectedEOF(self.start_pos())),
        }
    }

    fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        self.advance();
        self.expect_token(token)
    }

    fn skip_comments(&mut self) {
        loop {
            if let Some(token) = &self.current_token {
                if token == &Token::LineComment || token == &Token::BlockComment {
                    self.advance();
                    continue;
                }
            }
            break;
        }
    }
}
