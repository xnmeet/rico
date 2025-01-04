//! Type-related writing functionality for Thrift IDL.
//!
//! This module handles the writing of Thrift type definitions, including:
//! - Base types (i32, string, etc.)
//! - Container types (list, set)
//! - Map types
//! - User-defined types

use super::Writer;
use crate::ast::*;
use std::fmt::Write;

impl Writer {
    /// Writes a field type to the output string.
    /// Handles all Thrift type variations including:
    /// - Base types (i32, string, etc.)
    /// - Container types (list<T>, set<T>)
    /// - Map types (map<K,V>)
    /// - User-defined types
    pub(crate) fn write_field_type(&mut self, output: &mut String, field_type: &FieldType) {
        match field_type {
            FieldType::CommonType(t) => write!(output, "{}", t.value).unwrap(),
            FieldType::ListType(t) => {
                write!(output, "{}<", t.value).unwrap();
                self.write_field_type(output, &t.value_type);
                write!(output, ">").unwrap();
            }
            FieldType::SetType(t) => {
                write!(output, "{}<", t.value).unwrap();
                self.write_field_type(output, &t.value_type);
                write!(output, ">").unwrap();
            }
            FieldType::MapType(t) => {
                write!(output, "{}<", t.value).unwrap();
                self.write_field_type(output, &t.key_type);
                write!(output, ", ").unwrap();
                self.write_field_type(output, &t.value_type);
                write!(output, ">").unwrap();
            }
        }
    }
}
