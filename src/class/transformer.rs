use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetTransformer")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetTransformer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue, opts: &JsValue) -> ParquetTransformer;
}
