use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetEnvelopeReader")]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type EnvelopeReader;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(read_fn: &JsValue, close_fn: &JsValue, file_size: &JsValue) -> EnvelopeReader;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = EnvelopeReader, js_name = "openFile")]
    pub fn open_file(file_path: &JsValue) -> Promise;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "readHeader")]
    pub fn read_header(this: &EnvelopeReader) -> Promise;

    #[wasm_bindgen(method, js_name = "readRowGroup")]
    pub fn read_row_group(this: &EnvelopeReader, schema: &JsValue, row_group: &JsValue, column_list: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = "readColumnChunk")]
    pub fn read_column_chunk(this: &EnvelopeReader, schema: &JsValue, col_chunk: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = "readFooter")]
    pub fn read_footer(this: &EnvelopeReader) -> Promise;
}
