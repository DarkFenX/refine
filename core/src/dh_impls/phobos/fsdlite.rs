use crate::defines::ReeInt;
use crate::dh;

use super::error::{Error, Result};

pub(super) trait FsdMerge<T> {
    fn fsd_merge(self, id: ReeInt) -> T;
}

#[derive(Debug)]
pub(super) struct FsdItem {
    pub(super) id: String,
    pub(super) item: serde_json::Value,
}
impl FsdItem {
    pub(super) fn new<T: Into<String>>(id: T, item: serde_json::Value) -> FsdItem {
        FsdItem { id: id.into(), item }
    }
}

pub(super) fn handle<T, U>(unprocessed: serde_json::Value) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let decomposed = decompose(unprocessed)?;
    convert::<T, U>(decomposed)
}

fn decompose(json: serde_json::Value) -> Result<Vec<FsdItem>> {
    match json {
        serde_json::Value::Object(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
        _ => Err(Error::new(
            "FSD Lite decomposition failed: highest-level structure is not a map",
        )),
    }
}

fn convert<T, U>(decomposed: Vec<FsdItem>) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let mut data = Vec::new();
    let mut errors: u32 = 0;
    for fsd_item in decomposed {
        match (
            fsd_item.id.parse::<ReeInt>(),
            serde_json::from_value::<T>(fsd_item.item),
        ) {
            (Ok(id), Ok(item)) => data.push(item.fsd_merge(id)),
            _ => errors += 1,
        }
    }
    Ok(dh::Container::new(data, errors))
}
