use crate::ast::*;
use crate::lexer::Token;

use super::{
    Common, ConstList, ConstMap, FieldInitialValue, FieldMapType, FieldType, MapProperty, NodeType,
    LOC,
};

// for field type

pub fn create_keyword_field_type(token: &Token, loc: LOC, slice: &str) -> FieldType {
    FieldType::CommonType(Common {
        kind: NodeType::from_token(token).unwrap(),
        loc,
        value: slice.to_string(),
    })
}

pub fn create_identifier_field_type(loc: LOC, slice: &str) -> FieldType {
    FieldType::CommonType(Common {
        kind: NodeType::Identifier,
        loc,
        value: slice.to_string(),
    })
}

pub fn create_map_field_type(
    loc: LOC,
    slice: &str,
    key_type: FieldType,
    value_type: FieldType,
) -> FieldType {
    FieldType::MapType(FieldMapType {
        loc,
        key_type: Box::new(key_type),
        value_type: Box::new(value_type),
        value: slice.to_string(),
    })
}

pub fn create_list_field_type(loc: LOC, slice: &str, value_type: FieldType) -> FieldType {
    FieldType::ListType(FieldListType {
        loc,
        value_type: Box::new(value_type),
        value: slice.to_string(),
    })
}

pub fn create_set_field_type(loc: LOC, slice: &str, value_type: FieldType) -> FieldType {
    FieldType::SetType(FieldSetType {
        loc,
        value_type: Box::new(value_type),
        value: slice.to_string(),
    })
}

// for field value

pub fn create_const_value(token: &Token, loc: LOC, slice: &str) -> FieldInitialValue {
    FieldInitialValue::ConstValue(Common {
        kind: NodeType::from_token(token).unwrap(),
        loc,
        value: slice.to_string(),
    })
}

pub fn create_identifier_value(loc: LOC, slice: &str) -> FieldInitialValue {
    FieldInitialValue::ConstValue(Common {
        kind: NodeType::Identifier,
        loc,
        value: slice.to_string(),
    })
}

pub fn create_const_list_value(loc: LOC, elements: Vec<FieldInitialValue>) -> FieldInitialValue {
    FieldInitialValue::ConstList(ConstList { loc, elements })
}

pub fn create_map_value(loc: LOC, properties: Vec<MapProperty>) -> FieldInitialValue {
    FieldInitialValue::ConstMap(ConstMap { loc, properties })
}

pub(crate) fn create_identifier(loc: LOC, value: String) -> Common {
    Common {
        kind: NodeType::Identifier,
        value,
        loc,
    }
}

pub(crate) fn create_enum_member(
    loc: LOC,
    name: Common,
    initializer: Option<Initializer>,
    comments: Vec<Comment>,
    annotations: Option<Annotations>,
) -> EnumMember {
    EnumMember {
        loc,
        name,
        initializer,
        comments,
        annotations,
    }
}

pub(crate) fn create_initializer(loc: LOC, value: Common, kind: NodeType) -> Initializer {
    Initializer { kind, loc, value }
}

pub(crate) fn create_field_id(loc: LOC, value: String) -> Common {
    Common {
        kind: NodeType::FieldID,
        value,
        loc,
    }
}

pub fn create_void(loc: LOC, value: String) -> Common<String> {
    Common {
        kind: NodeType::VoidKeyword,
        value,
        loc,
    }
}
