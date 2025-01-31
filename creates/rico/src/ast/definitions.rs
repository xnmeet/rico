use serde::{Deserialize, Serialize};

use super::types::{Common, NodeType, LOC};

/// Represents a comment in the Thrift IDL.
///
/// Comments can be either single-line (//) or multi-line (/* */),
/// and are preserved in the AST for documentation purposes.
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    /// The type of the node (CommentLine or CommentBlock)
    pub kind: NodeType,
    /// The text content of the comment
    pub value: String,
    /// The location of the comment in the source code
    pub loc: LOC,
}

/// Represents a single annotation in the Thrift IDL.
///
/// Annotations provide metadata for Thrift definitions and can be used
/// to customize code generation or add runtime behavior.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always Annotation)
#[serde(tag = "kind", rename = "Annotation")]
pub struct Annotation {
    /// The value of the annotation
    pub value: Common<String>,
    /// The location of the annotation in the source code
    pub loc: LOC,
    /// The name of the annotation
    pub name: Common<String>,
}

/// A collection of annotations attached to a Thrift definition.
///
/// Multiple annotations can be specified in parentheses after a definition.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always Annotations)
#[serde(tag = "kind", rename = "Annotations")]
pub struct Annotations {
    /// The location of the annotations block in the source code
    pub loc: LOC,
    /// The list of individual annotations
    pub members: Vec<Annotation>,
}

/// Represents a collection type (list or set) in a field definition.
///
/// Collection types can hold multiple values of the same type.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always ListType)
#[serde(tag = "kind", rename = "ListType")]
pub struct FieldListType {
    /// The location in the source code
    pub loc: LOC,
    /// The type name ("list")
    pub value: String,
    /// The type of elements in the collection
    #[serde(rename = "valueType")]
    pub value_type: Box<FieldType>,
}

/// Represents a collection type (list or set) in a field definition.
///
/// Collection types can hold multiple values of the same type.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always SetType)
#[serde(tag = "kind", rename = "SetType")]
pub struct FieldSetType {
    /// The location in the source code
    pub loc: LOC,
    /// The type name ("set")
    pub value: String,
    /// The type of elements in the collection
    #[serde(rename = "valueType")]
    pub value_type: Box<FieldType>,
}

/// Represents a map type in a field definition.
///
/// Map types associate keys with values, where both key and value
/// types can be specified.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always MapType)
#[serde(tag = "kind", rename = "MapType")]
pub struct FieldMapType {
    /// The location in the source code
    pub loc: LOC,
    /// The type name ("map")
    pub value: String,
    /// The type of values in the map
    #[serde(rename = "valueType")]
    pub value_type: Box<FieldType>,
    /// The type of keys in the map
    #[serde(rename = "keyType")]
    pub key_type: Box<FieldType>,
}

/// Represents a list of constant values.
///
/// Used for list literals in constant definitions and default values.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always ConstList)
#[serde(tag = "kind", rename = "ConstList")]
pub struct ConstList {
    /// The location in the source code
    pub loc: LOC,
    /// The list elements
    pub elements: Vec<FieldInitialValue>,
}

/// Represents a key-value pair in a map constant.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always PropertyAssignment)
#[serde(tag = "kind", rename = "PropertyAssignment")]
pub struct MapProperty {
    /// The location in the source code
    pub loc: LOC,
    /// The property value
    pub value: FieldInitialValue,
    /// The property key
    pub name: FieldInitialValue,
}

/// Represents a map of constant values.
///
/// Used for map literals in constant definitions and default values.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always ConstMap)
#[serde(tag = "kind", rename = "ConstMap")]
pub struct ConstMap {
    /// The location in the source code
    pub loc: LOC,
    /// The map entries
    pub properties: Vec<MapProperty>,
}

/// Represents an initial value for a field or constant.
///
/// Can be a simple value, a list, or a map.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// enum will deserialize by order,so we should put the most complex type first
pub enum FieldInitialValue {
    /// A map of key-value pairs
    ConstMap(ConstMap),
    /// A list of values
    ConstList(ConstList),
    /// A simple constant value
    ConstValue(Common<String>),
}

/// Represents a field type in the Thrift IDL.
///
/// Field types can be:
/// - Base types (i32, string, etc.)
/// - Collections (list, set)
/// - Maps
/// - User-defined types
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FieldType {
    /// A map type
    MapType(FieldMapType),
    /// A list type
    ListType(FieldListType),
    /// A set type
    SetType(FieldSetType),
    /// A base type or user-defined type
    CommonType(Common<String>),
}

/// Represents an initializer for an enum value.
///
/// Enum values can optionally be assigned explicit integer values.
#[derive(Serialize, Deserialize, Debug)]
pub struct Initializer {
    pub kind: NodeType,
    /// The explicit value assigned to the enum member
    pub value: Common<String>,
    /// The location in the source code
    pub loc: LOC,
}

/// Represents a member of an enum definition.
///
/// Each enum member can have an optional explicit value and annotations.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always EnumMember)
#[serde(tag = "kind", rename = "EnumMember")]
pub struct EnumMember {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the enum member
    pub name: Common<String>,
    /// Optional explicit value assignment
    pub initializer: Option<Initializer>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents a field in a struct, union, exception, or function parameter.
///
/// Fields have an optional field ID, type, and various modifiers.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always FieldDefinition)
#[serde(tag = "kind", rename = "FieldDefinition")]
pub struct Field {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the field
    pub name: Common<String>,
    /// Optional field ID (1, 2, etc.)
    #[serde(rename = "fieldID")]
    pub field_id: Option<Common<String>>,
    /// The type of the field
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
    /// Required/Optional/Default modifier
    #[serde(rename = "requiredType")]
    pub required_type: String,
    /// Optional default value
    #[serde(rename = "defaultValue")]
    pub default_value: Option<FieldInitialValue>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
    /// Associated comments
    pub comments: Vec<Comment>,
}

/// Represents a function definition in a service.
///
/// Functions define the methods that can be called on a service.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always FunctionDefinition)
#[serde(tag = "kind", rename = "FunctionDefinition")]
pub struct Function {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the function
    pub name: Common<String>,
    /// The return type of the function
    #[serde(rename = "returnType")]
    pub return_type: FieldType,
    /// The function parameters
    pub params: Vec<Field>,
    /// Optional exceptions that can be thrown
    pub throws: Option<Vec<Field>>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Whether the function is oneway
    pub oneway: bool,
}

/// Represents a namespace declaration in the Thrift IDL.
///
/// Namespaces specify the package/module name for generated code in
/// different target languages.
#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    /// The location of the namespace declaration in the source code
    pub loc: LOC,
    /// The namespace identifier
    pub name: Common<String>,
    /// The target language scope (e.g., "py", "java", "rs")
    pub scope: Common<String>,
    /// Associated comments
    pub comments: Vec<Comment>,
}

/// Represents an include statement in the Thrift IDL.
///
/// Include statements allow splitting Thrift definitions across multiple
/// files for better organization.
#[derive(Serialize, Deserialize, Debug)]
pub struct Include {
    /// The location of the include statement in the source code
    pub loc: LOC,
    /// The name/path of the included file
    pub name: Common<String>,
    /// Associated comments
    pub comments: Vec<Comment>,
}

/// Represents a constant definition in the Thrift IDL.
///
/// Constants can be used to define shared values of any type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Const {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the constant
    pub name: Common<String>,
    /// The value of the constant
    pub value: FieldInitialValue,
    /// The type of the constant
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
    /// Associated comments
    pub comments: Vec<Comment>,
}

/// Represents a typedef definition in the Thrift IDL.
///
/// Typedefs create aliases for existing types, which can be used
/// to provide more meaningful names or documentation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Typedef {
    /// The location in the source code
    pub loc: LOC,
    /// The new type name
    pub name: Common<String>,
    /// The original type being aliased
    #[serde(rename = "fieldType")]
    pub field_type: FieldType,
    /// Associated comments
    pub comments: Vec<Comment>,
}

/// Represents an enum definition in the Thrift IDL.
///
/// Enums define a set of named constants that can be used as field types.
#[derive(Serialize, Deserialize, Debug)]
pub struct Enum {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the enum
    pub name: Common<String>,
    /// The enum members
    pub members: Vec<EnumMember>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents an exception definition in the Thrift IDL.
///
/// Exceptions are similar to structs but are used for error handling
/// in service methods.
#[derive(Serialize, Deserialize, Debug)]
pub struct Exception {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the exception
    pub name: Common<String>,
    /// The fields of the exception
    pub members: Vec<Field>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents a struct definition in the Thrift IDL.
///
/// Structs are the primary way to define complex data types in Thrift.
#[derive(Serialize, Deserialize, Debug)]
pub struct Struct {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the struct
    pub name: Common<String>,
    /// The fields of the struct
    pub members: Vec<Field>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents a union definition in the Thrift IDL.
///
/// Unions are similar to structs but only one field can be set at a time.
#[derive(Serialize, Deserialize, Debug)]
pub struct Union {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the union
    pub name: Common<String>,
    /// The fields of the union
    pub members: Vec<Field>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents a service definition in the Thrift IDL.
///
/// Services define interfaces that can be implemented by servers
/// and called by clients.
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    /// The location in the source code
    pub loc: LOC,
    /// The name of the service
    pub name: Common<String>,
    /// Optional parent service (for inheritance)
    pub extends: Option<Common<String>>,
    /// The functions defined in the service
    pub members: Vec<Function>,
    /// Associated comments
    pub comments: Vec<Comment>,
    /// Optional annotations
    pub annotations: Option<Annotations>,
}

/// Represents a complete Thrift IDL document.
///
/// A document is the root node of the AST and contains all the
/// top-level definitions.
#[derive(Serialize, Deserialize, Debug)]
/// The type of the node (always ThriftDocument)
#[serde(tag = "kind", rename = "ThriftDocument")]
pub struct Document {
    /// The list of top-level definitions
    pub members: Vec<DocumentMembers>,
}

/// Represents a top-level definition in a Thrift document.
///
/// Each member can be one of several types of definitions that are
/// allowed at the document level.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum DocumentMembers {
    /// A namespace declaration
    #[serde(rename = "NamespaceDefinition")]
    Namespace(Namespace),
    /// An include statement
    #[serde(rename = "IncludeDefinition")]
    Include(Include),
    /// A constant definition
    #[serde(rename = "ConstDefinition")]
    Const(Const),
    /// A typedef definition
    #[serde(rename = "TypedefDefinition")]
    Typedef(Typedef),
    /// An enum definition
    #[serde(rename = "EnumDefinition")]
    Enum(Enum),
    /// A struct definition
    #[serde(rename = "StructDefinition")]
    Struct(Struct),
    /// A service definition
    #[serde(rename = "ServiceDefinition")]
    Service(Service),
    /// An exception definition
    #[serde(rename = "ExceptionDefinition")]
    Exception(Exception),
    /// A union definition
    #[serde(rename = "UnionDefinition")]
    Union(Union),
}
