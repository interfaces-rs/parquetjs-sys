use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetCursor")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Cursor;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(metadata: &JsValue, envelope_reader: &JsValue, schema: &JsValue, column_list: &JsValue) -> Cursor;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn next(this: &Cursor) -> Promise;

    #[wasm_bindgen(method)]
    pub fn rewind(this: &Cursor);
}
