use super::types::{Common, NodeType, LOC};

#[derive(Debug)]
pub struct Comment {
    pub kind: NodeType,
    pub value: String,
    pub loc: LOC,
}

#[derive(Debug)]
pub struct Annotation {
    pub kind: NodeType,
    pub value: String,
    pub loc: LOC,
    pub name: Common<String>,
}

#[derive(Debug)]
pub struct Annotations {
    pub kind: NodeType,
    pub loc: LOC,
    pub members: Vec<Annotation>,
}

#[derive(Debug)]
pub struct Namespace {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
    pub scope: Common<String>,
}

#[derive(Debug)]
pub struct Include {
    pub kind: NodeType,
    pub loc: LOC,
    pub name: Common<String>,
}

#[derive(Debug)]
pub enum DocumentMembers {
    Namespace(Namespace),
    Include(Include),
}

#[derive(Debug)]
pub struct Document {
    pub kind: NodeType,
    pub members: Vec<DocumentMembers>,
}
