use crate::ast::{FieldInitialValue, MapProperty, NodeType};
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::factory::*;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    fn parse_delimited_values<T>(
        &mut self,
        is_end: fn(&mut Self) -> bool,
        parse_element: impl Fn(&mut Self) -> Result<T, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        let mut elements = Vec::new();

        loop {
            self.advance();
            self.skip_comments();

            if is_end(self) {
                break;
            }

            let element = parse_element(self)?;
            elements.push(element);

            if let Some(Token::Comma) = self.peek() {
                self.consume(Token::Comma)?;
            }
        }

        Ok(elements)
    }

    pub(crate) fn parse_list_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        let start_loc = self.get_token_loc();

        let elements = self.parse_delimited_values(Self::is_bracket_end, |parser| {
            parser.parse_field_value_opt(false)
        })?;

        Ok(create_const_list_value(
            self.get_token_parent_loc(start_loc.start, self.end_pos()),
            elements,
        ))
    }

    pub(crate) fn parse_map_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        let start_pos = self.start_pos();

        let properties = self.parse_delimited_values(Self::is_brace_end, |parser| {
            let property_start_pos = parser.start_pos();
            let property_key = parser.parse_field_value_opt(false)?;

            parser.consume(Token::Colon)?;
            let property_value = parser.parse_field_value()?;

            Ok(MapProperty {
                kind: NodeType::PropertyAssignment,
                loc: parser.get_token_parent_loc(property_start_pos, parser.get_token_loc().end),
                name: property_key,
                value: property_value,
            })
        })?;

        Ok(create_map_value(
            self.get_token_parent_loc(start_pos, self.end_pos()),
            properties,
        ))
    }

    pub(crate) fn parse_field_value(&mut self) -> Result<FieldInitialValue, ParseError> {
        self.parse_field_value_opt(true)
    }

    pub(crate) fn parse_field_value_opt(
        &mut self,
        auto_advance: bool,
    ) -> Result<FieldInitialValue, ParseError> {
        if auto_advance {
            self.advance();
        }

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
