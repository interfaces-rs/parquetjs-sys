

use crate::class::Cursor;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetTransformer")]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Transformer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue, opts: &JsValue) -> Transformer;
}
