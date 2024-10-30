use crate::deserialize::JsonApiDeserialize;
use crate::link::Link;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub struct Document<T>
where
    T: JsonApiDeserialize,
{
    pub data: T,
    pub meta: Option<HashMap<String, Value>>,
    pub links: Option<DocumentLinks>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DocumentLinks {
    #[serde(rename = "self")]
    this: Option<Link>,
    related: Option<Link>,
    #[serde(rename = "describedby")]
    described_by: Option<Link>,
    first: Option<Link>,
    last: Option<Link>,
    prev: Option<Link>,
    next: Option<Link>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Reference {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawDocument {
    pub data: Value,
    pub meta: Option<HashMap<String, Value>>,
    pub links: Option<DocumentLinks>,
    pub included: Option<Vec<RawResource>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawResource {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub attributes: Option<Value>,
    pub relationships: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct RawSingleRelationship {
    pub data: Reference,
}

#[derive(Debug, Deserialize)]
pub struct RawOptionalRelationship {
    pub data: Option<Reference>,
}

#[derive(Debug, Deserialize)]
pub struct RawMultipleRelationship {
    pub data: Vec<Reference>,
}

impl<'a> From<&'a RawResource> for Value {
    fn from(resource: &'a RawResource) -> Self {
        let mut value = serde_json::json!({
            "type": resource.kind,
            "id": resource.id,
        });

        if let Some(attributes) = &resource.attributes {
            let attrs_value = serde_json::json!(attributes);
            value["attributes"] = attrs_value;
        }

        if let Some(relationships) = &resource.relationships {
            let rels_value = serde_json::json!(relationships);
            value["relationships"] = rels_value;
        }

        value
    }
}
