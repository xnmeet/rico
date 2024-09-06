use super::types::Type;
use super::values::Value;

#[derive(Debug)]
pub enum Definition {
    Const(Const),
    Typedef(Typedef),
    Enum(Enum),
    Struct(Struct),
    Union(Union),
    Exception(Exception),
    Service(Service),
}

#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub const_type: Type,
    pub value: Value,
}

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<i32>,
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Union {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Exception {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub id: Option<i16>,
    pub name: String,
    pub field_type: Type,
    pub required: bool,
    pub default: Option<Value>,
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub extends: Option<String>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<Type>,
    pub params: Vec<Field>,
    pub throws: Vec<Field>,
    pub oneway: bool,
}

#[derive(Debug)]
pub struct Document {
    pub headers: Vec<String>,
    pub definitions: Vec<Definition>,
}
