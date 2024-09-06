use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

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

    #[token("oneway")]
    Oneway,

    #[token("void")]
    Void,

    #[token("throws")]
    Throws,

    #[token("required")]
    Required,

    #[token("optional")]
    Optional,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r"-?[0-9]+")]
    IntegerLiteral,

    #[regex(r"-?[0-9]+\.[0-9]+([eE][-+]?[0-9]+)?")]
    DoubleLiteral,

    #[token("true")]
    True,

    #[token("false")]
    False,

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

    #[regex(r"//.*")]
    LineComment,

    #[regex(r"/\*[\s\S]*?\*/")]
    BlockComment,
}
