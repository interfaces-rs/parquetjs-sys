use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    #[derive(Clone, Debug, PartialEq)]
    pub type RowBuffer;

    #[wasm_bindgen(method, getter, js_name = "column_data")]
    pub fn column_data(this: &RowBuffer) -> Object;

    #[wasm_bindgen(method, getter, js_name = "rowCount")]
    pub fn row_count(this: &RowBuffer) -> usize;
}
