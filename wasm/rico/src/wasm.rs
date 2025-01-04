use rico::{Parser as RicoParser, Writer as RicoWriter};
use wasm_bindgen::prelude::*;

use crate::error::RicoError;
use crate::utils::set_panic_hook;

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
        let ast: rico::Document = serde_wasm_bindgen::from_value(ast)
            .map_err(|e| RicoError::deserialization(e).to_js_value())?;

        Ok(self.inner.write(&ast))
    }
}
