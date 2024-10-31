#![allow(unused)]

use jsonapi_deserialize::{deserialize_document, Document, JsonApiDeserialize};

#[derive(Debug, JsonApiDeserialize)]
#[json_api(rename_all = "snake_case")]
struct SnakeCase {
    id: String,
    foo_bar: String,
}

#[derive(Debug, JsonApiDeserialize)]
#[json_api(rename_all = "pascal_case")]
struct PascalCase {
    id: String,
    foo_bar: String,
}

#[derive(Debug, JsonApiDeserialize)]
#[json_api(rename_all = "camel_case")]
struct CamelCase {
    id: String,
    foo_bar: String,
}

#[derive(Debug, JsonApiDeserialize)]
struct FieldRename {
    pub id: String,
    #[json_api(rename = "foobar")]
    foo_bar: String,
}

#[test]
fn test_snake_case() {
    let document: Document<SnakeCase> = deserialize_document(
        r#"{
            "data": {
                "id": "1",
                "type": "snake_case",
                "attributes": {
                    "foo_bar": "Foo"
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(document.data.foo_bar, "Foo".to_string());
}

#[test]
fn test_pascal_case() {
    let document: Document<PascalCase> = deserialize_document(
        r#"{
            "data": {
                "id": "1",
                "type": "pascal_case",
                "attributes": {
                    "FooBar": "Foo"
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(document.data.foo_bar, "Foo".to_string());
}

#[test]
fn test_camel_case() {
    let document: Document<CamelCase> = deserialize_document(
        r#"{
            "data": {
                "id": "1",
                "type": "camel_case",
                "attributes": {
                    "fooBar": "Foo"
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(document.data.foo_bar, "Foo".to_string());
}

#[test]
fn test_field_rename() {
    let document: Document<FieldRename> = deserialize_document(
        r#"{
            "data": {
                "id": "1",
                "type": "field_rename",
                "attributes": {
                    "foobar": "Foo"
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(document.data.foo_bar, "Foo".to_string());
}
