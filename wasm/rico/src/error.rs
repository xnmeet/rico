use std::error::Error;

use miette::{Diagnostic, LabeledSpan};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct Location {
    line: usize,
    column: usize,
    length: usize,
    source_text: String,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub enum RicoError {
    #[serde(rename = "ParseError")]
    Parse {
        message: String,
        code: String,
        help: Option<String>,
        location: Option<Location>,
    },
    #[serde(rename = "SerializationError")]
    Serialization { message: String, code: String },
    #[serde(rename = "DeserializationError")]
    Deserialization { message: String, code: String },
}

impl RicoError {
    pub fn serialization(e: impl Error) -> Self {
        Self::Serialization {
            message: format!("Failed to serialize AST: {}", e),
            code: "SERIALIZATION_ERROR".to_string(),
        }
    }

    pub fn deserialization(e: impl Error) -> Self {
        Self::Deserialization {
            message: format!("Failed to deserialize AST: {}", e),
            code: "DESERIALIZATION_ERROR".to_string(),
        }
    }

    pub fn parse(e: impl Diagnostic, source: &str) -> Self {
        Self::Parse {
            message: e.to_string(),
            code: e
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "PARSE_ERROR".to_string()),
            help: e.help().map(|s| s.to_string()),
            location: get_error_location(&e, source),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn get_error_location(e: &impl Diagnostic, source: &str) -> Option<Location> {
    e.labels()
        .into_iter()
        .flatten()
        .next()
        .map(|label: LabeledSpan| calculate_location(label.offset(), label.len(), source))
}

fn calculate_location(offset: usize, length: usize, source: &str) -> Location {
    let lines: Vec<&str> = source.lines().collect();

    // Calculate line number and column by counting newlines
    let mut line = 1;
    let mut last_newline = 0;
    let mut pos = 0;
    for (i, c) in source[..offset].chars().enumerate() {
        if c == '\n' {
            line += 1;
            last_newline = i + 1;
        }
        pos = i;
    }
    let column = pos - last_newline + 1;

    Location {
        line,
        column,
        length,
        source_text: lines
            .get(line - 1)
            .map(|s| s.to_string())
            .unwrap_or_default(),
    }
}
