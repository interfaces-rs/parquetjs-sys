mod helper {
    use js_sys::{Map, Object, Promise, Reflect};
    use parquetjs_sys as parquet;
    use wasm_bindgen::prelude::*;

    #[must_use]
    pub(crate) fn open_file() -> Promise {
        let schema = crate::helper::schema();
        let file_path = &"produce.parquet".into();
        let opts = None;
        parquet::ParquetWriter::open_file(&schema, file_path, opts)
    }

    pub(crate) fn schema() -> parquet::ParquetSchema {
        parquet::ParquetSchema::new(&{
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
        })
    }
}

mod reader {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn open_file() {
        JsFuture::from(crate::helper::open_file()).await.unwrap_throw();
    }
}

mod schema {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn new() {
        crate::helper::schema();
    }
}
