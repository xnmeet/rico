use serde::{Deserialize, Serialize};

use super::types::{Common, NodeType, LOC};

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub kind: NodeType,
    pub value: String,
    pub loc: LOC,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub kind: NodeType,
    pub value: String,
    pub loc: LOC,
    pub name: Common<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    pub kind: NodeType,
    pub loc: LOC,
    pub members: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
    pub scope: Common<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Include {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldCollectionType {
    pub kind: NodeType,
    pub loc: LOC,
    pub value: String,
    #[serde(rename = "valueType")]
    pub value_type: Box<FieldType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldMapType {
    pub kind: NodeType,
    pub loc: LOC,
    pub value: String,
    #[serde(rename = "valueType")]
    pub value_type: Box<FieldType>,
    #[serde(rename = "keyType")]
    pub key_type: Box<FieldType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FieldType {
    // set or list
    CollectionType(FieldCollectionType),
    MapType(FieldMapType),
    CommonType(Common<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstList {
    pub kind: NodeType,
    pub loc: LOC,
    pub elements: Vec<FieldInitialValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapProperty {
    pub kind: NodeType,
    pub loc: LOC,
    pub value: FieldInitialValue,
    pub name: FieldInitialValue,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ConstMap {
    pub kind: NodeType,
    pub loc: LOC,
    pub properties: Vec<MapProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FieldInitialValue {
    ConstValue(Common<String>),
    ConstList(ConstList),
    ConstMap(ConstMap),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Const {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
    pub value: FieldInitialValue,
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Typedef {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DocumentMembers {
    Namespace(Namespace),
    Include(Include),
    Const(Const),
    Typedef(Typedef),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub kind: NodeType,
    pub members: Vec<DocumentMembers>,
}
