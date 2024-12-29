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

    #[error("Missing namespace identifier")]
    #[diagnostic(
        code(rico::parser::missing_namespace_identifier),
        help("Namespace declaration requires a scope and name identifier, use like: \"namespace cpp my.namespace\"")
    )]
    MissingNamespaceIdentifier {
        #[label("Expected namespace identifier here")]
        span: SourceSpan,
    },

    #[error("Missing namespace scope")]
    #[diagnostic(
        code(rico::parser::missing_namespace_scope),
        help("Namespace declaration requires a scope (e.g., cpp, java, py), use like: \"namespace cpp my.namespace\"")
    )]
    MissingNamespaceScope {
        #[label("Expected namespace scope (cpp, java, py, etc.) here")]
        span: SourceSpan,
    },

    #[error("Missing include path")]
    #[diagnostic(
        code(rico::parser::missing_include_identifier),
        help("Include statement requires a string literal path, use like: \"include \\\"myservice.thrift\\\"\"")
    )]
    MissingIncludeIdentifier {
        #[label("Expected string literal path here")]
        span: SourceSpan,
    },

    #[error("Missing const identifier")]
    #[diagnostic(
        code(rico::parser::missing_const_identifier),
        help("Const declaration requires a name, use like: \"const i32 MAX_RETRIES = 3\"")
    )]
    MissingConstIdentifier {
        #[label("Expected const name here")]
        span: SourceSpan,
    },

    #[error("Missing typedef identifier")]
    #[diagnostic(
        code(rico::parser::missing_typedef_identifier),
        help("Typedef requires a name, use like: \"typedef i32 UserId\"")
    )]
    MissingTypedefIdentifier {
        #[label("Expected typedef name here")]
        span: SourceSpan,
    },

    #[error("Missing enum identifier")]
    #[diagnostic(
        code(rico::parser::missing_enum_identifier),
        help("Enum declaration requires a name, use like: \"enum Status {{ OK = 1 }}\"")
    )]
    MissingEnumIdentifier {
        #[label("Expected enum name here")]
        span: SourceSpan,
    },

    #[error("Missing struct identifier")]
    #[diagnostic(
        code(rico::parser::missing_struct_identifier),
        help("Struct declaration requires a name, use like: \"struct User {{ 1: string name }}\"")
    )]
    MissingStructIdentifier {
        #[label("Expected struct name here")]
        span: SourceSpan,
    },

    #[error("Missing service identifier")]
    #[diagnostic(
        code(rico::parser::missing_service_identifier),
        help("Service declaration requires a name, use like: \"service UserService {{ }}\"")
    )]
    MissingServiceIdentifier {
        #[label("Expected service name here")]
        span: SourceSpan,
    },

    #[error("Missing service extends")]
    #[diagnostic(
        code(rico::parser::missing_service_extends),
        help("Service extends clause requires a service name, use like: \"service UserService extends BaseService {{ }}\"")
    )]
    MissingServiceExtends {
        #[label("Expected service name after extends keyword")]
        span: SourceSpan,
    },

    #[error("Invalid enum member name")]
    #[diagnostic(
        code(rico::parser::invalid_enum_member_name),
        help("Enum member names must be valid identifiers and cannot be keywords")
    )]
    InvalidEnumMemberName {
        #[label("This enum member name is invalid")]
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
            ParseErrorKind::MissingNamespaceIdentifier => {
                Self::MissingNamespaceIdentifier { span: source_span }
            }
            ParseErrorKind::MissingNamespaceScope => {
                Self::MissingNamespaceScope { span: source_span }
            }
            ParseErrorKind::MissingIncludeIdentifier => {
                Self::MissingIncludeIdentifier { span: source_span }
            }
            ParseErrorKind::MissingConstIdentifier => {
                Self::MissingConstIdentifier { span: source_span }
            }
            ParseErrorKind::MissingTypedefIdentifier => {
                Self::MissingTypedefIdentifier { span: source_span }
            }
            ParseErrorKind::MissingEnumIdentifier => {
                Self::MissingEnumIdentifier { span: source_span }
            }
            ParseErrorKind::MissingStructIdentifier => {
                Self::MissingStructIdentifier { span: source_span }
            }
            ParseErrorKind::MissingServiceIdentifier => {
                Self::MissingServiceIdentifier { span: source_span }
            }
            ParseErrorKind::MissingServiceExtends => {
                Self::MissingServiceExtends { span: source_span }
            }
            ParseErrorKind::InvalidEnumMemberName => {
                Self::InvalidEnumMemberName { span: source_span }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnrecognizedToken,
    UnexpectedToken,
    UnexpectedEOF,
    UnsupportedType,
    MissingTypeDeclaration,
    InvalidValueDeclaration,
    InvalidReturnType,
    InvalidFieldName,
    InvalidFieldId,
    MissingNamespaceIdentifier,
    MissingNamespaceScope,
    MissingIncludeIdentifier,
    MissingConstIdentifier,
    MissingTypedefIdentifier,
    MissingEnumIdentifier,
    MissingStructIdentifier,
    MissingServiceIdentifier,
    MissingServiceExtends,
    InvalidEnumMemberName,
}
