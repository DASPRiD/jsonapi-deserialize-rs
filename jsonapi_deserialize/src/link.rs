use http::{uri, Uri};
use langtag::LangTagBuf;
use serde::de::{self, MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid link URI")]
    UriError(#[from] uri::InvalidUri),
}

#[derive(Debug, Deserialize)]
#[serde(remote = "Self")]
pub struct Link {
    #[serde(deserialize_with = "deserialize_uri")]
    pub href: Uri,
    pub rel: Option<String>,
    #[serde(rename = "describedby")]
    pub described_by: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub href_lang: Option<LangTagBuf>,
    pub meta: Option<HashMap<String, Value>>,
}

impl FromStr for Link {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Link {
            href: Uri::from_str(s)?,
            rel: None,
            described_by: None,
            title: None,
            media_type: None,
            href_lang: None,
            meta: None,
        })
    }
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinkVisitor;

        impl<'de> Visitor<'de> for LinkVisitor {
            type Value = Link;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("string or object representing a Link")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Link::from_str(v).map_err(|error| {
                    de::Error::invalid_value(Unexpected::Str(&error.to_string()), &self)
                })
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Link::deserialize(de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(LinkVisitor)
    }
}

fn deserialize_uri<'de, D>(deserializer: D) -> Result<Uri, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: String = Deserialize::deserialize(deserializer)?;
    Uri::from_str(&raw).map_err(|error| {
        de::Error::invalid_value(Unexpected::Str(&error.to_string()), &"URI reference")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_from_string() {
        let link: Link = serde_json::from_str(r#""http://example.com""#).unwrap();
        assert_eq!(link.href, Uri::from_str("http://example.com").unwrap());
    }

    #[test]
    fn test_deserialize_from_object() {
        let link: Link = serde_json::from_str(r#"{"href": "http://example.com"}"#).unwrap();
        assert_eq!(link.href, Uri::from_str("http://example.com").unwrap());
    }

    #[test]
    fn test_deserialize_relative_href() {
        let link: Link = serde_json::from_str(r#"{"href": "/example"}"#).unwrap();
        assert_eq!(link.href, Uri::from_str("/example").unwrap());
    }

    #[test]
    fn test_missing_href() {
        let result: Result<Link, _> = serde_json::from_str(r#"{}"#);
        assert!(result.is_err());
    }
}
