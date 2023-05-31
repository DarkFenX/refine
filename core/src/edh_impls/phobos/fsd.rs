use crate::{
    defs::ReeInt,
    edh,
    util::{IntError, IntResult},
};

pub(in crate::edh_impls::phobos) trait FsdMerge<T> {
    fn fsd_merge(self, id: ReeInt) -> Vec<T>;
}

#[derive(Debug)]
pub(in crate::edh_impls::phobos) struct FsdItem {
    pub(in crate::edh_impls::phobos) id: String,
    pub(in crate::edh_impls::phobos) item: serde_json::Value,
}
impl FsdItem {
    pub(in crate::edh_impls::phobos) fn new(id: String, item: serde_json::Value) -> Self {
        Self { id, item }
    }
}

pub(in crate::edh_impls::phobos) fn handle<T, U>(unprocessed: serde_json::Value) -> edh::Result<edh::Container<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let decomposed = decompose(unprocessed)?;
    Ok(convert::<T, U>(decomposed))
}

fn decompose(json: serde_json::Value) -> IntResult<Vec<FsdItem>> {
    match json {
        serde_json::Value::Object(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
        _ => Err(IntError::new(
            "FSD decomposition failed: highest-level entity is not a map".to_string(),
        )),
    }
}

fn convert<T, U>(decomposed: Vec<FsdItem>) -> edh::Container<U>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let mut cont = edh::Container::new();
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
    cont
}
