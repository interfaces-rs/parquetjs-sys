use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetEnvelopeWriter")]
    #[derive(Clone, Debug, PartialEq)]
    pub type EnvelopeWriter;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(
        schema: &JsValue,
        write_fn: &JsValue,
        close_fn: &JsValue,
        file_offset: &JsValue,
        opts: &JsValue,
    ) -> EnvelopeWriter;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = EnvelopeWriter, js_name = "openStream")]
    pub fn open_stream(schema: &JsValue, output_stream: &JsValue, opts: &JsValue) -> EnvelopeWriter;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn set_page_size(this: &EnvelopeWriter, count: usize);

    #[wasm_bindgen(method, js_name = "writeFooter")]
    pub fn write_footer(this: &EnvelopeWriter, user_metadata: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeHeader")]
    pub fn write_header(this: &EnvelopeWriter) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeRowGroup")]
    pub fn write_row_group(this: &EnvelopeWriter, records: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn write_section(this: &EnvelopeWriter, buf: &JsValue);
}
