use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetSchema")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Schema;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue) -> Schema;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "findField")]
    pub fn find_field(this: &Schema, path: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "findFieldBranch")]
    pub fn find_field_branch(this: &Schema, path: &JsString) -> JsValue;
}
