//! Writer module for converting Thrift AST back to Thrift IDL format.
//!
//! This module provides functionality to write Thrift AST nodes back to their textual representation.
//! It handles proper indentation, formatting, and all Thrift IDL constructs including:
//! - Namespaces
//! - Includes
//! - Constants
//! - Typedefs
//! - Enums
//! - Structs
//! - Unions
//! - Exceptions
//! - Services
//!
//! The writer maintains proper indentation and formatting according to common Thrift IDL conventions.

mod common;
mod document;
mod types;
mod values;

use crate::ast::*;
use std::fmt::Write;

/// A writer that converts Thrift AST nodes back to Thrift IDL text format.
///
/// The writer handles proper indentation and formatting of all Thrift constructs.
/// It provides methods to write individual AST nodes as well as complete documents.
///
/// # Example
///
/// ```rust
/// use rico::writer::Writer;
/// use rico::ast::Document;
///
/// let document = Document {
///     members: vec![]
/// };
/// let mut writer = Writer::new();
/// let thrift_text = writer.write(&document);
/// ```
pub struct Writer {
    /// Current indentation level (each level represents 2 spaces)
    indent_level: usize,
}

impl Writer {
    /// Creates a new Writer instance with default settings.
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    /// Increases the current indentation level by 1 (2 spaces).
    fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decreases the current indentation level by 1 (2 spaces).
    /// Will not decrease below 0.
    fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Writes the current indentation to the output string.
    /// Each indentation level represents 2 spaces.
    fn write_indent(&mut self, output: &mut String) {
        write!(output, "{:width$}", "", width = self.indent_level * 2).unwrap();
    }

    /// Writes a complete Thrift document to string format.
    ///
    /// This method handles all document members in sequence, including:
    /// - Namespaces
    /// - Includes
    /// - Constants
    /// - Typedefs
    /// - Enums
    /// - Structs
    /// - Unions
    /// - Exceptions
    /// - Services
    ///
    /// Each member is properly formatted with comments, annotations, and indentation.
    ///
    /// # Arguments
    ///
    /// * `doc` - The Thrift document AST to write
    ///
    /// # Returns
    ///
    /// A string containing the formatted Thrift IDL text
    pub fn write(&mut self, doc: &Document) -> String {
        let mut output = String::new();

        for member in &doc.members {
            match member {
                DocumentMembers::Namespace(ns) => self.write_namespace(&mut output, ns),
                DocumentMembers::Include(inc) => self.write_include(&mut output, inc),
                DocumentMembers::Const(c) => self.write_const(&mut output, c),
                DocumentMembers::Typedef(td) => self.write_typedef(&mut output, td),
                DocumentMembers::Enum(e) => self.write_enum(&mut output, e),
                DocumentMembers::Struct(s) => self.write_struct(&mut output, s),
                DocumentMembers::Union(u) => self.write_union(&mut output, u),
                DocumentMembers::Exception(e) => self.write_exception(&mut output, e),
                DocumentMembers::Service(s) => self.write_service(&mut output, s),
            }
            writeln!(output).unwrap();
        }

        output
    }
}
