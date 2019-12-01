use crate::interface::{Row, RowBuffer, Schema};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Shredder;

    pub static shredder: Shredder;

    #[wasm_bindgen(method, js_name = "shredRecord")]
    pub fn shred_record(this: &Shredder, schema: &Schema, record: &Row, buffer: &RowBuffer);

    #[wasm_bindgen(method, js_name = "materializeRecords")]
    pub fn materialize_records(this: &Shredder, schema: &Schema, buffer: &RowBuffer);
}
