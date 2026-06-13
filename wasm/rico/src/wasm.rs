use rico::{Parser as RicoParser, Writer as RicoWriter};
use wasm_bindgen::prelude::*;

use crate::error::RicoError;
use crate::utils::set_panic_hook;
use serde_json;

fn parse_to_json(input: &str) -> Result<String, String> {
    let mut parser = RicoParser::new(input);
    parser
        .parse()
        .map_err(|e| RicoError::parse(e, input).to_string())
        .and_then(|ast| {
            serde_json::to_string(&ast).map_err(|e| RicoError::serialization(e).to_string())
        })
}

#[wasm_bindgen(js_name = parse)]
pub fn parse(input: &str) -> Result<String, String> {
    set_panic_hook();
    parse_to_json(input)
}

#[wasm_bindgen(js_name = write)]
pub fn write(ast: &str) -> Result<String, String> {
    set_panic_hook();
    let ast: rico::Document =
        serde_json::from_str(ast).map_err(|e| RicoError::deserialization(e).to_string())?;

    Ok(RicoWriter::new().write(&ast))
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
    pub fn parse(&mut self) -> Result<String, String> {
        parse_to_json(&self.input)
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
    pub fn write(&mut self, ast: &str) -> Result<String, String> {
        let ast: rico::Document =
            serde_json::from_str(&ast).map_err(|e| RicoError::deserialization(e).to_string())?;

        let result = self.inner.write(&ast);

        Ok(result)
    }
}
