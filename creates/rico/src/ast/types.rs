use crate::lexer::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}
impl Span {
    pub fn new(line: usize, column: usize, index: usize) -> Self {
        Self {
            line,
            column,
            index,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LOC {
    pub start: Span,
    pub end: Span,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Common<T = String> {
    pub kind: NodeType,
    pub value: T,
    pub loc: LOC,
}
impl<T> Common<T> {
    pub fn new(kind: NodeType, value: T, loc: LOC) -> Self {
        Self { kind, value, loc }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    ThriftDocument,
    ThriftErrors,

    Identifier,
    FieldID,

    // Statements
    NamespaceDefinition,
    IncludeDefinition,
    CppIncludeDefinition,
    ConstDefinition,
    StructDefinition,
    EnumDefinition,
    ServiceDefinition,
    ExceptionDefinition,
    TypedefDefinition,
    UnionDefinition,

    // Fields
    FieldDefinition,
    FunctionDefinition,
    ParametersDefinition,
    ThrowsDefinition,

    // Type Annotations
    FieldType,
    BaseType,
    SetType,
    MapType,
    ListType,

    // Values
    ConstValue,
    IntConstant,
    DoubleConstant,

    ConstList,
    ConstMap,
    EnumMember,

    // Literals
    CommentLine,
    CommentBlock,
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    HexLiteral,
    ExponentialLiteral,
    BooleanLiteral,
    PropertyAssignment,

    // Keywords
    NamespaceKeyword,
    IncludeKeyword,
    CppIncludeKeyword,
    ExceptionKeyword,
    ServiceKeyword,
    ExtendsKeyword,
    RequiredKeyword,
    OptionalKeyword,
    FalseKeyword,
    TrueKeyword,
    ConstKeyword,
    DoubleKeyword,
    StructKeyword,
    TypedefKeyword,
    UnionKeyword,
    StringKeyword,
    BinaryKeyword,
    BoolKeyword,
    ByteKeyword,
    EnumKeyword,
    ListKeyword,
    SetKeyword,
    MapKeyword,
    I8Keyword,
    I16Keyword,
    I32Keyword,
    I64Keyword,
    ThrowsKeyword,
    VoidKeyword,
    OnewayKeyword,

    // Other
    Annotation,
    Annotations,

    EOF,
}

impl NodeType {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token {
            // Keywords
            Token::Namespace => Some(NodeType::NamespaceKeyword),
            Token::Include => Some(NodeType::IncludeKeyword),
            Token::Exception => Some(NodeType::ExceptionKeyword),
            Token::Service => Some(NodeType::ServiceKeyword),
            Token::Extends => Some(NodeType::ExtendsKeyword),
            Token::Required => Some(NodeType::RequiredKeyword),
            Token::Optional => Some(NodeType::OptionalKeyword),
            Token::Const => Some(NodeType::ConstKeyword),
            Token::Double => Some(NodeType::DoubleKeyword),
            Token::Struct => Some(NodeType::StructKeyword),
            Token::Typedef => Some(NodeType::TypedefKeyword),
            Token::Union => Some(NodeType::UnionKeyword),
            Token::String => Some(NodeType::StringKeyword),
            Token::Binary => Some(NodeType::BinaryKeyword),
            Token::Bool => Some(NodeType::BoolKeyword),
            Token::Byte => Some(NodeType::ByteKeyword),
            Token::Enum => Some(NodeType::EnumKeyword),
            Token::List => Some(NodeType::ListKeyword),
            Token::Set => Some(NodeType::SetKeyword),
            Token::Map => Some(NodeType::MapKeyword),
            Token::I16 => Some(NodeType::I16Keyword),
            Token::I32 => Some(NodeType::I32Keyword),
            Token::I64 => Some(NodeType::I64Keyword),
            Token::Throws => Some(NodeType::ThrowsKeyword),
            Token::Void => Some(NodeType::VoidKeyword),
            // Literals
            Token::LineComment => Some(NodeType::CommentLine),
            Token::BlockComment => Some(NodeType::CommentBlock),
            Token::StringLiteral => Some(NodeType::StringLiteral),
            Token::IntegerLiteral => Some(NodeType::IntegerLiteral),
            Token::DoubleLiteral => Some(NodeType::FloatLiteral),
            Token::BooleanLiteral => Some(NodeType::BooleanLiteral),
            Token::HexLiteral => Some(NodeType::HexLiteral),
            _ => None,
        }
    }
}
