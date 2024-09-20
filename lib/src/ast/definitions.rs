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
pub enum DocumentMembers {
    Namespace(Namespace),
    Include(Include),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub kind: NodeType,
    pub members: Vec<DocumentMembers>,
}
