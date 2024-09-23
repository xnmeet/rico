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
pub struct FiledType {
    pub kind: NodeType,
    pub loc: LOC,
    pub value: String,
    #[serde(rename = "keyType")]
    pub key_type: Option<Common<String>>,
    #[serde(rename = "valueType")]
    pub value_type: Option<Common<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Const {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
    pub value: Common<String>,
    #[serde(rename = "fieldType")]
    pub field_type: FiledType,
}

#[derive(Serialize, Deserialize, Debug)]
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
