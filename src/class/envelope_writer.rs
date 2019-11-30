

use crate::class::Cursor;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetEnvelopeWriter")]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type EnvelopeWriter;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &JsValue, write_fn: &JsValue, close_fn: &JsValue, file_offset: &JsValue, opts: &JsValue) -> EnvelopeWriter;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Writer, js_name = "openStream")]
    pub fn open_stream(schema: &JsValue, output_stream: &JsValue, opts: &JsValue) -> EnvelopeWriter;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn set_page_size(this: &Writer, count: usize);

    #[wasm_bindgen(method, js_name = "writeFooter")]
    pub fn write_footer(this: &Writer, user_metadata: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeHeader")]
    pub fn write_header(this: &Writer) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeRowGroup")]
    pub fn write_row_group(this: &Writer, records: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn write_section(this: &Writer, buf: &JsValue);
}
