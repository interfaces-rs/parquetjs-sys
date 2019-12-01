use crate::{
    class::Schema,
    interface::{RowBuffer, WriterOptions},
};
use js_sys::{Function, Map, Promise};
use node_sys::{Buffer, WriteStream};
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
        schema: &Schema,
        write_fn: &Function,
        close_fn: &Function,
        file_offset: usize,
        opts: WriterOptions,
    ) -> EnvelopeWriter;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = EnvelopeWriter, js_name = "openStream")]
    pub fn open_stream(schema: &Schema, output_stream: &WriteStream, opts: WriterOptions) -> EnvelopeWriter;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn set_page_size(this: &EnvelopeWriter, count: usize);

    #[wasm_bindgen(method, js_name = "writeFooter")]
    pub fn write_footer(this: &EnvelopeWriter, user_metadata: &Map) -> Promise;

    #[wasm_bindgen(method, js_name = "writeHeader")]
    pub fn write_header(this: &EnvelopeWriter) -> Promise;

    #[wasm_bindgen(method, js_name = "writeRowGroup")]
    pub fn write_row_group(this: &EnvelopeWriter, records: &RowBuffer) -> Promise;

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn write_section(this: &EnvelopeWriter, buf: &Buffer);
}
