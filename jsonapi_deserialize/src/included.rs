use crate::deserialize::{Error, JsonApiDeserialize};
use crate::document::RawResource;
use serde_json::Value;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

struct Handle<T: ?Sized + Send + Sync + 'static>(Arc<T>);

#[derive(Default)]
pub struct IncludedMap<'a> {
    raw_map: HashMap<(&'a str, &'a str), &'a RawResource>,
    deserialized_map: HashMap<(&'a str, &'a str, TypeId), Arc<dyn Any + Send + Sync>>,
}

impl<'a> IncludedMap<'a> {
    pub fn get<T>(&mut self, kind: &str, id: &str) -> Result<Arc<T>, Error>
    where
        T: JsonApiDeserialize + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        if let Some(existing) = self.deserialized_map.get(&(kind, id, type_id)).cloned() {
            return Ok(existing.downcast_ref::<Handle<T>>().unwrap().0.clone());
        }

        let (kind, id, value) = {
            let raw_resource =
                self.raw_map
                    .get(&(kind, id))
                    .ok_or_else(|| Error::MissingResource {
                        kind: kind.to_string(),
                        id: id.to_string(),
                    })?;
            let kind = raw_resource.kind.as_str();
            let id = raw_resource.id.as_str();
            let value: Value = (*raw_resource).into();

            (kind, id, value)
        };

        let handle = Handle(Arc::new(T::from_value(&value, self)?));
        let resource = handle.0.clone();
        self.deserialized_map
            .insert((kind, id, type_id), Arc::new(handle));
        Ok(resource)
    }
}

impl<'a> From<&'a Vec<RawResource>> for IncludedMap<'a> {
    fn from(resources: &'a Vec<RawResource>) -> Self {
        let raw_map = resources
            .iter()
            .map(|raw| ((raw.kind.as_str(), raw.id.as_str()), raw))
            .collect();

        Self {
            raw_map,
            deserialized_map: HashMap::new(),
        }
    }
}
