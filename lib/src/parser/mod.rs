// mod error;

// use crate::lexer::Token;
// use crate::parser::error::ParseError;
// use crate::{ast::*, lexer};
// use logos::Logos;

// pub struct Parser<'a> {
//     lexer: logos::Lexer<'a, Token>,
//     current_token: Option<Token>,
// }

// impl<'a> Parser<'a> {
//     pub fn new(input: &'a str) -> Self {
//         let lexer = Token::lexer(input);
//         Parser {
//             lexer,
//             current_token: None,
//         }
//     }

//     pub fn parse(&mut self) -> Result<Document, ParseError> {
//         let mut headers = Vec::new();
//         let mut definitions = Vec::new();

//         let start_span = self.lexer.span();
//         while let Some(Ok(token)) = self.advance() {
//             self.skip_comments();
//             match token {
//                 Token::Namespace | Token::Include => {
//                     headers.push(self.parse_header()?);
//                 }
//                 Token::Const => {
//                     definitions.push(Definition::Const(self.parse_const()?));
//                 }
//                 // Token::Typedef => {
//                 //     definitions.push(Definition::Typedef(self.parse_typedef()?));
//                 // }
//                 // Token::Enum => {
//                 //     definitions.push(Definition::Enum(self.parse_enum()?));
//                 // }
//                 // Token::Struct => {
//                 //     definitions.push(Definition::Struct(self.parse_struct()?));
//                 // }
//                 // Token::Union => {
//                 //     definitions.push(Definition::Union(self.parse_union()?));
//                 // }
//                 // Token::Exception => {
//                 //     definitions.push(Definition::Exception(self.parse_exception()?));
//                 // }
//                 // Token::Service => {
//                 //     definitions.push(Definition::Service(self.parse_service()?));
//                 // }
//                 _ => return Err(ParseError::UnexpectedToken(format!("{:?}", token))),
//             }
//         }

//         Ok(Document {
//             span: Span {
//                 start: start_span.start,
//                 end: self.lexer.span().end,
//             },
//             headers,
//             definitions,
//         })
//     }

//     fn advance(&mut self) -> Option<Result<Token, ()>> {
//         let current_token = self.lexer.next();
//         if let Some(Ok(token)) = &current_token {
//             self.current_token = Some(token.clone());
//         }
//         return current_token;
//     }

//     fn parse_header(&mut self) -> Result<String, ParseError> {
//         let mut header = String::new();
//         loop {
//             if let Some(token) = &self.current_token {
//                 header.push_str(self.lexer.slice());
//                 if *token == Token::Semicolon {
//                     break;
//                 }
//                 self.advance();
//             }
//         }
//         Ok(header.trim().to_string())
//     }

//     fn parse_const(&mut self) -> Result<Const, ParseError> {
//         let start_span = self.lexer.span();

//         self.expect_token(Token::Const)?;
//         let const_type = self.parse_type()?;
//         let name = self.expect_identifier()?;
//         self.expect_token(Token::Equals)?;
//         let value = self.parse_value()?;
//         self.expect_token(Token::Semicolon)?;

//         Ok(Const {
//             span: Span {
//                 start: start_span.start,
//                 end: self.lexer.span().end,
//             },
//             name,
//             const_type,
//             value,
//         })
//     }

//     fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
//         let start_span = self.lexer.span();
//         self.expect_token(Token::Typedef)?;
//         let type_ = self.parse_type()?;
//         let name = self.expect_identifier()?;
//         self.expect_token(Token::Semicolon)?;
//         Ok(Typedef {
//             name,
//             type_,
//             span: Span {
//                 start: start_span.start,
//                 end: self.lexer.span().end,
//             },
//         })
//     }

//     // fn parse_enum(&mut self) -> Result<Enum, ParseError> {
//     //     self.skip_comments();
//     //     self.expect_token(Token::Enum)?;
//     //     let name = self.expect_identifier()?;
//     //     self.expect_token(Token::LeftBrace)?;
//     //     let mut variants = Vec::new();
//     //     while let Some(Ok(token)) = self.tokens.peek() {
//     //         if *token == Token::RightBrace {
//     //             break;
//     //         }
//     //         let variant_name = self.expect_identifier()?;
//     //         let value = if let Some(Ok(Token::Equals)) = self.tokens.peek() {
//     //             self.tokens.next(); // consume '='
//     //             Some(self.parse_integer()?)
//     //         } else {
//     //             None
//     //         };
//     //         variants.push(EnumVariant {
//     //             name: variant_name,
//     //             value,
//     //         });
//     //         if let Some(Ok(Token::Comma)) = self.tokens.peek() {
//     //             self.tokens.next(); // consume ','
//     //         }
//     //     }
//     //     self.expect_token(Token::RightBrace)?;
//     //     Ok(Enum { name, variants })
//     // }

//     // fn parse_struct(&mut self) -> Result<Struct, ParseError> {
//     //     self.expect_token(Token::Struct)?;
//     //     let name = self.expect_identifier()?;
//     //     self.expect_token(Token::LeftBrace)?;
//     //     let fields = self.parse_fields()?;
//     //     self.expect_token(Token::RightBrace)?;
//     //     Ok(Struct { name, fields })
//     // }

//     // fn parse_union(&mut self) -> Result<Union, ParseError> {
//     //     self.expect_token(Token::Union)?;
//     //     let name = self.expect_identifier()?;
//     //     self.expect_token(Token::LeftBrace)?;
//     //     let fields = self.parse_fields()?;
//     //     self.expect_token(Token::RightBrace)?;
//     //     Ok(Union { name, fields })
//     // }

//     // fn parse_exception(&mut self) -> Result<Exception, ParseError> {
//     //     self.expect_token(Token::Exception)?;
//     //     let name = self.expect_identifier()?;
//     //     self.expect_token(Token::LeftBrace)?;
//     //     let fields = self.parse_fields()?;
//     //     self.expect_token(Token::RightBrace)?;
//     //     Ok(Exception { name, fields })
//     // }

//     // fn parse_service(&mut self) -> Result<Service, ParseError> {
//     //     self.expect_token(Token::Service)?;
//     //     let name = self.expect_identifier()?;
//     //     let extends = if let Some(Ok(Token::Extends)) = self.tokens.peek() {
//     //         self.tokens.next(); // consume 'extends'
//     //         Some(self.expect_identifier()?)
//     //     } else {
//     //         None
//     //     };
//     //     self.expect_token(Token::LeftBrace)?;
//     //     let mut functions = Vec::new();
//     //     while let Some(Ok(token)) = self.tokens.peek() {
//     //         if *token == Token::RightBrace {
//     //             break;
//     //         }
//     //         functions.push(self.parse_function()?);
//     //     }
//     //     self.expect_token(Token::RightBrace)?;
//     //     Ok(Service {
//     //         name,
//     //         extends,
//     //         functions,
//     //     })
//     // }

//     // fn parse_function(&mut self) -> Result<Function, ParseError> {
//     //     let oneway = if let Some(Ok(Token::Oneway)) = self.tokens.peek() {
//     //         self.tokens.next();
//     //         true
//     //     } else {
//     //         false
//     //     };
//     //     let return_type = if let Some(Ok(Token::Void)) = self.tokens.peek() {
//     //         self.tokens.next();
//     //         None
//     //     } else {
//     //         Some(self.parse_type()?)
//     //     };
//     //     let name = self.expect_identifier()?;
//     //     self.expect_token(Token::LeftParen)?;
//     //     let params = self.parse_fields()?;
//     //     self.expect_token(Token::RightParen)?;
//     //     let throws = if let Some(Ok(Token::Throws)) = self.tokens.peek() {
//     //         self.tokens.next();
//     //         self.expect_token(Token::LeftParen)?;
//     //         let throws = self.parse_fields()?;
//     //         self.expect_token(Token::RightParen)?;
//     //         throws
//     //     } else {
//     //         Vec::new()
//     //     };
//     //     self.expect_token(Token::Semicolon)?;
//     //     Ok(Function {
//     //         name,
//     //         return_type,
//     //         params,
//     //         throws,
//     //         oneway,
//     //     })
//     // }

//     // fn parse_fields(&mut self) -> Result<Vec<Field>, ParseError> {
//     //     let mut fields = Vec::new();
//     //     while let Some(Ok(token)) = self.tokens.peek() {
//     //         if *token == Token::RightBrace || *token == Token::RightParen {
//     //             break;
//     //         }
//     //         fields.push(self.parse_field()?);
//     //     }
//     //     Ok(fields)
//     // }

//     // fn parse_field(&mut self) -> Result<Field, ParseError> {
//     //     let id = if let Some(Ok((Token::IntegerLiteral))) = self.tokens.peek() {
//     //         Some(self.parse_integer()? as i16)
//     //     } else {
//     //         None
//     //     };
//     //     self.expect_token(Token::Colon)?;
//     //     let required = match self.tokens.peek() {
//     //         Some(Ok(Token::Required)) => {
//     //             self.tokens.next();
//     //             true
//     //         }
//     //         Some(Ok(Token::Optional)) => {
//     //             self.tokens.next();
//     //             false
//     //         }
//     //         _ => false,
//     //     };
//     //     let field_type = self.parse_type()?;
//     //     let name = self.expect_identifier()?;
//     //     let default = if let Some(Ok(Token::Equals)) = self.tokens.peek() {
//     //         self.tokens.next(); // consume '='
//     //         Some(self.parse_value()?)
//     //     } else {
//     //         None
//     //     };
//     //     self.expect_token(Token::Semicolon)?;
//     //     Ok(Field {
//     //         id,
//     //         name,
//     //         field_type,
//     //         required,
//     //         default,
//     //     })
//     // }

//     fn parse_type(&mut self) -> Result<Type, ParseError> {
//         match &self.current_token {
//             Some(Token::Identifier) => {
//                 let type_name = self.lexer.slice();
//                 if let Some(base_type) = Type::from_str(type_name) {
//                     Ok(base_type)
//                 } else {
//                     Ok(Type::Custom(type_name.to_string()))
//                 }
//             }
//             // Some(Ok(Token::Map)) => {
//             //     self.expect_token(Token::LeftAngle)?;
//             //     let key_type = Box::new(self.parse_type()?);
//             //     self.expect_token(Token::Comma)?;
//             //     let value_type = Box::new(self.parse_type()?);
//             //     self.expect_token(Token::RightAngle)?;
//             //     Ok(Type::Map(key_type, value_type))
//             // }
//             // Some(Ok(Token::List)) => {
//             //     self.expect_token(Token::LeftAngle)?;
//             //     let elem_type = Box::new(self.parse_type()?);
//             //     self.expect_token(Token::RightAngle)?;
//             //     Ok(Type::List(elem_type))
//             // }
//             // Some(Ok(Token::Set)) => {
//             //     self.expect_token(Token::LeftAngle)?;
//             //     let elem_type = Box::new(self.parse_type()?);
//             //     self.expect_token(Token::RightAngle)?;
//             //     Ok(Type::Set(elem_type))
//             // }
//             Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn parse_value(&mut self) -> Result<Value, ParseError> {
//         match &self.current_token {
//             Some(Token::IntegerLiteral(num)) => {
//                 // let value = self.lexer.slice();
//                 Ok(Value::Integer(*num))
//             }
//             // Some(Ok(Token::DoubleLiteral)) => {
//             //     let value = self.tokens.source().slice(..).parse().map_err(|_| {
//             //         ParseError::InvalidFloat(self.tokens.source().slice(..).to_string())
//             //     })?;
//             //     Ok(Value::Double(value))
//             // }
//             Some(Token::StringLiteral) => {
//                 let value = self.lexer.slice();
//                 Ok(Value::String(value.to_string()))
//             }
//             // Some(Ok(Token::True)) => Ok(Value::Bool(true)),
//             // Some(Ok(Token::False)) => Ok(Value::Bool(false)),
//             // Some(Ok(Token::LeftBracket)) => {
//             //     let mut list = Vec::new();
//             //     while self.tokens.peek() != Some(&Token::RightBracket) {
//             //         list.push(self.parse_value()?);
//             //         if self.tokens.peek() == Some(&Token::Comma) {
//             //             self.tokens.next();
//             //         }
//             //     }
//             //     self.expect_token(Token::RightBracket)?;
//             //     Ok(Value::List(list))
//             // }
//             // Some(Ok(Token::LeftBrace)) => {
//             //     let mut map = std::collections::HashMap::new();
//             //     while self.tokens.peek() != Some(&Token::RightBrace) {
//             //         let key = self.parse_value()?;
//             //         self.expect_token(Token::Colon)?;
//             //         let value = self.parse_value()?;
//             //         map.insert(key, value);
//             //         if self.tokens.peek() == Some(&Token::Comma) {
//             //             self.tokens.next();
//             //         }
//             //     }
//             //     self.expect_token(Token::RightBrace)?;
//             //     Ok(Value::Map(map))
//             // }
//             // Some(Ok(Token::Identifier)) => Ok(Value::Identifier(
//             //     self.tokens.source().slice(..).to_string(),
//             // )),
//             Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     // fn parse_integer(&mut self) -> Result<i32, ParseError> {
//     //     match self.tokens.next() {
//     //         Some(Ok(Token::IntegerLiteral)) => {
//     //             self.tokens.source().slice(..).parse().map_err(|_| {
//     //                 ParseError::InvalidInteger(self.tokens.source().slice(..).to_string())
//     //             })
//     //         }
//     //         Some(Ok(token)) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
//     //         Some(Err(_)) => Err(ParseError::LexerError),
//     //         None => Err(ParseError::UnexpectedEOF),
//     //     }
//     // }

//     fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
//         match &self.current_token {
//             Some(token) if token == &expected => Ok(()),
//             Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn expect_identifier(&mut self) -> Result<String, ParseError> {
//         match &self.current_token {
//             Some(token) => {
//                 if token == &Token::Identifier {
//                     return Ok(format!("{:?}", token));
//                 }
//                 return Err(ParseError::UnexpectedToken(format!("{:?}", token)));
//             }
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn skip_comments(&mut self) {
//         loop {
//             if let Some(token) = &self.current_token {
//                 if token == &Token::LineComment || token == &Token::BlockComment {
//                     self.advance();
//                 }
//             }
//             break;
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_header() {
//         let input = "include \"test.thrift\";";
//         let mut parser = Parser::new(input);
//         let result = parser.parse_header();
//         print!("result {:?}", result)
//     }

//     #[test]
//     fn test_parse_const() {
//         let input = "const i32 TEST_CONST = 42;";
//         let mut parser = Parser::new(input);
//         let result = parser.parse_const();
//         print!("result {:?}", result)
//     }

//     #[test]
//     fn test_parse_typedef() {
//         let input = "typedef i32 TestType;";
//         let mut parser = Parser::new(input);
//         let result = parser.parse_typedef();
//         print!("result {:?}", result)
//     }
// }
