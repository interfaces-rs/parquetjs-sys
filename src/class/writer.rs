
use crate::class::Cursor;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetWriter")]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Writer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue, envelope_writter: &JsValue, opts: &JsValue) -> Writer;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Writer, js_name = "openFile")]
    pub fn open_file(schema: &JsValue, file_path: &JsValue, opts: &JsValue) -> Promise;

    #[wasm_bindgen(static_method_of = Writer, js_name = "openStream")]
    pub fn open_stream(schema: &JsValue, output_stream: &JsValue, opts: &JsValue) -> Writer;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "appendRow")]
    pub fn append_row(this: &Writer, row: &JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn close(this: &Writer, callback: &Function) -> Promise;

    #[wasm_bindgen(method, js_name = "setMetadata")]
    pub fn set_metadata(this: &Writer, key: &JsValue, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setPageSize")]
    pub fn set_page_size(this: &Writer, count: usize);

    #[wasm_bindgen(method, js_name = "setRowGroupSize")]
    pub fn set_row_group_size(this: &Writer, count: usize);
}
