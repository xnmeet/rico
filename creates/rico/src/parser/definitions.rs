use crate::ast::*;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::factory::*;
use crate::parser::location::LocationTracker;
use crate::parser::Parser;

use super::error::ParseErrorKind;

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

        let result = Const {
            kind: NodeType::ConstDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            value: const_value,
            field_type,
            comments,
        };
        self.skip_trivia();
        Ok(result)
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
                    return Err(parser.error(ParseErrorKind::InvalidValueDeclaration));
                }
            }

            let annotations = parser.parse_annotations()?;
            Ok(create_enum_member(
                parser.get_token_parent_loc(member_start_loc.start, parser.end_pos()),
                member_name,
                initializer,
                member_comments,
                annotations,
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

    fn parse_field_name(&mut self) -> Result<Common<String>, ParseError> {
        self.advance();

        const VALID_TOKENS: &[Token] = &[
            Token::Identifier,
            Token::Namespace,
            Token::Include,
            Token::List,
            Token::Map,
            Token::Set,
            Token::Oneway,
            Token::Required,
            Token::Optional,
            Token::Throws,
            Token::Bool,
        ];

        if !VALID_TOKENS.iter().any(|valid| self.token() == Some(valid)) {
            return Err(self.error(ParseErrorKind::InvalidFieldName));
        }

        Ok(create_identifier(
            self.get_token_loc(),
            self.text().to_owned(),
        ))
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let field_comments = self.take_pending_comments();

        // Parse field ID using the new function
        let field_id = self.parse_field_id()?;

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
        let field_name = self.parse_field_name()?;

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

    fn parse_struct_like<T>(
        &mut self,
        kind: NodeType,
        constructor: impl FnOnce(NodeType, LOC, Common<String>, Vec<Field>, Vec<Comment>) -> T,
    ) -> Result<T, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        let members = self.parse_members(|parser| parser.parse_field())?;

        Ok(constructor(
            kind,
            tracker.to_parent_loc(&self.get_token_loc()),
            name,
            members,
            comments,
        ))
    }

    pub(crate) fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        self.parse_struct_like(
            NodeType::StructDefinition,
            |kind, loc, name, members, comments| Struct {
                kind,
                loc,
                name,
                members,
                comments,
            },
        )
    }

    pub(crate) fn parse_union(&mut self) -> Result<Union, ParseError> {
        self.parse_struct_like(
            NodeType::UnionDefinition,
            |kind, loc, name, members, comments| Union {
                kind,
                loc,
                name,
                members,
                comments,
            },
        )
    }

    pub(crate) fn parse_exception(&mut self) -> Result<Exception, ParseError> {
        self.parse_struct_like(
            NodeType::ExceptionDefinition,
            |kind, loc, name, members, comments| Exception {
                kind,
                loc,
                name,
                members,
                comments,
            },
        )
    }

    pub(crate) fn parse_annotations(&mut self) -> Result<Option<Annotations>, ParseError> {
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

            Ok(Some(Annotations {
                kind: NodeType::Annotations,
                loc: tracker.to_parent_loc(&self.get_token_loc()),
                members: annotations,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_extends(&mut self) -> Result<Option<Common<String>>, ParseError> {
        if let Some(Token::Extends) = self.peek() {
            self.advance(); // Consume 'extends'
            self.consume(Token::Identifier)?;
            Ok(Some(create_identifier(
                self.get_token_loc(),
                self.text().to_owned(),
            )))
        } else {
            Ok(None)
        }
    }

    fn parse_return_type(&mut self) -> Result<FieldType, ParseError> {
        match self.token() {
            Some(Token::Void) => Ok(FieldType::CommonType(create_void(
                self.get_token_loc(),
                self.text().to_owned(),
            ))),
            Some(_) => self.parse_field_type_opt(false),
            None => Err(self.error(ParseErrorKind::InvalidReturnType)),
        }
    }

    pub(crate) fn parse_service(&mut self) -> Result<Service, ParseError> {
        let tracker = LocationTracker::new(self.start_pos());
        let comments = self.take_pending_comments();

        self.consume(Token::Identifier)?;
        let name = create_identifier(self.get_token_loc(), self.text().to_owned());

        // Parse extends clause if present
        let extends = self.parse_extends()?;

        let members = self.parse_members(|parser| {
            let function_comments = parser.take_pending_comments();

            // Parse oneway if present
            let oneway = if let Some(Token::Oneway) = parser.token() {
                parser.advance();
                true
            } else {
                false
            };

            // Parse return type
            let return_type = parser.parse_return_type()?;

            parser.consume(Token::Identifier)?;
            let function_name = create_identifier(parser.get_token_loc(), parser.text().to_owned());
            let function_start_loc = function_name.loc;

            // Parse parameters
            let params = parser.parse_parameters(|p| p.parse_field())?;

            // Parse throws if present
            let throws = parser.parse_throws()?;

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
                throws,
                annotations: function_annotations,
                comments: function_comments,
                oneway,
            })
        })?;

        // Parse service annotations
        let annotations = self.parse_annotations()?;

        Ok(Service {
            kind: NodeType::ServiceDefinition,
            loc: tracker.to_parent_loc(&self.get_token_loc()),
            name,
            extends,
            members,
            comments,
            annotations,
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

    fn parse_throws(&mut self) -> Result<Option<Vec<Field>>, ParseError> {
        if let Some(Token::Throws) = self.peek() {
            self.advance(); // 消费 throws 关键字
            let throws = self.parse_parameters(|p| p.parse_field())?;
            Ok(Some(throws))
        } else {
            Ok(None)
        }
    }

    fn parse_field_id(&mut self) -> Result<Common<String>, ParseError> {
        // Get the token text without the colon
        let text = self.text();
        // Validate that it's a valid unsigned integer
        let field_id = match text.parse::<u64>() {
            Ok(_) => Ok(create_field_id(self.get_token_loc(), text.to_string())),
            Err(_) => Err(self.error(ParseErrorKind::InvalidFieldId)),
        };
        if field_id.is_ok() {
            self.consume(Token::Colon)?;
        }
        field_id
    }
}
