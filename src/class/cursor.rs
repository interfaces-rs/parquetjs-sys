use crate::{
    class::{ParquetEnvelopeReader, ParquetSchema},
    interface::Metadata,
};
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "parquetjs")]
extern {
    #[wasm_bindgen(js_name = "ParquetCursor")]
    #[derive(Clone, Debug, PartialEq)]
    pub type ParquetCursor;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(
        metadata: &Metadata,
        envelope_reader: &ParquetEnvelopeReader,
        schema: &ParquetSchema,
        column_list: &Array,
    ) -> ParquetCursor;

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn next(this: &ParquetCursor) -> Promise;

    #[wasm_bindgen(method)]
    pub fn rewind(this: &ParquetCursor);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "columnList")]
    pub fn column_list(this: &ParquetCursor) -> Array;

    #[wasm_bindgen(method, getter, js_name = "envelopeReader")]
    pub fn envelope_reader(this: &ParquetCursor) -> ParquetEnvelopeReader;

    #[wasm_bindgen(method, getter)]
    pub fn metadata(this: &ParquetCursor) -> Metadata;

    #[wasm_bindgen(method, getter, js_name = "rowGroup")]
    pub fn row_group(this: &ParquetCursor) -> Array;

    #[wasm_bindgen(method, getter, js_name = "rowGroupIndex")]
    pub fn row_group_index(this: &ParquetCursor) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn schema(this: &ParquetCursor) -> ParquetSchema;
}
