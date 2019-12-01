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

mod writer {
    use js_sys::{Date, Map, Object};
    use parquetjs_sys as parquet;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn append_row() {
        let writer = JsFuture::from(crate::helper::open_file())
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

        let clo = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
        let fun = clo.as_ref().unchecked_ref();
        JsFuture::from(writer.close(&fun)).await.unwrap_throw();
        clo.forget();
    }
}
