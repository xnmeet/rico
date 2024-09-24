use crate::lexer::Token;

use super::{
    Common, ConstList, ConstMap, FieldCollectionType, FieldInitialValue, FieldMapType, FieldType,
    MapProperty, NodeType, LOC,
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
        kind: NodeType::MapType,
        loc,
        key_type: Box::new(key_type),
        value_type: Box::new(value_type),
        value: slice.to_string(),
    })
}

pub fn create_list_field_type(loc: LOC, slice: &str, value_type: FieldType) -> FieldType {
    FieldType::CollectionType(FieldCollectionType {
        kind: NodeType::ListType,
        loc,
        value_type: Box::new(value_type),
        value: slice.to_string(),
    })
}

pub fn create_set_field_type(loc: LOC, slice: &str, value_type: FieldType) -> FieldType {
    FieldType::CollectionType(FieldCollectionType {
        kind: NodeType::SetType,
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
    FieldInitialValue::ConstList(ConstList {
        kind: NodeType::ConstList,
        loc,
        elements,
    })
}

pub fn create_map_value(loc: LOC, properties: Vec<MapProperty>) -> FieldInitialValue {
    FieldInitialValue::ConstMap(ConstMap {
        kind: NodeType::ConstMap,
        loc,
        properties,
    })
}
