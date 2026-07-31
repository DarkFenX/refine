pub use refine_serde_derive::VecAsMapEntry;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_with::SerializeAs;

pub trait AsMapEntry {
    type Key: Serialize;
    type Value: Serialize;

    fn get_key(&self) -> &Self::Key;
    fn get_value(&self) -> &Self::Value;
}

/// `serde_with` extension to serialize vector of structs as map. Elements have to implement
/// `AsMapEntry` either directly, or via `VecAsMapEntry` derive.
pub struct VecAsMap;

impl<T> SerializeAs<Vec<T>> for VecAsMap
where
    T: AsMapEntry,
{
    fn serialize_as<S>(source: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(source.len()))?;
        for item in source {
            map.serialize_entry(item.get_key(), item.get_value())?;
        }
        map.end()
    }
}
