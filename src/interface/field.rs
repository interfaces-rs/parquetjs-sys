use js_sys::{Array, JsString, Map, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen]
    #[derive(Clone, Debug, PartialEq)]
    pub type Field;

    // FIXME: are these readonly?

    #[wasm_bindgen(getter)]
    pub fn compression(this: &Field) -> Option<JsString>;

    #[wasm_bindgen(getter, js_name = "dLevelMax")]
    pub fn d_level_max(this: &Field) -> Option<Number>;

    #[wasm_bindgen(getter)]
    pub fn encoding(this: &Field) -> Option<JsString>;

    #[wasm_bindgen(getter, js_name = "fieldCount")]
    pub fn field_count(this: &Field) -> Option<usize>;

    #[wasm_bindgen(getter)]
    pub fn fields(this: &Field) -> Option<Map>;

    #[wasm_bindgen(getter, js_name = "isNested")]
    pub fn is_nested(this: &Field) -> Option<bool>;

    #[wasm_bindgen(getter)]
    pub fn name(this: &Field) -> JsString;

    #[wasm_bindgen(getter, js_name = "originalType")]
    pub fn original_type(this: &Field) -> Option<JsString>;

    #[wasm_bindgen(getter, js_name = "primitiveType")]
    pub fn primitive_type(this: &Field) -> Option<JsString>;

    #[wasm_bindgen(getter)]
    pub fn path(this: &Field) -> Array;

    #[wasm_bindgen(getter, js_name = "rLevelMax")]
    pub fn r_level_max(this: &Field) -> Option<Number>;

    #[wasm_bindgen(getter, js_name = "repetitionType")]
    pub fn repetition_type(this: &Field) -> JsString;

    #[wasm_bindgen(getter, js_name = "typeLength")]
    pub fn type_length(this: &Field) -> Option<usize>;
}
