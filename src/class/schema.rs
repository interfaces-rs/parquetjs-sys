use crate::interface;
use js_sys::{Array, JsString, Map};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetSchema")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Schema;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &interface::Schema) -> Schema;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "findField")]
    pub fn find_field(this: &Schema, path: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "findFieldBranch")]
    pub fn find_field_branch(this: &Schema, path: &JsString) -> JsValue;

    //*********************//
    // Instance Properties //
    //*********************//

    // FIXME: are these readonly?

    #[wasm_bindgen(getter)]
    pub fn field_list(this: &Schema) -> Array;

    #[wasm_bindgen(getter)]
    pub fn fields(this: &Schema) -> Map;

    #[wasm_bindgen(getter)]
    pub fn schema(this: &Schema) -> interface::Schema;
}
