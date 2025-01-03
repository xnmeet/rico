use std::error::Error;

use miette::{Diagnostic, LabeledSpan};
use rico::{Parser as RicoParser, Writer as RicoWriter};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[derive(Serialize)]
struct Location {
    line: usize,
    column: usize,
    length: usize,
    source_text: String,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum RicoError {
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
    fn serialization(e: impl Error) -> Self {
        Self::Serialization {
            message: format!("Failed to serialize AST: {}", e),
            code: "SERIALIZATION_ERROR".to_string(),
        }
    }

    fn deserialization(e: impl Error) -> Self {
        Self::Deserialization {
            message: format!("Failed to deserialize AST: {}", e),
            code: "DESERIALIZATION_ERROR".to_string(),
        }
    }

    fn parse(e: impl Diagnostic, source: &str) -> Self {
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

    fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
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

#[wasm_bindgen]
pub struct Parser {
    input: String,
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Self {
        set_panic_hook();
        Self { input }
    }

    #[wasm_bindgen]
    pub fn parse(&mut self) -> Result<JsValue, JsValue> {
        let mut parser = RicoParser::new(&self.input);
        match parser.parse() {
            Ok(ast) => serde_wasm_bindgen::to_value(&ast)
                .map_err(|e| RicoError::serialization(e).to_js_value()),
            Err(e) => Err(RicoError::parse(e, &self.input).to_js_value()),
        }
    }
}

#[wasm_bindgen]
pub struct Writer {
    inner: RicoWriter,
}

#[wasm_bindgen]
impl Writer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();
        Self {
            inner: RicoWriter::new(),
        }
    }

    #[wasm_bindgen]
    pub fn write(&mut self, ast: JsValue) -> Result<String, JsValue> {
        let ast = serde_wasm_bindgen::from_value(ast)
            .map_err(|e| RicoError::deserialization(e).to_js_value())?;
        Ok(self.inner.write(&ast))
    }
}
