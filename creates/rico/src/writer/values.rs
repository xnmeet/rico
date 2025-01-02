//! Value writing functionality for Thrift IDL.
//!
//! This module handles the writing of Thrift values and constants, including:
//! - Primitive values (integers, strings, etc.)
//! - List values
//! - Map values
//! - Constant definitions

use super::Writer;
use crate::ast::*;
use std::fmt::Write;

impl Writer {
    /// Writes a field value to the output string.
    /// Handles all Thrift value types including:
    /// - Constant values (integers, strings, etc.)
    /// - List values ([1, 2, 3])
    /// - Map values ({key: value})
    pub(crate) fn write_field_value(&mut self, output: &mut String, value: &FieldInitialValue) {
        match value {
            FieldInitialValue::ConstValue(v) => write!(output, "{}", v.value).unwrap(),
            FieldInitialValue::ConstList(list) => {
                write!(output, "[").unwrap();
                for (i, element) in list.elements.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    self.write_field_value(output, element);
                }
                write!(output, "]").unwrap();
            }
            FieldInitialValue::ConstMap(map) => {
                write!(output, "{{").unwrap();
                for (i, property) in map.properties.iter().enumerate() {
                    if i > 0 {
                        write!(output, ", ").unwrap();
                    }
                    self.write_field_value(output, &property.name);
                    write!(output, ": ").unwrap();
                    self.write_field_value(output, &property.value);
                }
                write!(output, "}}").unwrap();
            }
        }
    }
}
