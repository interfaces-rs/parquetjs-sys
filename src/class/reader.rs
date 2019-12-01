use crate::{
    class::{Cursor, EnvelopeReader, Schema},
    interface::Metadata,
};
use js_sys::{Array, JsString, Map, Promise};
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
    pub fn new(metadata: &Metadata, envelope_reader: &EnvelopeReader) -> Reader;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Reader, js_name = "openFile")]
    pub fn open_file(file_path: &JsString) -> Promise;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn close(this: &Reader) -> Promise;

    #[wasm_bindgen(method, js_name = "getCursor")]
    pub fn get_cursor(this: &Reader, column_list: &Array) -> Cursor;

    #[wasm_bindgen(method, js_name = "getMetadata")]
    pub fn get_metadata(this: &Reader) -> Map;

    #[wasm_bindgen(method, js_name = "getRowCount")]
    pub fn get_row_count(this: &Reader) -> usize;

    #[wasm_bindgen(method, js_name = "getSchema")]
    pub fn get_schema(this: &Reader) -> Schema;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn envelope_reader(this: &Reader) -> EnvelopeReader;

    #[wasm_bindgen(method, getter)]
    pub fn metadata(this: &Reader) -> Metadata;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &Reader) -> Schema;
}
