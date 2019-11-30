use crate::class::Cursor;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetReader")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Reader;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(metadata: &JsValue, envelope_reader: &JsValue) -> Reader;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Reader, js_name = "openFile")]
    pub fn open_file(file_path: &JsValue) -> Promise;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn close(this: &Reader) -> Promise;

    #[wasm_bindgen(method, js_name = "getCursor")]
    pub fn get_cursor(this: &Reader, column_list: &JsValue) -> Cursor;

    #[wasm_bindgen(method, js_name = "getMetadata")]
    pub fn get_metadata(this: &Reader) -> JsValue;

    #[wasm_bindgen(method, js_name = "getRowCount")]
    pub fn get_row_count(this: &Reader) -> usize;

    #[wasm_bindgen(method, js_name = "getSchema")]
    pub fn get_schema(this: &Reader) -> JsValue;
}
