use crate::lexer::Token;

use super::{Common, FieldType, NodeType, LOC};

pub fn create_keyword_field_type(token: &Token, loc: LOC, slice: &str) -> FieldType {
    FieldType {
        kind: NodeType::from_token(token).unwrap(),
        loc,
        value: slice.to_string(),
        key_type: None,
        value_type: None,
    }
}

pub fn create_map_field_type(
    loc: LOC,
    slice: &str,
    key_type: Common,
    value_type: Common,
) -> FieldType {
    FieldType {
        kind: NodeType::MapType,
        loc,
        key_type: Some(key_type),
        value_type: Some(value_type),
        value: slice.to_string(),
    }
}

pub fn create_list_field_type(loc: LOC, slice: &str, value_type: Common) -> FieldType {
    FieldType {
        kind: NodeType::ListType,
        loc,
        key_type: None,
        value_type: Some(value_type),
        value: slice.to_string(),
    }
}

pub fn create_set_field_type(loc: LOC, slice: &str, value_type: Common) -> FieldType {
    FieldType {
        kind: NodeType::SetType,
        loc,
        key_type: None,
        value_type: Some(value_type),
        value: slice.to_string(),
    }
}
