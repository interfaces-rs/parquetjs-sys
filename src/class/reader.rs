use crate::{
    class::{ParquetCursor, ParquetEnvelopeReader, ParquetSchema},
    interface::Metadata,
};
use js_sys::{Array, JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetReader")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetReader;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(metadata: &Metadata, envelope_reader: &ParquetEnvelopeReader) -> ParquetReader;

    //****************//
    // Static Methods //
    //****************//

    #[must_use]
    #[wasm_bindgen(static_method_of = ParquetReader, js_name = "openFile")]
    pub fn open_file(file_path: &JsString) -> Promise;

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn close(this: &ParquetReader) -> Promise;

    #[wasm_bindgen(method, js_name = "getCursor")]
    pub fn get_cursor(this: &ParquetReader, column_list: &Array) -> ParquetCursor;

    #[wasm_bindgen(method, js_name = "getMetadata")]
    pub fn get_metadata(this: &ParquetReader) -> Object;

    #[wasm_bindgen(method, js_name = "getRowCount")]
    pub fn get_row_count(this: &ParquetReader) -> usize;

    #[wasm_bindgen(method, js_name = "getSchema")]
    pub fn get_schema(this: &ParquetReader) -> ParquetSchema;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn envelope_reader(this: &ParquetReader) -> ParquetEnvelopeReader;

    #[wasm_bindgen(method, getter)]
    pub fn metadata(this: &ParquetReader) -> Metadata;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &ParquetReader) -> ParquetSchema;
}
