#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Bool,
    Byte,
    I16,
    I32,
    I64,
    Double,
    String,
    Binary,
    Map(Box<Type>, Box<Type>),
    List(Box<Type>),
    Set(Box<Type>),
    Custom(String),
}

impl Type {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bool" => Some(Type::Bool),
            "byte" => Some(Type::Byte),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "double" => Some(Type::Double),
            "string" => Some(Type::String),
            "binary" => Some(Type::Binary),
            _ => Some(Type::Custom(s.to_string())),
        }
    }
}
