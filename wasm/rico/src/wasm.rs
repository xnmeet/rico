use rico::{Parser as RicoParser, Writer as RicoWriter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Parser {
    input: String,
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Self {
        Self { input }
    }

    #[wasm_bindgen]
    pub fn parse(&mut self) -> Result<JsValue, JsError> {
        let mut parser = RicoParser::new(&self.input);
        match parser.parse() {
            Ok(ast) => serde_wasm_bindgen::to_value(&ast)
                .map_err(|e| JsError::new(&format!("Failed to serialize AST: {}", e))),
            Err(e) => Err(JsError::new(&format!("Parse error: {}", e))),
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
        Self {
            inner: RicoWriter::new(),
        }
    }

    #[wasm_bindgen]
    pub fn write(&mut self, ast: JsValue) -> Result<String, JsError> {
        let ast = serde_wasm_bindgen::from_value(ast)
            .map_err(|e| JsError::new(&format!("Failed to deserialize AST: {}", e)))?;
        Ok(self.inner.write(&ast))
    }
}
