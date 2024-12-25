use crate::ast::FieldType;
use crate::ast::LOC;
use crate::lexer::Token;
use crate::parser::error::ParseError;
use crate::parser::factory::*;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_complex_type<F>(
        &mut self,
        create_field_type: F,
    ) -> Result<FieldType, ParseError>
    where
        F: Fn(LOC, &str, FieldType) -> FieldType,
    {
        let start_loc = self.start_pos();
        let slice = self.text().to_owned();

        self.consume(Token::LeftAngle)?;
        let filed_type = self.parse_field_type()?;

        self.consume(Token::RightAngle)?;
        let end_loc = self.get_token_loc();
        Ok(create_field_type(
            LOC {
                start: start_loc,
                end: end_loc.end,
            },
            &slice,
            filed_type,
        ))
    }

    pub(crate) fn parse_list_type(&mut self) -> Result<FieldType, ParseError> {
        self.parse_complex_type(create_list_field_type)
    }

    pub(crate) fn parse_set_type(&mut self) -> Result<FieldType, ParseError> {
        self.parse_complex_type(create_set_field_type)
    }

    pub(crate) fn parse_map_type(&mut self) -> Result<FieldType, ParseError> {
        let start_loc = self.start_pos();
        let slice = self.text().to_owned();

        self.consume(Token::LeftAngle)?;
        let filed_key_type = self.parse_field_type()?;

        self.consume(Token::Comma)?;
        let filed_value_type = self.parse_field_type()?;

        self.consume(Token::RightAngle)?;

        let end_loc = self.get_token_loc();
        Ok(create_map_field_type(
            self.get_token_parent_loc(start_loc, end_loc.end),
            &slice,
            filed_key_type,
            filed_value_type,
        ))
    }

    pub(crate) fn parse_field_type(&mut self) -> Result<FieldType, ParseError> {
        self.parse_field_type_opt(true)
    }

    pub(crate) fn parse_field_type_opt(
        &mut self,
        auto_advance: bool,
    ) -> Result<FieldType, ParseError> {
        if auto_advance {
            self.advance();
        }

        match self.token() {
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
                    self.text(),
                )),
                Token::Identifier => Ok(create_identifier_field_type(
                    self.get_token_loc(),
                    self.text(),
                )),
                Token::List => self.parse_list_type(),
                Token::Map => self.parse_map_type(),
                Token::Set => self.parse_set_type(),
                _ => Err(ParseError::UnsupportedType(self.start_pos())),
            },
            None => Err(ParseError::MissingTypeDeclaration(self.start_pos())),
        }
    }
}
