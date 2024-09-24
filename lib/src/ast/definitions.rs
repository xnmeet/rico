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
pub struct FieldType {
    pub kind: NodeType,
    pub loc: LOC,
    pub value: String,
    #[serde(rename = "keyType", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<Common<String>>,
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<Common<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstList {
    pub kind: NodeType,
    pub loc: LOC,
    pub elements: Vec<FieldInitialValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstMap {
    pub kind: NodeType,
    pub loc: LOC,
    pub properties: Vec<Common<FieldInitialValue>>,
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
#[serde(untagged)]
pub enum DocumentMembers {
    Namespace(Namespace),
    Include(Include),
    Const(Const),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub kind: NodeType,
    pub members: Vec<DocumentMembers>,
}
