use js_sys::Map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Map)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Row;
}
