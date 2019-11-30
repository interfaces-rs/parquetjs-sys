use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetSchema")]
    #[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn find_field(path: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "findFieldBranch")]
    pub fn find_field_branch(path: &JsString) -> JsValue;
}
