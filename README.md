# JSON-API 1.1 Deserialization

[![Release](https://github.com/DASPRiD/jsonapi-deserialize-rs/actions/workflows/release.yml/badge.svg)](https://github.com/DASPRiD/jsonapi-deserialize-rs/actions/workflows/release.yml)

A simple library to facilitate deserialization of [JSON-API](https://jsonapi.org/) responses into structs. In contrast
to existing libraries this one specifically focuses on consumption of payloads, rather than generation. The reasons for
this are manifold, but primarily is there a large contrast between consumption and generation.

When consuming payloads from an API, you will like receive several included resources several times. While JSON-API
does already take care of de-duplication, inlining those resources becomes tricky. On the consuming side you
additionally might only have a sparse set of data, which would not necessarily match a generating model.

## Usage

This library exports a single function `deserialize_json_api_document()`. Simply pass in a `6str` of your JSON and you
are good to go. The type you are deserializing to must implement the `JsonApiDeserialize` trait. For your convenience
the library also exports a derive macro with the same name.

When using the macro, you must have an `id` field on your struct with a `String` type. Any other fields are considered
either attributes or relationships.

All attributes in a struct must implement Serde's `Deserialize` trait.

### Resource type

By default, the resource type for a struct will be derived from the struct name, converted to snake_case. To override
this, you can annotate the struct the following way:

```rust
#[json_api(resource_type = "foo")]
struct Bar;
```

### Field renaming

Without further configuration, the library follows the JSON-API recommendation that all fields in JSON should be
camel-cased. You can change this behavior with the `rename_all` attribute, similar to Serde. You can choose between
`camel_case`, `pascal_case` and `snake_case`.

Additionally you can also rename individual fields with the `rename` attribute. 

### Relationships

Unless specified otherwise, a field is always an attribute. To specify a field as a relationship, set the relationship
attribute to one of the following values:

- `single`: a single resource or reference
- `optional`: an optional resource or reference
- `multiple`: one or more resources or references

References must be typed as one of the following three types:

- `Reference`
- `Option<Reference>`
- `Vec<Reference>`.

Resources which are included in the document must be typed as one of the following three types:

- `Arc<T>`
- `Option<Arc<T>>`
- `Vec<Arc<T>>`

The reason for the `Arc` is because the same resource can be shared across multiple relationships.

## Error handling

There are two possible failure cases when calling `deserialize_json_api_document()` which can result in an error:

- `Error::DeserializeError(DeserializeError)`: There was a syntactic error while parsing the document
- `Error::DocumentError(Vec<DocumentError>)`: The document contains errors instead of data

The first kind of error either means that your structs do not match what's returned or that the server generated
garbage. The second kind means that either there was a server error or that your request had errors. You can
distinguish this based on whether the HTTP response code was in the 4xx or 5xx range.

## Examples

Have a look at the tests in the [test_suite](./test_suite/tests) folder. Those are examples covering all current
use-cases.
