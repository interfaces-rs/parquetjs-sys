mod helper {
    use js_sys::{Map, Object, Promise};
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
        parquet::ParquetSchema::new(
            &Object::from_entries(
                &Map::new()
                    .set(
                        &"name".into(),
                        &Object::from_entries(&Map::new().set(&"type".into(), &"UTF8".into())).unwrap_throw(),
                    )
                    .set(
                        &"quantity".into(),
                        &Object::from_entries(&Map::new().set(&"type".into(), &"INT64".into())).unwrap_throw(),
                    )
                    .set(
                        &"price".into(),
                        &Object::from_entries(&Map::new().set(&"type".into(), &"DOUBLE".into())).unwrap_throw(),
                    )
                    .set(
                        &"date".into(),
                        &Object::from_entries(&Map::new().set(&"type".into(), &"TIMESTAMP_MILLIS".into()))
                            .unwrap_throw(),
                    )
                    .set(
                        &"in_stock".into(),
                        &Object::from_entries(&Map::new().set(&"type".into(), &"BOOLEAN".into())).unwrap_throw(),
                    ),
            )
            .unwrap_throw(),
        )
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
