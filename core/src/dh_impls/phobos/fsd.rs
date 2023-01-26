use crate::{dh, Error, ReeInt, Result};

pub(super) trait FsdMerge<T> {
    fn fsd_merge(self, id: ReeInt) -> Vec<T>;
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
            "FSD decomposition failed: highest-level entity is not a map",
        )),
    }
}

fn convert<T, U>(decomposed: Vec<FsdItem>) -> dh::Result<dh::Container<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let mut cont = dh::Container::new();
    for fsd_item in decomposed {
        match fsd_item.id.parse::<ReeInt>() {
            Ok(id) => match serde_json::from_value::<T>(fsd_item.item) {
                Ok(item) => cont.data.extend(item.fsd_merge(id)),
                Err(e) => cont
                    .warns
                    .push(format!("failed to parse FSD item with key \"{}\": {}", id, e)),
            },
            Err(_) => cont
                .warns
                .push(format!("failed to cast FSD key \"{}\" to integer", fsd_item.id)),
        }
    }
    Ok(cont)
}
