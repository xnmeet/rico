use super::types::Type;
use super::values::Value;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum Headers {
    Include(Include),
    Namespace(Namespace),
}

#[derive(Debug)]
pub enum Definition {
    Const(Const),
    Typedef(Typedef),
    Enum(Enum),
    Struct(Struct),
    Exception(Exception),
    Service(Service),
}

#[derive(Debug)]
pub struct Include {
    pub name: String,
    pub value: Value,
    pub span: Span,
    pub kind: String, // include
}

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub value: Value,
    pub span: Span,
    pub kind: String, // namespace
}

#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub type_: Type,
    pub value: Value,
    pub span: Span,
    pub kind: String, // const
}

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
    pub span: Span,
    pub kind: String, // typedef
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
    pub kind: String, // enum
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<i32>,
    pub span: Span,
    pub kind: String, // enum_variant
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
    pub kind: String, // struct
}

#[derive(Debug)]
pub struct Exception {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
    pub kind: String, // exception
}

#[derive(Debug)]
pub struct Field {
    pub id: Option<i16>,
    pub name: String,
    pub type_: Type,
    pub required: bool,
    pub default: Option<Value>,
    pub annotations: Option<Vec<(String, String)>>,
    pub span: Span,
    pub kind: String, // field
}

#[derive(Debug)]
pub struct Params {
    pub id: Option<i16>,
    pub name: String,
    pub type_: Type,
    pub span: Span,
    pub kind: String, // params
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub extends: Option<String>,
    pub functions: Vec<Function>,
    pub span: Span,
    pub kind: String, // service
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<Type>,
    pub params: Vec<Params>,
    pub span: Span,
    pub kind: String, // function
}

#[derive(Debug)]
pub struct Document {
    pub headers: Vec<Headers>,
    pub definitions: Vec<Definition>,
    pub span: Span,
    pub kind: String, // document
}
