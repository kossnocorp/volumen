use volumen_parser::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPES_IMPORTS: &'static str = r#"
import type { ParseResult } from "@volumen/types";

export * from "@volumen/types";
"#;

#[wasm_bindgen(js_name = parsePrompts, unchecked_return_type = "ParseResult")]
pub fn parse_prompts(source: &str, filename: &str) -> Result<JsValue, JsValue> {
    let result = Parser::parse(source, filename);
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
