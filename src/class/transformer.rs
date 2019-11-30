use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetTransformer")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Transformer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue, opts: &JsValue) -> Transformer;
}
