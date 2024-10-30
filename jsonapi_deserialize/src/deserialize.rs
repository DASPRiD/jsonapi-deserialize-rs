use crate::document::{Document, RawDocument};
use crate::included::IncludedMap;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid type")]
    InvalidType(&'static str),

    #[error("Missing ID")]
    MissingId,

    #[error("Missing attributes")]
    MissingAttributes,

    #[error("Missing relationships")]
    MissingRelationships,

    #[error("Missing field")]
    MissingField(&'static str),

    #[error("Missing resource")]
    MissingResource { kind: String, id: String },

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

pub trait JsonApiDeserialize: Sized {
    fn from_value(value: &Value, included: &mut IncludedMap) -> Result<Self, Error>;
}

impl<T> JsonApiDeserialize for Option<T>
where
    T: JsonApiDeserialize,
{
    fn from_value(value: &Value, included: &mut IncludedMap) -> Result<Self, Error> {
        if value.is_null() {
            return Ok(None);
        }

        T::from_value(value, included).map(Some)
    }
}

impl<T> JsonApiDeserialize for Vec<T>
where
    T: JsonApiDeserialize,
{
    fn from_value(value: &Value, included: &mut IncludedMap) -> Result<Self, Error> {
        value
            .as_array()
            .ok_or_else(|| Error::InvalidType("Expected an array"))?
            .iter()
            .map(|value| T::from_value(value, included))
            .collect()
    }
}

pub fn deserialize_document<T: JsonApiDeserialize>(json: &str) -> Result<Document<T>, Error> {
    let raw_document: RawDocument = serde_json::from_str(json)?;
    let mut included_map: IncludedMap = match raw_document.included {
        Some(ref resources) => resources.into(),
        None => Default::default(),
    };

    let data = T::from_value(&raw_document.data, &mut included_map)?;

    Ok(Document {
        data,
        meta: raw_document.meta,
        links: raw_document.links,
    })
}
