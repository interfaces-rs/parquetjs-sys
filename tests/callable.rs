#![allow(unused_imports)]

use js_sys::{Map, Object, Reflect};
use parquetjs_sys as parquet;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn schema() {
    let _produce = parquet::ParquetSchema::new(&{
        Map::new()
            .set(&"name".into(), &{
                let val = Object::new();
                Reflect::set(&val, &"type".into(), &"UTF8".into()).unwrap_throw();
                val.into()
            })
            .set(&"quantity".into(), &{
                let val = Object::new();
                Reflect::set(&val, &"type".into(), &"INT64".into()).unwrap_throw();
                val.into()
            })
            .set(&"price".into(), &{
                let val = Object::new();
                Reflect::set(&val, &"type".into(), &"DOUBLE".into()).unwrap_throw();
                val.into()
            })
            .set(&"date".into(), &{
                let val = Object::new();
                Reflect::set(&val, &"type".into(), &"TIMESTAMP_MILLIS".into()).unwrap_throw();
                val.into()
            })
            .set(&"in_stock".into(), &{
                let val = Object::new();
                Reflect::set(&val, &"type".into(), &"BOOLEAN".into()).unwrap_throw();
                val.into()
            })
    });
}
