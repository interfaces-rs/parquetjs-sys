pub(crate) mod helper {
    use js_sys::{Map, Object};
    use parquetjs_sys as parquet;
    use wasm_bindgen::prelude::*;

    pub(crate) mod reader {
        use js_sys::Promise;
        use parquetjs_sys as parquet;

        #[must_use]
        pub(crate) fn open_file() -> Promise {
            let file_path = &"data/read/produce.parquet".into();
            parquet::ParquetReader::open_file(file_path)
        }
    }

    pub(crate) mod writer {
        use js_sys::Promise;
        use parquetjs_sys as parquet;

        #[must_use]
        pub(crate) fn open_file() -> Promise {
            let schema = crate::helper::schema();
            let file_path = &"data/write/produce.parquet".into();
            let opts = None;
            parquet::ParquetWriter::open_file(&schema, file_path, opts)
        }
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
    use parquetjs_sys as parquet;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn open_file() {
        if let Ok(value) = JsFuture::from(crate::helper::reader::open_file()).await {
            let reader = value.unchecked_into::<parquet::ParquetReader>();
            JsFuture::from(reader.close()).await.unwrap_throw();
        }
    }

    #[wasm_bindgen_test]
    async fn get_cursor() {
        if let Ok(value) = JsFuture::from(crate::helper::reader::open_file()).await {
            let reader = value.unchecked_into::<parquet::ParquetReader>();
            reader.get_cursor(None);
            JsFuture::from(reader.close()).await.unwrap_throw();
        }
    }

    #[wasm_bindgen_test]
    async fn iterate_cursor() {
        if let Ok(value) = JsFuture::from(crate::helper::reader::open_file()).await {
            let reader = value.unchecked_into::<parquet::ParquetReader>();
            let cursor = reader.get_cursor(None);
            while let Ok(value) = JsFuture::from(cursor.next()).await {
                if value.is_null() {
                    break;
                }
                value.unchecked_into::<parquet::Row>();
            }
            JsFuture::from(reader.close()).await.unwrap_throw();
        }
    }
}

mod schema {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn new() {
        crate::helper::schema();
    }
}

mod writer {
    use js_sys::{Date, Map, Object};
    use parquetjs_sys as parquet;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn append_row() {
        let writer = JsFuture::from(crate::helper::writer::open_file())
            .await
            .unwrap_throw()
            .unchecked_into::<parquet::ParquetWriter>();

        JsFuture::from(
            writer.append_row(
                &Object::from_entries(
                    &Map::new()
                        .set(&"name".into(), &"apples".into())
                        .set(&"quantity".into(), &10u32.into())
                        .set(&"price".into(), &2.5f32.into())
                        .set(&"date".into(), &Date::now().into())
                        .set(&"in_stock".into(), &true.into()),
                )
                .unwrap_throw(),
            ),
        )
        .await
        .unwrap_throw();

        JsFuture::from(
            writer.append_row(
                &Object::from_entries(
                    &Map::new()
                        .set(&"name".into(), &"oranges".into())
                        .set(&"quantity".into(), &10u32.into())
                        .set(&"price".into(), &2.5f32.into())
                        .set(&"date".into(), &Date::now().into())
                        .set(&"in_stock".into(), &true.into()),
                )
                .unwrap_throw(),
            ),
        )
        .await
        .unwrap_throw();

        JsFuture::from(writer.close(None)).await.unwrap_throw();
    }
}
