use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    // Keywords
    #[token("namespace")]
    Namespace,
    #[token("include")]
    Include,
    #[token("typedef")]
    Typedef,
    #[token("const")]
    Const,
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("exception")]
    Exception,
    #[token("service")]
    Service,
    #[token("extends")]
    Extends,
    #[token("throws")]
    Throws,
    #[token("required")]
    Required,
    #[token("optional")]
    Optional,

    // Base types
    #[token("bool")]
    Bool,
    #[token("byte")]
    Byte,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("double")]
    Double,
    #[token("string")]
    String,
    #[token("binary")]
    Binary,
    #[token("map")]
    Map,
    #[token("list")]
    List,
    #[token("set")]
    Set,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]+)+", priority = 3)]
    ChainIdentifier,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r"\d+:", priority = 2)]
    FieldId,

    #[regex(r"[+-]?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    IntegerLiteral(i64),

    #[regex(r"[+-]?[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap() )]
    DoubleLiteral(f64),

    #[token("true")]
    True,
    #[token("false")]
    False,

    // Punctuation
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("<")]
    LeftAngle,
    #[token(">")]
    RightAngle,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token(".")]
    Dot,

    // Comments
    #[regex(r"//.*")]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
}
