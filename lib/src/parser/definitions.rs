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
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let members = self.parse_members(|parser| {
            let member_comments = parser.take_pending_comments();

            parser.expect_token(Token::Identifier)?;
            let member_name = create_identifier(parser.get_token_loc(), parser.text().to_owned());
            let member_start_loc = member_name.loc;

            let mut initializer = None;
            if let Some(Token::Equals) = parser.peek() {
                parser.consume(Token::Equals)?;
                parser.advance();
                let value_token = parser.token().unwrap();

                if matches!(value_token, Token::IntegerLiteral | Token::HexLiteral) {
                    let value = Common::new(
                        NodeType::from_token(value_token).unwrap(),
                        parser.text().to_owned(),
                        parser.get_token_loc(),
                    );

                    initializer = Some(create_initializer(
                        parser.get_token_loc(),
                        value,
                        NodeType::IntConstant,
                    ));
                } else {
                    return Err(ParseError::InvalidValueDeclaration(parser.start_pos()));
                }
            }

            Ok(create_enum_member(
                parser.get_token_parent_loc(member_start_loc.start, parser.end_pos()),
                member_name,
                initializer,
                member_comments,
            ))
        })?;

        Ok(Enum {
            kind: NodeType::EnumDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            members,
            comments,
        })
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let field_comments = self.take_pending_comments();
        // Parse field ID
        self.expect_token(Token::FieldId)?;
        let field_id = create_field_id(self.get_token_loc(), self.text().to_owned());

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

        // Parse default value if present
        let default_value = if let Some(Token::Equals) = self.peek() {
            self.consume(Token::Equals)?;
            Some(self.parse_field_value()?)
        } else {
            None
        };

        // Parse annotations if present
        let field_annotations = self.parse_annotations()?;

        Ok(Field {
            kind: NodeType::FieldDefinition,
            loc: self.get_token_parent_loc(field_id.loc.start, self.end_pos()),
            field_id,
            name: field_name,
            field_type,
            required_type,
            default_value,
            annotations: field_annotations,
            comments: field_comments,
        })
    }

    pub(crate) fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let members = self.parse_members(|parser| parser.parse_field())?;

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

    pub(crate) fn parse_service(&mut self) -> Result<Service, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let members = self.parse_members(|parser| {
            let function_comments = parser.take_pending_comments();

            // Parse return type
            let return_type = match parser.token() {
                Some(Token::Void) => create_void(parser.get_token_loc(), parser.text().to_owned()),
                Some(Token::Identifier) => {
                    create_identifier(parser.get_token_loc(), parser.text().to_owned())
                }
                Some(_) | None => return Err(ParseError::InvalidReturnType(parser.start_pos())),
            };

            // Parse function name
            parser.consume(Token::Identifier)?;
            let function_name = create_identifier(parser.get_token_loc(), parser.text().to_owned());
            let function_start_loc = function_name.loc;

            // Parse parameters
            let params = parser.parse_parameters(|p| p.parse_field())?;
            // skip trivia like comma, semicolon, line comment, block comment
            parser.skip_trivia();
            // Parse function annotations
            let function_annotations = parser.parse_annotations()?;

            Ok(Function {
                kind: NodeType::FunctionDefinition,
                loc: parser.get_token_parent_loc(function_start_loc.start, parser.end_pos()),
                name: function_name,
                return_type,
                params,
                annotations: function_annotations,
                comments: function_comments,
            })
        })?;

        Ok(Service {
            kind: NodeType::ServiceDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            members,
            comments,
        })
    }

    pub(crate) fn parser_comments(&mut self) {
        if let Some(token) = self.token() {
            if token == &Token::LineComment || token == &Token::BlockComment {
                let comment = Comment {
                    kind: NodeType::from_token(token).unwrap(),
                    value: self.text().to_string(),
                    loc: self.get_token_loc(),
                };
                self.pending_comments.push(comment);
            }
        }
    }

    fn parse_members<T, F>(&mut self, mut parse_member: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        let mut members = Vec::new();

        self.consume(Token::LeftBrace)?;

        loop {
            self.advance();
            self.skip_separator();
            self.skip_comments();

            if let Some(Token::RightBrace) = self.token() {
                break;
            }

            members.push(parse_member(self)?);
        }

        Ok(members)
    }

    fn parse_parameters<T, F>(&mut self, mut parse_param: F) -> Result<Vec<T>, ParseError>
    where
        F: FnMut(&mut Self) -> Result<T, ParseError>,
    {
        let mut params = Vec::new();
        self.consume(Token::LeftParen)?;

        loop {
            self.advance();
            if let Some(Token::RightParen) = self.token() {
                break;
            }

            params.push(parse_param(self)?);

            if let Some(Token::Comma) = self.peek() {
                self.advance();
            }
        }

        Ok(params)
    }
}
