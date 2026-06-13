export type NodeType =
  | 'ThriftDocument'
  | 'ThriftErrors'
  | 'Identifier'
  | 'FieldID'
  | 'NamespaceDefinition'
  | 'IncludeDefinition'
  | 'CppIncludeDefinition'
  | 'ConstDefinition'
  | 'TypedefDefinition'
  | 'EnumDefinition'
  | 'StructDefinition'
  | 'UnionDefinition'
  | 'ExceptionDefinition'
  | 'ServiceDefinition'
  | 'FunctionDefinition'
  | 'FieldDefinition'
  | 'CommentLine'
  | 'CommentBlock'
  | 'Annotation'
  | 'Annotations'
  | 'FieldType'
  | 'BaseType'
  | 'SetType'
  | 'ListType'
  | 'MapType'
  | 'ConstValue'
  | 'IntConstant'
  | 'DoubleConstant'
  | 'ConstList'
  | 'ConstMap'
  | 'StringLiteral'
  | 'IntegerLiteral'
  | 'FloatLiteral'
  | 'HexLiteral'
  | 'ExponentialLiteral'
  | 'BooleanLiteral'
  | 'PropertyAssignment'
  | 'NamespaceKeyword'
  | 'IncludeKeyword'
  | 'CppIncludeKeyword'
  | 'ExceptionKeyword'
  | 'ServiceKeyword'
  | 'ExtendsKeyword'
  | 'RequiredKeyword'
  | 'OptionalKeyword'
  | 'FalseKeyword'
  | 'TrueKeyword'
  | 'ConstKeyword'
  | 'DoubleKeyword'
  | 'StructKeyword'
  | 'TypedefKeyword'
  | 'UnionKeyword'
  | 'StringKeyword'
  | 'BinaryKeyword'
  | 'BoolKeyword'
  | 'ByteKeyword'
  | 'EnumKeyword'
  | 'ListKeyword'
  | 'SetKeyword'
  | 'MapKeyword'
  | 'I8Keyword'
  | 'I16Keyword'
  | 'I32Keyword'
  | 'I64Keyword'
  | 'ThrowsKeyword'
  | 'VoidKeyword'
  | 'OnewayKeyword'
  | 'EOF';

export interface Span {
  line: number;
  column: number;
  index: number;
}

export interface LOC {
  start: Span;
  end: Span;
}

export interface Common<T = string> {
  kind: NodeType;
  value: T;
  loc: LOC;
}

export interface Document {
  kind: 'ThriftDocument';
  members: DocumentMember[];
}

export type DocumentMember =
  | Namespace
  | Include
  | CppInclude
  | Const
  | Typedef
  | Enum
  | Struct
  | Union
  | Exception
  | Service;

export interface BaseNode {
  kind: NodeType;
  loc: LOC;
  comments: Comment[];
  annotations?: Annotations;
}

export interface Comment {
  kind: 'CommentLine' | 'CommentBlock';
  value: string;
  loc: LOC;
}

export interface Annotation {
  kind: 'Annotation';
  name: Common<string>;
  value: Common<string>;
  loc: LOC;
}

export interface Annotations {
  kind: 'Annotations';
  loc: LOC;
  members: Annotation[];
}

export interface Namespace extends BaseNode {
  kind: 'NamespaceDefinition';
  scope: Common<string>;
  name: Common<string>;
}

export interface Include extends BaseNode {
  kind: 'IncludeDefinition';
  name: Common<string>;
}

export interface CppInclude extends BaseNode {
  kind: 'CppIncludeDefinition';
  name: Common<string>;
}

export interface Const extends BaseNode {
  kind: 'ConstDefinition';
  name: Common<string>;
  fieldType: FieldType;
  value: FieldValue;
}

export interface Typedef extends BaseNode {
  kind: 'TypedefDefinition';
  name: Common<string>;
  fieldType: FieldType;
}

export interface Initializer {
  kind: NodeType;
  value: Common<string>;
  loc: LOC;
}

export interface EnumMember extends BaseNode {
  kind: NodeType;
  name: Common<string>;
  initializer?: Initializer;
}

export interface Enum extends BaseNode {
  kind: 'EnumDefinition';
  name: Common<string>;
  members: EnumMember[];
}

export interface Field extends BaseNode {
  kind: 'FieldDefinition';
  name: Common<string>;
  fieldID?: Common<string>;
  fieldType: FieldType;
  requiredType: string;
  defaultValue?: FieldValue;
}

export interface Struct extends BaseNode {
  kind: 'StructDefinition';
  name: Common<string>;
  members: Field[];
}

export interface Union extends BaseNode {
  kind: 'UnionDefinition';
  name: Common<string>;
  members: Field[];
}

export interface Exception extends BaseNode {
  kind: 'ExceptionDefinition';
  name: Common<string>;
  members: Field[];
}

export interface Function extends BaseNode {
  kind: 'FunctionDefinition';
  name: Common<string>;
  returnType: FieldType;
  params: Field[];
  throws?: Field[];
  oneway: boolean;
}

export interface Service extends BaseNode {
  kind: 'ServiceDefinition';
  name: Common<string>;
  extends?: Common<string>;
  members: Function[];
}

export interface FieldListType {
  kind: 'ListType';
  loc: LOC;
  value: string;
  valueType: FieldType;
}

export interface FieldSetType {
  kind: 'SetType';
  loc: LOC;
  value: string;
  valueType: FieldType;
}

export interface FieldMapType {
  kind: 'MapType';
  loc: LOC;
  value: string;
  valueType: FieldType;
  keyType: FieldType;
}

export type FieldType =
  | Common<string>
  | FieldListType
  | FieldSetType
  | FieldMapType;

export interface ConstList {
  kind: 'ConstList';
  loc: LOC;
  elements: FieldValue[];
}

export interface MapProperty {
  kind: 'PropertyAssignment';
  loc: LOC;
  name: FieldValue;
  value: FieldValue;
}

export interface ConstMap {
  kind: 'ConstMap';
  loc: LOC;
  properties: MapProperty[];
}

export type FieldValue =
  | Common<string | number | boolean>
  | ConstList
  | ConstMap;

export interface ParseError {
  kind: 'ParseError' | 'SerializationError' | 'DeserializationError';
  message: string;
  code: string;
  help?: string;
  location?: {
    line: number;
    column: number;
    length: number;
    sourceText: string;
  };
}
