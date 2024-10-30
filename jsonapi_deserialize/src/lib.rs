mod deserialize;
mod document;
mod included;
mod link;

pub use deserialize::{deserialize_document, Error, JsonApiDeserialize};
pub use document::{
    Document, DocumentLinks, RawMultipleRelationship, RawOptionalRelationship,
    RawSingleRelationship, Reference,
};
pub use included::IncludedMap;
pub use link::Link;

extern crate jsonapi_deserialize_derive;
pub use jsonapi_deserialize_derive::JsonApiDeserialize;
