use crate::ast::{FieldInitialValue, MapProperty, NodeType};
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::factory::*;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_list_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        let start_loc = self.get_token_loc();
        let mut elements: Vec<FieldInitialValue> = Vec::new();

        loop {
            let value = self.parse_field_value();

            if let Ok(val) = value {
                elements.push(val);
            } else if self.is_bracket_end() {
                break;
            } else {
                return value;
            }

            self.advance();
            if self.is_bracket_end() {
                break;
            }
            self.expect_token(Token::Comma)?;
        }

        Ok(create_const_list_value(
            self.get_token_parent_loc(start_loc.start, self.end_pos()),
            elements,
        ))
    }

    pub(crate) fn parse_map_value(&mut self) -> Result<FieldInitialValue, ParseError> {
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
            } else if self.is_brace_end() {
                break;
            } else {
                return property_value;
            }

            self.advance();
            if self.is_brace_end() {
                break;
            }
            self.expect_token(Token::Comma)?;
        }

        Ok(create_map_value(
            self.get_token_parent_loc(start_pos, self.end_pos()),
            properties,
        ))
    }

    pub(crate) fn parse_field_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        self.advance();
        match self.token() {
            Some(token) => match token {
                Token::StringLiteral
                | Token::IntegerLiteral
                | Token::DoubleLiteral
                | Token::BooleanLiteral
                | Token::HexLiteral => {
                    Ok(create_const_value(token, self.get_token_loc(), self.text()))
                }
                Token::Identifier => Ok(create_identifier_value(self.get_token_loc(), self.text())),
                Token::LeftBracket => self.parse_list_value(),
                Token::LeftBrace => self.parse_map_value(),
                _ => Err(ParseError::InvalidValueDeclaration(self.start_pos())),
            },
            None => Err(ParseError::UnexpectedEOF(self.start_pos())),
        }
    }

    fn is_bracket_end(&mut self) -> bool {
        self.expect_token(Token::RightBracket).is_ok()
    }

    fn is_brace_end(&mut self) -> bool {
        self.expect_token(Token::RightBrace).is_ok()
    }
}
