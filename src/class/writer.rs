use crate::{
    class::{ParquetEnvelopeWriter, ParquetSchema},
    interface::{Row, WriterOptions},
};
use js_sys::{Function, JsString, Object, Promise};
use node_sys::fs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetWriter")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetWriter;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &ParquetSchema, envelope_writer: &ParquetEnvelopeWriter, opts: WriterOptions) -> ParquetWriter;

    //****************//
    // Static Methods //
    //****************//

    #[must_use]
    #[wasm_bindgen(static_method_of = ParquetWriter, js_name = "openFile")]
    pub fn open_file(schema: &ParquetSchema, file_path: &JsString, opts: Option<WriterOptions>) -> Promise;

    #[wasm_bindgen(static_method_of = ParquetWriter, js_name = "openStream")]
    pub fn open_stream(
        schema: &ParquetSchema,
        output_stream: &fs::WriteStream,
        opts: Option<WriterOptions>,
    ) -> ParquetWriter;

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method, js_name = "appendRow")]
    pub fn append_row(this: &ParquetWriter, row: &Row) -> Promise;

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn close(this: &ParquetWriter, callback: Option<&Function>) -> Promise;

    #[wasm_bindgen(method, js_name = "setMetadata")]
    pub fn set_metadata(this: &ParquetWriter, key: &JsValue, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setPageSize")]
    pub fn set_page_size(this: &ParquetWriter, count: usize);

    #[wasm_bindgen(method, js_name = "setRowGroupSize")]
    pub fn set_row_group_size(this: &ParquetWriter, count: usize);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn closed(this: &ParquetWriter) -> bool;

    #[wasm_bindgen(method, getter, js_name = "envelopeWriter")]
    pub fn envelope_writer(this: &ParquetWriter) -> ParquetEnvelopeWriter;

    #[wasm_bindgen(method, getter, js_name = "rowBuffer")]
    pub fn row_buffer(this: &ParquetWriter) -> ParquetSchema;

    #[wasm_bindgen(method, getter, js_name = "rowGroupSize")]
    pub fn row_group_size(this: &ParquetWriter) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &ParquetWriter) -> ParquetSchema;

    #[wasm_bindgen(method, getter, js_name = "userMetaData")]
    pub fn user_meta_data(this: &ParquetWriter) -> Object;
}
