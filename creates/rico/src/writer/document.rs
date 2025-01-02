//! Document member writing functionality for Thrift IDL.
//!
//! This module handles the writing of all major Thrift document members:
//! - Namespaces
//! - Includes
//! - Constants
//! - Typedefs
//! - Enums
//! - Structs
//! - Unions
//! - Exceptions
//! - Services and their functions

use super::Writer;
use crate::ast::*;
use std::fmt::Write;

impl Writer {
    /// Writes a namespace definition to the output string.
    pub(crate) fn write_namespace(&mut self, output: &mut String, ns: &Namespace) {
        self.write_comments(output, &ns.comments);
        writeln!(output, "namespace {} {}", ns.scope.value, ns.name.value).unwrap();
    }

    /// Writes an include statement to the output string.
    pub(crate) fn write_include(&mut self, output: &mut String, inc: &Include) {
        self.write_comments(output, &inc.comments);
        writeln!(output, "include \"{}\"", inc.name.value).unwrap();
    }

    /// Writes a constant definition to the output string.
    pub(crate) fn write_const(&mut self, output: &mut String, c: &Const) {
        self.write_comments(output, &c.comments);
        write!(output, "const ").unwrap();
        self.write_field_type(output, &c.field_type);
        write!(output, " {} = ", c.name.value).unwrap();
        self.write_field_value(output, &c.value);
        writeln!(output).unwrap();
    }

    /// Writes a typedef definition to the output string.
    pub(crate) fn write_typedef(&mut self, output: &mut String, td: &Typedef) {
        self.write_comments(output, &td.comments);
        write!(output, "typedef ").unwrap();
        self.write_field_type(output, &td.field_type);
        writeln!(output, " {}", td.name.value).unwrap();
    }

    /// Writes an enum definition to the output string.
    /// Handles enum members, their values, and annotations.
    pub(crate) fn write_enum(&mut self, output: &mut String, e: &Enum) {
        self.write_comments(output, &e.comments);
        writeln!(output, "enum {} {{", e.name.value).unwrap();
        self.indent();

        for member in &e.members {
            self.write_comments(output, &member.comments);
            self.write_indent(output);
            write!(output, "{}", member.name.value).unwrap();
            if let Some(value) = &member.initializer {
                write!(output, " = {}", value.value.value).unwrap();
            }
            self.write_annotations(output, &member.annotations);
            writeln!(output, ",").unwrap();
        }

        self.dedent();
        write!(output, "}}").unwrap();
        self.write_annotations(output, &e.annotations);
        writeln!(output).unwrap();
    }

    /// Writes a struct definition to the output string.
    /// Handles struct fields, their types, and annotations.
    pub(crate) fn write_struct(&mut self, output: &mut String, s: &Struct) {
        self.write_comments(output, &s.comments);
        writeln!(output, "struct {} {{", s.name.value).unwrap();
        self.indent();

        for member in &s.members {
            self.write_field(output, member);
        }

        self.dedent();
        write!(output, "}}").unwrap();
        self.write_annotations(output, &s.annotations);
        writeln!(output).unwrap();
    }

    /// Writes a union definition to the output string.
    /// Handles union fields and their annotations.
    pub(crate) fn write_union(&mut self, output: &mut String, u: &Union) {
        self.write_comments(output, &u.comments);
        writeln!(output, "union {} {{", u.name.value).unwrap();
        self.indent();

        for member in &u.members {
            self.write_field(output, member);
        }

        self.dedent();
        write!(output, "}}").unwrap();
        self.write_annotations(output, &u.annotations);
        writeln!(output).unwrap();
    }

    /// Writes an exception definition to the output string.
    /// Handles exception fields and their annotations.
    pub(crate) fn write_exception(&mut self, output: &mut String, e: &Exception) {
        self.write_comments(output, &e.comments);
        writeln!(output, "exception {} {{", e.name.value).unwrap();
        self.indent();

        for member in &e.members {
            self.write_field(output, member);
        }

        self.dedent();
        write!(output, "}}").unwrap();
        self.write_annotations(output, &e.annotations);
        writeln!(output).unwrap();
    }

    /// Writes a service definition to the output string.
    /// Handles service functions, inheritance, and annotations.
    pub(crate) fn write_service(&mut self, output: &mut String, s: &Service) {
        self.write_comments(output, &s.comments);
        write!(output, "service {}", s.name.value).unwrap();
        if let Some(extends) = &s.extends {
            write!(output, " extends {}", extends.value).unwrap();
        }
        writeln!(output, " {{").unwrap();
        self.indent();

        for member in &s.members {
            self.write_function(output, member);
        }

        self.dedent();
        write!(output, "}}").unwrap();
        self.write_annotations(output, &s.annotations);
        writeln!(output).unwrap();
    }

    /// Writes a function definition to the output string.
    /// Handles function parameters, return type, exceptions, and annotations.
    pub(crate) fn write_function(&mut self, output: &mut String, f: &Function) {
        self.write_comments(output, &f.comments);
        self.write_indent(output);

        if f.oneway {
            write!(output, "oneway ").unwrap();
        }

        self.write_field_type(output, &f.return_type);
        write!(output, " {}(", f.name.value).unwrap();

        self.indent();
        for (i, param) in f.params.iter().enumerate() {
            self.write_param(output, param, i == 0);
        }
        self.dedent();
        write!(output, ")").unwrap();

        if let Some(throws) = &f.throws {
            write!(output, " throws (").unwrap();
            self.indent();
            for (i, throw) in throws.iter().enumerate() {
                self.write_param(output, throw, i == 0);
            }
            self.dedent();
            write!(output, ")").unwrap();
        }

        self.write_annotations(output, &f.annotations);
        writeln!(output).unwrap();
    }
}
