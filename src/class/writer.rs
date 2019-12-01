use crate::{
    class::{EnvelopeWriter, Schema},
    interface::{Row, WriterOptions},
};
use js_sys::{Function, JsString, Map, Promise};
use node_sys::WriteStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetWriter")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Writer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(schema: &Schema, envelope_writer: &EnvelopeWriter, opts: WriterOptions) -> Writer;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Writer, js_name = "openFile")]
    pub fn open_file(schema: &Schema, file_path: &JsString, opts: WriterOptions) -> Promise;

    #[wasm_bindgen(static_method_of = Writer, js_name = "openStream")]
    pub fn open_stream(schema: &Schema, output_stream: &WriteStream, opts: WriterOptions) -> Writer;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "appendRow")]
    pub fn append_row(this: &Writer, row: &Row) -> Promise;

    #[wasm_bindgen(method)]
    pub fn close(this: &Writer, callback: &Function) -> Promise;

    #[wasm_bindgen(method, js_name = "setMetadata")]
    pub fn set_metadata(this: &Writer, key: &JsValue, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setPageSize")]
    pub fn set_page_size(this: &Writer, count: usize);

    #[wasm_bindgen(method, js_name = "setRowGroupSize")]
    pub fn set_row_group_size(this: &Writer, count: usize);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn closed(this: &Writer) -> bool;

    #[wasm_bindgen(method, getter, js_name = "envelopeWriter")]
    pub fn envelope_writer(this: &Writer) -> EnvelopeWriter;

    #[wasm_bindgen(method, getter, js_name = "rowBuffer")]
    pub fn row_buffer(this: &Writer) -> Schema;

    #[wasm_bindgen(method, getter, js_name = "rowGroupSize")]
    pub fn row_group_size(this: &Writer) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &Writer) -> Schema;

    #[wasm_bindgen(method, getter, js_name = "userMetaData")]
    pub fn user_meta_data(this: &Writer) -> Map;
}
