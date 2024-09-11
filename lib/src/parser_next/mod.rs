mod error;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser_next::error::ParseError;
use error::Loc;
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
        let mut headers = Vec::new();
        let mut definitions = Vec::new();

        let start_span = self.lexer.span();

        self.advance();
        loop {
            self.skip_comments();
            if let Some(token) = &self.current_token {
                match token {
                    Token::Include => {
                        headers.push(Headers::Include(self.parse_include()?));
                    }
                    Token::Namespace => {
                        headers.push(Headers::Namespace(self.parse_namespace()?));
                    }
                    _ => return Err(ParseError::UnexpectedToken(self.current_loc())),
                }
                self.advance();
            } else {
                break;
            }
        }

        Ok(Document {
            span: Span {
                start: start_span.start,
                end: self.lexer.span().end,
            },
            headers,
            kind: String::from("Document"),
            definitions,
        })
    }

    fn advance(&mut self) -> Option<&Token> {
        let current_token = self.lexer.next();
        if let Some(Ok(token)) = current_token {
            self.current_token = Some(token);
        } else {
            self.current_token = None;
        }
        return self.current_token.as_ref();
    }

    fn current_loc(&self) -> Loc {
        let line = self.lexer.extras.0;
        let column = self.lexer.span().start - self.lexer.extras.1;
        Loc { line, column }
    }

    fn parse_include(&mut self) -> Result<Include, ParseError> {
        let start_span = self.lexer.span();
        self.advance();
        self.expect_token(Token::StringLiteral)?;
        let value = self.lexer.slice();

        Ok(Include {
            name: "include".to_string(),
            kind: "Include".to_string(),
            span: Span {
                start: start_span.start,
                end: self.lexer.span().end,
            },
            value: Value::String(value.to_string()),
        })
    }

    fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        let start_span = self.lexer.span();

        self.advance();
        self.expect_token(Token::Identifier)?;
        // for example: namespace go a.b.c
        let indent_scope = self.lexer.slice(); // result go

        self.advance();
        self.expect_token(Token::ChainIdentifier)?;
        let space = self.lexer.slice(); // result a.b.c
        let combined = format!("{}{}", indent_scope, space);

        Ok(Namespace {
            name: "namespace".to_string(),
            kind: "Namespace".to_string(),
            span: Span {
                start: start_span.start,
                end: start_span.end,
            },
            value: Value::String(combined),
        })
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        match &self.current_token {
            Some(token) if token == &expected => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken(self.current_loc())),
            None => Err(ParseError::UnexpectedEOF(self.current_loc())),
        }
    }

    fn skip_comments(&mut self) {
        loop {
            if let Some(token) = &self.current_token {
                if token == &Token::LineComment || token == &Token::BlockComment {
                    self.advance();
                }
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let input = "include \"test.thrift\";";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        print!("result {:?}", result)
    }

    // #[test]
    // fn test_parse_const() {
    //     let input = "const i32 TEST_CONST = 42;";
    //     let mut parser = Parser::new(input);
    //     let result = parser.parse_const();
    //     print!("result {:?}", result)
    // }

    // #[test]
    // fn test_parse_typedef() {
    //     let input = "typedef i32 TestType;";
    //     let mut parser = Parser::new(input);
    //     let result = parser.parse_typedef();
    //     print!("result {:?}", result)
    // }
}
