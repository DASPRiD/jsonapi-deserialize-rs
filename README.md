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

### Resource type

By default, the resource type for a struct will be derived from the struct name, converted to snake_case. To override
this, you can annotate the struct the following way:

```rust
#[json_api(resource_type = "foo")]
struct Bar;
```

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

## Examples

Have a look at the tests in the [test_suite](./test_suite/tests) folder. Those are examples covering all current
use-cases.
