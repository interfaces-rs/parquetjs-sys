use js_sys::{Array, JsString, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    #[derive(Clone, Debug, PartialEq)]
    pub type Metadata;

    // FIXME: are these readonly?

    #[wasm_bindgen(getter, js_name = "createdBy")]
    pub fn created_by(this: &Metadata) -> JsString;

    #[wasm_bindgen(getter, js_name = "keyValueMetadata")]
    pub fn key_value_metadata(this: &Metadata) -> Array;

    #[wasm_bindgen(getter, js_name = "numRows")]
    pub fn num_rows(this: &Metadata) -> usize;

    #[wasm_bindgen(getter, js_name = "rowGroups")]
    pub fn row_groups(this: &Metadata) -> Array;

    #[wasm_bindgen(getter)]
    pub fn schema(this: &Metadata) -> Array;

    #[wasm_bindgen(getter)]
    pub fn version(this: &Metadata) -> Number;
}
