use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_include(&mut self) -> Result<Include, ParseError> {
        let include_start_pos = self.start_pos();

        self.consume(Token::StringLiteral)?;
        let value_loc = self.get_token_loc();
        let value = self.text();

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

    pub(crate) fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        let namespace_start_pos = self.start_pos();

        self.consume(Token::Identifier)?;
        let scope_loc = self.get_token_loc();
        let indent_scope = self.text().to_owned();

        self.consume(Token::Identifier)?;
        let value_loc = self.get_token_loc();
        let value = self.text();

        Ok(Namespace {
            kind: NodeType::NamespaceDefinition,
            name: Common {
                kind: NodeType::Identifier,
                value: value.to_string(),
                loc: value_loc,
            },
            scope: Common {
                kind: NodeType::Identifier,
                value: indent_scope,
                loc: scope_loc,
            },
            loc: self.get_token_parent_loc(namespace_start_pos, value_loc.end),
        })
    }

    pub(crate) fn parse_const(&mut self) -> Result<Const, ParseError> {
        let const_start_pos = self.start_pos();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name_loc = self.get_token_loc();
        let name = self.text().to_owned();

        self.consume(Token::Equals)?;

        let const_value = self.parse_field_value()?;
        Ok(Const {
            kind: NodeType::ConstDefinition,
            loc: self.get_token_parent_loc(const_start_pos, self.get_token_loc().end),
            name: Common {
                loc: name_loc,
                value: name,
                kind: NodeType::Identifier,
            },
            value: const_value,
            field_type,
        })
    }

    pub(crate) fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        let typedef_start_pos = self.start_pos();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name_loc = self.get_token_loc();
        let name = self.text();

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

    pub(crate) fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        let enum_start_pos = self.start_pos();
        let mut members = Vec::new();

        self.consume(Token::Identifier)?;
        let name_loc = self.get_token_loc();
        let name = self.text().to_owned();

        self.consume(Token::LeftBrace)?;
        loop {
            self.advance();
            self.skip_comma();
            self.skip_comments();

            if let Some(Token::RightBrace) = self.token() {
                break;
            }

            self.expect_token(Token::Identifier)?;
            let member_name = self.text().to_owned();
            let member_name_loc = self.get_token_loc();

            let mut initializer = None;

            if let Some(Token::Equals) = self.peek() {
                self.consume(Token::Equals)?;
                self.advance();
                let value_token = self.token().unwrap();

                if matches!(value_token, Token::IntegerLiteral | Token::HexLiteral) {
                    initializer = Some(Initializer {
                        loc: self.get_token_loc(),
                        kind: NodeType::IntConstant,
                        value: Common {
                            loc: self.get_token_loc(),
                            kind: NodeType::from_token(value_token).unwrap(),
                            value: self.text().to_owned(),
                        },
                    });
                } else {
                    return Err(ParseError::InvalidValueDeclaration(self.start_pos()));
                }
            }

            members.push(EnumMember {
                kind: NodeType::EnumMember,
                loc: self.get_token_parent_loc(member_name_loc.start, self.end_pos()),
                name: Common {
                    loc: member_name_loc,
                    kind: NodeType::Identifier,
                    value: member_name,
                },
                initializer,
            });
        }

        Ok(Enum {
            kind: NodeType::EnumDefinition,
            loc: self.get_token_parent_loc(enum_start_pos, self.end_pos()),
            name: Common {
                value: name,
                loc: name_loc,
                kind: NodeType::Identifier,
            },
            members,
        })
    }
}
