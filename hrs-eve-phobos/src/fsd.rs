use crate::util::{Error, ErrorKind, Result};

pub(crate) trait FsdMerge<T> {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<T>;
}

#[derive(Debug)]
pub(crate) struct FsdItem {
    pub(crate) id: String,
    pub(crate) item: serde_json::Value,
}
impl FsdItem {
    pub(crate) fn new(id: String, item: serde_json::Value) -> Self {
        Self { id, item }
    }
}

pub(crate) fn handle<T, U>(unprocessed: serde_json::Value, suffix: &str) -> rc::ed::EResult<rc::ed::EDataCont<U>>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let decomposed = decompose(unprocessed, suffix)?;
    Ok(convert::<T, U>(decomposed))
}

fn decompose(json: serde_json::Value, suffix: &str) -> Result<Vec<FsdItem>> {
    match json {
        serde_json::Value::Object(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
        _ => Err(Error::new(ErrorKind::UnexpectedFsdTopEntity(suffix.to_string()))),
    }
}

fn convert<T, U>(decomposed: Vec<FsdItem>) -> rc::ed::EDataCont<U>
where
    T: serde::de::DeserializeOwned + FsdMerge<U>,
{
    let mut cont = rc::ed::EDataCont::new();
    for fsd_item in decomposed {
        match fsd_item.id.parse::<rc::ReeInt>() {
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
