use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Debug)]
pub struct LOC {
    pub start: Span,
    pub end: Span,
}
#[derive(Debug)]
pub struct Common<T = String> {
    pub kind: NodeType,
    pub value: T,
    pub loc: LOC,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
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

    // Tokens
    LeftParenToken,
    RightParenToken,
    LeftBraceToken,
    RightBraceToken,
    LeftBracketToken,
    RightBracketToken,
    CommaToken,
    DotToken,
    MinusToken,
    SemicolonToken,
    ColonToken,
    StarToken,
    EqualToken,
    LessThanToken,
    GreaterThanToken,

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
    SenumKeyword,
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
