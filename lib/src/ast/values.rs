use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Double(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Identifier(String),
}
