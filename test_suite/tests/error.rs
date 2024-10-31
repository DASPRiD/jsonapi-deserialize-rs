#![allow(unused)]

use jsonapi_deserialize::{deserialize_document, Document, Error, JsonApiDeserialize};

#[derive(Debug, JsonApiDeserialize)]
struct Foo {
    id: String,
}

#[test]
fn test_document_errors() {
    let result: Result<Document<Foo>, Error> = deserialize_document(
        r#"{
            "errors": [{
                "status": "404"
            }]
        }"#,
    );

    let errors = if let Err(Error::DocumentError(errors)) = result {
        errors
    } else {
        panic!("Expected DocumentError, but got {:?}", result);
    };

    assert_eq!(errors.get(0).unwrap().status, Some("404".to_string()));
}
