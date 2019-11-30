use crate::{
    class::{EnvelopeReader, Schema},
    interface::Metadata,
};
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "ParquetCursor")]
    #[derive(Clone, Debug, PartialEq)]
    pub type Cursor;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(metadata: &Metadata, envelope_reader: &EnvelopeReader, schema: &Schema, column_list: &Array) -> Cursor;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn next(this: &Cursor) -> Promise;

    #[wasm_bindgen(method)]
    pub fn rewind(this: &Cursor);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "columnList")]
    pub fn column_list(this: &Cursor) -> Array;

    #[wasm_bindgen(method, getter, js_name = "envelopeReader")]
    pub fn envelope_reader(this: &Cursor) -> EnvelopeReader;

    #[wasm_bindgen(method, getter)]
    pub fn metadata(this: &Cursor) -> Metadata;

    #[wasm_bindgen(method, getter, js_name = "rowGroup")]
    pub fn row_group(this: &Cursor) -> Array;

    #[wasm_bindgen(method, getter, js_name = "rowGroupIndex")]
    pub fn row_group_index(this: &Cursor) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &Cursor) -> Schema;
}
