use logos::Span;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Unrecognized token")]
    #[diagnostic(
        code(rico::parser::unrecognized_token),
        help("Found a token that is not valid in the Thrift IDL syntax")
    )]
    UnrecognizedToken {
        #[label("This token is not recognized in Thrift IDL")]
        span: SourceSpan,
    },

    #[error("Unexpected token")]
    #[diagnostic(
        code(rico::parser::unexpected_token),
        help("Expected a different token here")
    )]
    UnexpectedToken {
        #[label("This token was not expected in this context")]
        span: SourceSpan,
    },

    #[error("Unexpected end of file")]
    #[diagnostic(
        code(rico::parser::unexpected_eof),
        help("The file ended unexpectedly, you might be missing some closing tokens")
    )]
    UnexpectedEOF {
        #[label("The file ended here")]
        span: SourceSpan,
    },

    #[error("Invalid integer value")]
    #[diagnostic(
        code(rico::parser::invalid_integer),
        help("Integer values must be valid numbers")
    )]
    InvalidInteger {
        #[label("This integer value is invalid")]
        span: SourceSpan,
    },

    #[error("Invalid float value")]
    #[diagnostic(
        code(rico::parser::invalid_float),
        help("Float values must be valid decimal numbers")
    )]
    InvalidFloat {
        #[label("This float value is invalid")]
        span: SourceSpan,
    },

    #[error("Lexer error")]
    #[diagnostic(
        code(rico::parser::lexer_error),
        help("There was an error tokenizing the input")
    )]
    LexerError {
        #[label("Error occurred here")]
        span: SourceSpan,
    },

    #[error("Nested complex type")]
    #[diagnostic(
        code(rico::parser::nested_complex_type),
        help("Complex types cannot be nested directly. Consider using a typedef")
    )]
    NestedComplexType {
        #[label("This complex type is nested")]
        span: SourceSpan,
    },

    #[error("Unsupported type")]
    #[diagnostic(
        code(rico::parser::unsupported_type),
        help("This type is not supported in Thrift IDL")
    )]
    UnsupportedType {
        #[label("This type is not supported")]
        span: SourceSpan,
    },

    #[error("Missing type declaration")]
    #[diagnostic(
        code(rico::parser::missing_type),
        help("A type declaration is required here")
    )]
    MissingTypeDeclaration {
        #[label("Type declaration is missing")]
        span: SourceSpan,
    },

    #[error("Invalid value declaration")]
    #[diagnostic(
        code(rico::parser::invalid_value),
        help("The value declaration is not valid in this context")
    )]
    InvalidValueDeclaration {
        #[label("This value declaration is invalid")]
        span: SourceSpan,
    },

    #[error("Invalid return type")]
    #[diagnostic(
        code(rico::parser::invalid_return_type),
        help("The return type is not valid for this function")
    )]
    InvalidReturnType {
        #[label("This return type is invalid")]
        span: SourceSpan,
    },

    #[error("Invalid field name")]
    #[diagnostic(
        code(rico::parser::invalid_field_name),
        help("Field names must be valid identifiers")
    )]
    InvalidFieldName {
        #[label("This field name is invalid")]
        span: SourceSpan,
    },

    #[error("Invalid field ID")]
    #[diagnostic(
        code(rico::parser::invalid_field_id),
        help("Field IDs must be positive integers,use like: \"1: string name\"")
    )]
    InvalidFieldId {
        #[label("This field ID is invalid")]
        span: SourceSpan,
    },
}

// Helper function to convert our Span to miette's SourceSpan
impl ParseError {
    pub(crate) fn from_loc(span: Span, kind: ParseErrorKind) -> Self {
        let source_span = SourceSpan::new(span.start.into(), span.end - span.start);

        match kind {
            ParseErrorKind::UnrecognizedToken => Self::UnrecognizedToken { span: source_span },
            ParseErrorKind::UnexpectedToken => Self::UnexpectedToken { span: source_span },
            ParseErrorKind::UnexpectedEOF => Self::UnexpectedEOF { span: source_span },
            ParseErrorKind::UnsupportedType => Self::UnsupportedType { span: source_span },
            ParseErrorKind::MissingTypeDeclaration => {
                Self::MissingTypeDeclaration { span: source_span }
            }
            ParseErrorKind::InvalidValueDeclaration => {
                Self::InvalidValueDeclaration { span: source_span }
            }
            ParseErrorKind::InvalidReturnType => Self::InvalidReturnType { span: source_span },
            ParseErrorKind::InvalidFieldName => Self::InvalidFieldName { span: source_span },
            ParseErrorKind::InvalidFieldId => Self::InvalidFieldId { span: source_span },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ParseErrorKind {
    UnrecognizedToken,
    UnexpectedToken,
    UnexpectedEOF,
    UnsupportedType,
    MissingTypeDeclaration,
    InvalidValueDeclaration,
    InvalidReturnType,
    InvalidFieldName,
    InvalidFieldId,
}
