//! Common writing functionality for Thrift IDL elements.
//!
//! This module provides common writing operations that are used across different
//! Thrift elements, such as writing comments, annotations, fields, and parameters.

use super::Writer;
use crate::ast::*;
use std::fmt::Write;

impl Writer {
    /// Writes comments to the output string with proper indentation.
    /// Handles both single-line and block comments.
    pub(crate) fn write_comments(&mut self, output: &mut String, comments: &[Comment]) {
        for comment in comments {
            self.write_indent(output);
            writeln!(output, "{}", comment.value).unwrap();
        }
    }

    /// Writes annotations to the output string.
    /// Annotations are written in the format: (key = value, key2 = value2)
    pub(crate) fn write_annotations(
        &mut self,
        output: &mut String,
        annotations: &Option<Annotations>,
    ) {
        if let Some(annotations) = annotations {
            write!(output, " (").unwrap();
            for (i, annotation) in annotations.members.iter().enumerate() {
                if i > 0 {
                    write!(output, ", ").unwrap();
                }
                write!(
                    output,
                    "{} = {}",
                    annotation.name.value, annotation.value.value
                )
                .unwrap();
            }
            write!(output, ")").unwrap();
        }
    }

    /// Writes a field definition to the output string.
    /// Handles field ID, required/optional modifier, type, name, default value, and annotations.
    pub(crate) fn write_field(&mut self, output: &mut String, field: &Field) {
        self.write_comments(output, &field.comments);
        self.write_indent(output);

        if let Some(id) = &field.field_id {
            write!(output, "{}: ", id.value).unwrap();
        }

        match field.required_type.as_str() {
            "required" => write!(output, "required ").unwrap(),
            "optional" => write!(output, "optional ").unwrap(),
            _ => {}
        }

        self.write_field_type(output, &field.field_type);
        write!(output, " {}", field.name.value).unwrap();

        if let Some(default) = &field.default_value {
            write!(output, " = ").unwrap();
            self.write_field_value(output, default);
        }

        self.write_annotations(output, &field.annotations);
        writeln!(output, ",").unwrap();
    }

    /// Writes a parameter definition to the output string.
    /// Similar to write_field but with specific formatting for function parameters.
    pub(crate) fn write_param(&mut self, output: &mut String, field: &Field, is_first: bool) {
        if !is_first {
            write!(output, ", ").unwrap();
        }
        if let Some(id) = &field.field_id {
            write!(output, "{}: ", id.value).unwrap();
        }
        self.write_field_type(output, &field.field_type);
        write!(output, " {}", field.name.value).unwrap();
        if let Some(default) = &field.default_value {
            write!(output, " = ").unwrap();
            self.write_field_value(output, default);
        }
    }
}
