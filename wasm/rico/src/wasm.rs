use rico::{Parser as RicoParser, Writer as RicoWriter};
use wasm_bindgen::prelude::*;

use crate::error::RicoError;
use crate::utils::set_panic_hook;
use serde_json;

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
        let mut parser = RicoParser::new(&self.input);
        match parser.parse() {
            Ok(ast) => {
                let value = serde_json::to_string(&ast)
                    .map_err(|e| RicoError::serialization(e).to_string());

                return value;
            }
            Err(e) => Err(RicoError::parse(e, &self.input).to_string()),
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
    pub fn write(&mut self, ast: &str) -> Result<String, JsValue> {
        let ast: rico::Document =
            serde_json::from_str(&ast).map_err(|e| RicoError::deserialization(e).to_string())?;

        let result = self.inner.write(&ast);

        Ok(result)
    }
}
