mod error;

use crate::ast::*;
use crate::lexer::Token;
use logos::Logos;
use std::iter::Peekable;
use std::string::ParseError;

pub struct Parser<'a> {
    tokens: Peekable<logos::Lexer<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Token::lexer(input).peekable();
        Parser { tokens: lexer }
    }

    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut headers = Vec::new();
        let mut definitions = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Ok(token) => match token {
                    Token::Namespace | Token::Include => {
                        headers.push(self.parse_header()?);
                    }
                    Token::Const => {
                        definitions.push(Definition::Const(self.parse_const()?));
                    }
                    Token::Typedef => {
                        definitions.push(Definition::Typedef(self.parse_typedef()?));
                    }
                    Token::Enum => {
                        definitions.push(Definition::Enum(self.parse_enum()?));
                    }
                    Token::Struct => {
                        definitions.push(Definition::Struct(self.parse_struct()?));
                    }
                    Token::Union => {
                        definitions.push(Definition::Union(self.parse_union()?));
                    }
                    Token::Exception => {
                        definitions.push(Definition::Exception(self.parse_exception()?));
                    }
                    Token::Service => {
                        definitions.push(Definition::Service(self.parse_service()?));
                    }
                    _ => return Err(ParseError::UnexpectedToken(format!("{:?}", token))),
                },
                _ => return Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            }
        }

        Ok(Document {
            headers,
            definitions,
        })
    }

    fn parse_header(&mut self) -> Result<String, ParseError> {
        let mut header = String::new();
        while let Some(token) = self.tokens.next() {
            header.push_str(&format!("{:?} ", token));
            if token == Token::Semicolon {
                break;
            }
        }
        Ok(header.trim().to_string())
    }

    fn parse_const(&mut self) -> Result<Const, ParseError> {
        self.expect_token(Token::Const)?;
        let const_type = self.parse_type()?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::Equals)?;
        let value = self.parse_value()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Const {
            name,
            const_type,
            value,
        })
    }

    fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        self.expect_token(Token::Typedef)?;
        let type_ = self.parse_type()?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Typedef { name, type_ })
    }

    fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        self.expect_token(Token::Enum)?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftBrace)?;
        let mut variants = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if *token == Token::RightBrace {
                break;
            }
            let variant_name = self.expect_identifier()?;
            let value = if self.tokens.peek() == Some(&Token::Equals) {
                self.tokens.next(); // consume '='
                Some(self.parse_integer()?)
            } else {
                None
            };
            variants.push(EnumVariant {
                name: variant_name,
                value,
            });
            if self.tokens.peek() == Some(&Token::Comma) {
                self.tokens.next(); // consume ','
            }
        }
        self.expect_token(Token::RightBrace)?;
        Ok(Enum { name, variants })
    }

    fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        self.expect_token(Token::Struct)?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftBrace)?;
        let fields = self.parse_fields()?;
        self.expect_token(Token::RightBrace)?;
        Ok(Struct { name, fields })
    }

    fn parse_union(&mut self) -> Result<Union, ParseError> {
        self.expect_token(Token::Union)?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftBrace)?;
        let fields = self.parse_fields()?;
        self.expect_token(Token::RightBrace)?;
        Ok(Union { name, fields })
    }

    fn parse_exception(&mut self) -> Result<Exception, ParseError> {
        self.expect_token(Token::Exception)?;
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftBrace)?;
        let fields = self.parse_fields()?;
        self.expect_token(Token::RightBrace)?;
        Ok(Exception { name, fields })
    }

    fn parse_service(&mut self) -> Result<Service, ParseError> {
        self.expect_token(Token::Service)?;
        let name = self.expect_identifier()?;
        let extends = if self.tokens.peek() == Some(&Token::Extends) {
            self.tokens.next(); // consume 'extends'
            Some(self.expect_identifier()?)
        } else {
            None
        };
        self.expect_token(Token::LeftBrace)?;
        let mut functions = Vec::new();
        while self.tokens.peek() != Some(&Token::RightBrace) {
            functions.push(self.parse_function()?);
        }
        self.expect_token(Token::RightBrace)?;
        Ok(Service {
            name,
            extends,
            functions,
        })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        let oneway = if self.tokens.peek() == Some(&Token::Oneway) {
            self.tokens.next();
            true
        } else {
            false
        };
        let return_type = if self.tokens.peek() == Some(&Token::Void) {
            self.tokens.next();
            None
        } else {
            Some(self.parse_type()?)
        };
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftParen)?;
        let params = self.parse_fields()?;
        self.expect_token(Token::RightParen)?;
        let throws = if self.tokens.peek() == Some(&Token::Throws) {
            self.tokens.next();
            self.expect_token(Token::LeftParen)?;
            let throws = self.parse_fields()?;
            self.expect_token(Token::RightParen)?;
            throws
        } else {
            Vec::new()
        };
        self.expect_token(Token::Semicolon)?;
        Ok(Function {
            name,
            return_type,
            params,
            throws,
            oneway,
        })
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>, ParseError> {
        let mut fields = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if *token == Token::RightBrace || *token == Token::RightParen {
                break;
            }
            fields.push(self.parse_field()?);
        }
        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let id = if let Some(Token::IntegerLiteral) = self.tokens.peek() {
            Some(self.parse_integer()? as i16)
        } else {
            None
        };
        self.expect_token(Token::Colon)?;
        let required = match self.tokens.peek() {
            Some(Token::Required) => {
                self.tokens.next();
                true
            }
            Some(Token::Optional) => {
                self.tokens.next();
                false
            }
            _ => false,
        };
        let field_type = self.parse_type()?;
        let name = self.expect_identifier()?;
        let default = if self.tokens.peek() == Some(&Token::Equals) {
            self.tokens.next(); // consume '='
            Some(self.parse_value()?)
        } else {
            None
        };
        self.expect_token(Token::Semicolon)?;
        Ok(Field {
            id,
            name,
            field_type,
            required,
            default,
        })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.tokens.next() {
            Some(Token::Identifier) => {
                let type_name = self.tokens.slice().unwrap();
                if let Some(base_type) = Type::from_str(type_name) {
                    Ok(base_type)
                } else {
                    Ok(Type::Custom(type_name.to_string()))
                }
            }
            Some(Token::Map) => {
                self.expect_token(Token::LeftAngle)?;
                let key_type = Box::new(self.parse_type()?);
                self.expect_token(Token::Comma)?;
                let value_type = Box::new(self.parse_type()?);
                self.expect_token(Token::RightAngle)?;
                Ok(Type::Map(key_type, value_type))
            }
            Some(Token::List) => {
                self.expect_token(Token::LeftAngle)?;
                let elem_type = Box::new(self.parse_type()?);
                self.expect_token(Token::RightAngle)?;
                Ok(Type::List(elem_type))
            }
            Some(Token::Set) => {
                self.expect_token(Token::LeftAngle)?;
                let elem_type = Box::new(self.parse_type()?);
                self.expect_token(Token::RightAngle)?;
                Ok(Type::Set(elem_type))
            }
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.tokens.next() {
            Some(Token::IntegerLiteral) => {
                let value = self.tokens.slice().unwrap().parse().map_err(|_| {
                    ParseError::InvalidInteger(self.tokens.slice().unwrap().to_string())
                })?;
                Ok(Value::Integer(value))
            }
            Some(Token::DoubleLiteral) => {
                let value = self.tokens.slice().unwrap().parse().map_err(|_| {
                    ParseError::InvalidFloat(self.tokens.slice().unwrap().to_string())
                })?;
                Ok(Value::Double(value))
            }
            Some(Token::StringLiteral) => {
                let value = self.tokens.slice().unwrap().trim_matches('"').to_string();
                Ok(Value::String(value))
            }
            Some(Token::True) => Ok(Value::Bool(true)),
            Some(Token::False) => Ok(Value::Bool(false)),
            Some(Token::LeftBracket) => {
                let mut list = Vec::new();
                while self.tokens.peek() != Some(&Token::RightBracket) {
                    list.push(self.parse_value()?);
                    if self.tokens.peek() == Some(&Token::Comma) {
                        self.tokens.next();
                    }
                }
                self.expect_token(Token::RightBracket)?;
                Ok(Value::List(list))
            }
            Some(Token::LeftBrace) => {
                let mut map = std::collections::HashMap::new();
                while self.tokens.peek() != Some(&Token::RightBrace) {
                    let key = self.parse_value()?;
                    self.expect_token(Token::Colon)?;
                    let value = self.parse_value()?;
                    map.insert(key, value);
                    if self.tokens.peek() == Some(&Token::Comma) {
                        self.tokens.next();
                    }
                }
                self.expect_token(Token::RightBrace)?;
                Ok(Value::Map(map))
            }
            Some(Token::Identifier) => {
                Ok(Value::Identifier(self.tokens.slice().unwrap().to_string()))
            }
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_integer(&mut self) -> Result<i32, ParseError> {
        match self.tokens.next() {
            Some(Token::IntegerLiteral) => {
                self.tokens.slice().unwrap().parse().map_err(|_| {
                    ParseError::InvalidInteger(self.tokens.slice().unwrap().to_string())
                })
            }
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.tokens.next() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.tokens.next() {
            Some(Token::Identifier) => Ok(self.tokens.slice().unwrap().to_string()),
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}
