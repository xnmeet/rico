use crate::lexer::Token;

use super::{Common, FiledType, NodeType, LOC};

pub fn create_keyword_field_type(token: &Token, loc: LOC, slice: &str) -> FiledType {
    FiledType {
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
) -> FiledType {
    FiledType {
        kind: NodeType::MapKeyword,
        loc,
        key_type: Some(key_type),
        value_type: Some(value_type),
        value: slice.to_string(),
    }
}

pub fn create_list_field_type(loc: LOC, slice: &str, value_type: Common) -> FiledType {
    FiledType {
        kind: NodeType::ListKeyword,
        loc,
        key_type: None,
        value_type: Some(value_type),
        value: slice.to_string(),
    }
}
