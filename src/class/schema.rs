use crate::interface;
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetSchema;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &interface::Schema) -> ParquetSchema;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "findField")]
    pub fn find_field(this: &ParquetSchema, path: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "findFieldBranch")]
    pub fn find_field_branch(this: &ParquetSchema, path: &JsString) -> JsValue;

    //*********************//
    // Instance Properties //
    //*********************//

    // FIXME: are these readonly?

    #[wasm_bindgen(method, getter)]
    pub fn field_list(this: &ParquetSchema) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn fields(this: &ParquetSchema) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &ParquetSchema) -> interface::Schema;
}
