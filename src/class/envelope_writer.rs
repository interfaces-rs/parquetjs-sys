use crate::{
    class::ParquetSchema,
    interface::{RowBuffer, WriterOptions},
};
use js_sys::{Function, Object, Promise};
use node_sys::{Buffer, WriteStream};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetEnvelopeWriter")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetEnvelopeWriter;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(
        schema: &ParquetSchema,
        write_fn: &Function,
        close_fn: &Function,
        file_offset: usize,
        opts: WriterOptions,
    ) -> ParquetEnvelopeWriter;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = ParquetEnvelopeWriter, js_name = "openStream")]
    pub fn open_stream(
        schema: &ParquetSchema,
        output_stream: &WriteStream,
        opts: WriterOptions,
    ) -> ParquetEnvelopeWriter;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn set_page_size(this: &ParquetEnvelopeWriter, count: usize);

    #[must_use]
    #[wasm_bindgen(method, js_name = "writeFooter")]
    pub fn write_footer(this: &ParquetEnvelopeWriter, user_metadata: &Object) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "writeHeader")]
    pub fn write_header(this: &ParquetEnvelopeWriter) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "writeRowGroup")]
    pub fn write_row_group(this: &ParquetEnvelopeWriter, records: &RowBuffer) -> Promise;

    #[wasm_bindgen(method, js_name = "writeSection")]
    pub fn write_section(this: &ParquetEnvelopeWriter, buf: &Buffer);
}
