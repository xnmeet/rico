use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::factory::*;
use crate::parser::location::LocationTracker;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_include(&mut self) -> Result<Include, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::StringLiteral)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_string());
        let end_loc = name.loc;

        Ok(Include {
            name,
            kind: NodeType::IncludeDefinition,
            loc: tracker.to_parent_loc(&end_loc),
            comments,
        })
    }

    pub(crate) fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let scope = create_identifier(self.get_token_loc(), self.text().to_owned());

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_string());
        let end_loc = name.loc;

        Ok(Namespace {
            kind: NodeType::NamespaceDefinition,
            name,
            scope,
            loc: tracker.to_parent_loc(&end_loc),
            comments,
        })
    }

    pub(crate) fn parse_const(&mut self) -> Result<Const, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        self.consume(Token::Equals)?;
        let const_value = self.parse_field_value()?;

        Ok(Const {
            kind: NodeType::ConstDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            value: const_value,
            field_type,
            comments,
        })
    }

    pub(crate) fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();
        let field_type = self.parse_field_type()?;

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_string());

        Ok(Typedef {
            kind: NodeType::TypedefDefinition,
            loc: tracker.to_parent_loc(&name.loc),
            name,
            field_type,
            comments,
        })
    }

    pub(crate) fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let mut members = Vec::new();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let comments = self.take_pending_comments();

        self.consume(Token::LeftBrace)?;
        loop {
            self.advance();
            self.skip_comma();
            self.skip_comments();

            if let Some(Token::RightBrace) = self.token() {
                break;
            }

            let member_comments = self.take_pending_comments();

            self.expect_token(Token::Identifier)?;
            let member_name = create_identifier(self.get_token_loc(), self.text().to_owned());
            let member_start_loc = member_name.loc;

            let mut initializer = None;
            if let Some(Token::Equals) = self.peek() {
                self.consume(Token::Equals)?;
                self.advance();
                let value_token = self.token().unwrap();

                if matches!(value_token, Token::IntegerLiteral | Token::HexLiteral) {
                    let value = Common::new(
                        NodeType::from_token(value_token).unwrap(),
                        self.text().to_owned(),
                        self.get_token_loc(),
                    );

                    initializer = Some(create_initializer(
                        self.get_token_loc(),
                        value,
                        NodeType::IntConstant,
                    ));
                } else {
                    return Err(ParseError::InvalidValueDeclaration(self.start_pos()));
                }
            }

            members.push(create_enum_member(
                self.get_token_parent_loc(member_start_loc.start, self.end_pos()),
                member_name,
                initializer,
                member_comments,
            ));
        }

        Ok(Enum {
            kind: NodeType::EnumDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            members,
            comments,
        })
    }

    pub(crate) fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let mut members = Vec::new();

        self.consume(Token::LeftBrace)?;

        loop {
            self.advance();
            self.skip_comma();
            self.skip_comments();

            if let Some(Token::RightBrace) = self.token() {
                break;
            }

            let field_comments = self.take_pending_comments();

            // Parse field ID
            self.expect_token(Token::FieldId)?;
            let field_id = create_identifier(self.get_token_loc(), self.text().to_owned());

            // Parse required/optional
            let required_type = match self.peek() {
                Some(Token::Required) => {
                    self.advance();
                    self.text()
                }
                Some(Token::Optional) => {
                    self.advance();
                    self.text()
                }
                _ => "default",
            }
            .to_string();

            // Parse field type
            let field_type = self.parse_field_type()?;

            // Parse field name
            self.consume(Token::Identifier)?;
            let field_name = create_identifier(self.get_token_loc(), self.text().to_owned());

            // Parse annotations if present
            let field_annotations = self.parse_annotations()?;

            members.push(Field {
                kind: NodeType::FieldDefinition,
                loc: self.get_token_parent_loc(field_id.loc.start, self.end_pos()),
                field_id,
                name: field_name,
                field_type,
                required_type,
                default_value: None,
                annotations: field_annotations,
                comments: field_comments,
            });
        }

        Ok(Struct {
            kind: NodeType::StructDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            members,
            comments,
        })
    }

    pub(crate) fn parse_annotations(&mut self) -> Result<Annotations, ParseError> {
        let mut annotations = Vec::new();

        if let Some(Token::LeftParen) = self.peek() {
            let tracker = LocationTracker::new(self.start_pos());
            self.consume(Token::LeftParen)?;

            loop {
                self.advance();
                if let Some(Token::RightParen) = self.token() {
                    break;
                }

                self.expect_token(Token::Identifier)?;
                let annotation_name =
                    create_identifier(self.get_token_loc(), self.text().to_owned());

                self.consume(Token::Equals)?;
                self.consume(Token::StringLiteral)?;
                let value_loc = self.get_token_loc();
                let value = Common {
                    kind: NodeType::StringLiteral,
                    value: self.text().to_owned(),
                    loc: value_loc,
                };

                annotations.push(Annotation {
                    kind: NodeType::Annotation,
                    loc: self.get_token_parent_loc(annotation_name.loc.start, value_loc.end),
                    name: annotation_name,
                    value,
                });

                if let Some(Token::Comma) = self.peek() {
                    self.advance();
                }
            }

            Ok(Annotations {
                kind: NodeType::Annotations,
                loc: tracker.to_parent_loc(&self.get_token_loc()),
                members: annotations,
            })
        } else {
            Ok(Annotations {
                kind: NodeType::Annotations,
                loc: self.get_token_loc(),
                members: annotations,
            })
        }
    }
}
