use crate::{class::ParquetSchema, interface::MetadataRowGroups};
use js_sys::{Array, Function, JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetEnvelopeReader")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetEnvelopeReader;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(read_fn: &Function, close_fn: &Function, file_size: usize) -> ParquetEnvelopeReader;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = ParquetEnvelopeReader, js_name = "openFile")]
    pub fn open_file(file_path: &JsString) -> Promise;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "readHeader")]
    pub fn read_header(this: &ParquetEnvelopeReader) -> Promise;

    #[wasm_bindgen(method, js_name = "readRowGroup")]
    pub fn read_row_group(
        this: &ParquetEnvelopeReader,
        schema: &ParquetSchema,
        row_group: &MetadataRowGroups,
        column_list: &Array,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "readColumnChunk")]
    pub fn read_column_chunk(this: &ParquetEnvelopeReader, schema: &ParquetSchema, col_chunk: &Object) -> Promise;

    #[wasm_bindgen(method, js_name = "readFooter")]
    pub fn read_footer(this: &ParquetEnvelopeReader) -> Promise;
}
